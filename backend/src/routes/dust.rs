use actix_web::{post, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use influxdb2::Client;
use chrono::Utc;
use serde_json::json;
use serde::{Serialize, Deserialize};
use crate::influxdb_client::CustomInfluxClient;
use crate::sensor_client::SensorCommunityClient;

#[derive(Serialize, Deserialize)]
struct DustData {
    dust_concentration: f32,
    pm25: Option<f32>,  // PM2.5 spécifique
    pm10: Option<f32>,  // PM10 spécifique
}

#[post("/dust")]
async fn insert_dust(
    _client: web::Data<Client>,
    custom_client: web::Data<CustomInfluxClient>,
    sensor_community_client: web::Data<SensorCommunityClient>,
    mut payload: Multipart
) -> impl Responder {
    // Extraction des données du form-data
    let mut dust_concentration: Option<f32> = None;
    let mut pm25: Option<f32> = None;
    let mut pm10: Option<f32> = None;
    
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        
        if let Some(field_name) = content_disposition.get_name() {
            let name = field_name.to_string();
            
            let mut value = Vec::new();
            while let Some(chunk) = field.next().await {
                if let Ok(data) = chunk {
                    value.extend_from_slice(&data);
                }
            }
            
            if let Ok(val_str) = std::str::from_utf8(&value) {
                if let Ok(val) = val_str.parse::<f32>() {
                    match name.as_str() {
                        "dust_concentration" => dust_concentration = Some(val),
                        "pm25" => pm25 = Some(val),
                        "pm10" => pm10 = Some(val),
                        _ => {}
                    }
                }
            }
        }
    }
    
    // Vérification et insertion des données
    match dust_concentration {
        Some(dust_value) => {
            // Vérification de la plage de valeurs (0 à 1000 µg/m³ généralement)
            if dust_value < 0.0 || dust_value > 1000.0 {
                return HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "Valeur hors plage pour la concentration de poussière",
                    "expected_range": "0 à 1000 µg/m³",
                    "received": dust_value
                }));
            }
            
            // Si PM2.5 et PM10 ne sont pas fournis, utiliser dust_concentration comme estimation
            let pm25_value = pm25.unwrap_or(dust_value * 0.7); // Estimation: ~70% de la poussière totale
            let pm10_value = pm10.unwrap_or(dust_value);
            
            // Vérification des valeurs PM
            if pm25_value < 0.0 || pm25_value > 500.0 || pm10_value < 0.0 || pm10_value > 500.0 {
                return HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "Valeurs PM hors plage",
                    "expected_ranges": {
                        "pm25": "0 à 500 µg/m³",
                        "pm10": "0 à 500 µg/m³"
                    },
                    "received": {
                        "pm25": pm25_value,
                        "pm10": pm10_value
                    }
                }));
            }
            
            // Logger les données reçues
            println!("💨 Données de poussière reçues: concentration={}µg/m³, PM2.5={}µg/m³, PM10={}µg/m³", 
                    dust_value, pm25_value, pm10_value);
            
            // Variables pour tracker le succès des envois
            let mut influx_success = false;
            let mut sensor_community_success = false;
            
            // Récupérer les valeurs depuis les variables d'environnement
            let sensor_id = std::env::var("DUST_SENSOR_ID")
                .expect("DUST_SENSOR_ID non défini");
            let location = std::env::var("SENSOR_LOCATION")
                .expect("SENSOR_LOCATION non défini");
            
            // Écriture dans InfluxDB
            match custom_client.write_point(
                "dust_sensor",
                &[("sensor_id", &sensor_id), ("location", &location), ("sensor_type", "particulate_matter")],
                &[
                    ("dust_concentration", dust_value as f64),
                    ("pm25", pm25_value as f64),
                    ("pm10", pm10_value as f64)
                ]
            ).await {
                Ok(_) => {
                    println!("✅ Données poussière écrites dans InfluxDB avec succès");
                    influx_success = true;
                },
                Err(e) => {
                    println!("❌ Erreur lors de l'écriture dans InfluxDB: {}", e);
                }
            }
            
            // Envoi vers Sensor Community (utilise PM2.5 et PM10)
            match sensor_community_client.send_air_quality_data(pm25_value, pm10_value).await {
                Ok(_) => {
                    sensor_community_success = true;
                },
                Err(e) => {
                    println!("❌ Erreur lors de l'envoi vers Sensor Community: {}", e);
                }
            }
            
            // Réponse basée sur le succès des envois
            let (status, status_message) = match (influx_success, sensor_community_success) {
                (true, true) => ("success", "Données stockées dans InfluxDB et envoyées à Sensor Community"),
                (true, false) => ("partial_success", "Données stockées dans InfluxDB uniquement (erreur Sensor Community)"),
                (false, true) => ("partial_success", "Données envoyées à Sensor Community uniquement (erreur InfluxDB)"),
                (false, false) => ("error", "Erreur lors du stockage et de l'envoi des données"),
            };
            
            HttpResponse::Ok().json(json!({
                "status": status,
                "message": status_message,
                "data": {
                    "dust_concentration": {
                        "value": dust_value,
                        "unit": "µg/m³"
                    },
                    "pm25": {
                        "value": pm25_value,
                        "unit": "µg/m³",
                        "estimated": pm25.is_none()
                    },
                    "pm10": {
                        "value": pm10_value,
                        "unit": "µg/m³",
                        "estimated": pm10.is_none()
                    },
                    "sensor_type": "particulate_matter",
                    "location": location
                },
                "delivery_status": {
                    "influxdb": influx_success,
                    "sensor_community": sensor_community_success
                },
                "air_quality_index": {
                    "pm25_category": get_pm25_category(pm25_value),
                    "pm10_category": get_pm10_category(pm10_value)
                },
                "timestamp": Utc::now().to_rfc3339()
            }))
        },
        None => {
            println!("❌ Champ 'dust_concentration' manquant ou invalide");
            HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Champ 'dust_concentration' manquant ou invalide",
                "expected_format": {
                    "dust_concentration": "float (0.0 à 1000.0 µg/m³) - obligatoire",
                    "pm25": "float (0.0 à 500.0 µg/m³) - optionnel",
                    "pm10": "float (0.0 à 500.0 µg/m³) - optionnel"
                }
            }))
        }
    }
}

// Fonction helper pour déterminer la catégorie de qualité de l'air PM2.5
fn get_pm25_category(pm25: f32) -> &'static str {
    match pm25 {
        x if x <= 12.0 => "Good",
        x if x <= 35.4 => "Moderate", 
        x if x <= 55.4 => "Unhealthy for Sensitive Groups",
        x if x <= 150.4 => "Unhealthy",
        x if x <= 250.4 => "Very Unhealthy",
        _ => "Hazardous"
    }
}

// Fonction helper pour déterminer la catégorie de qualité de l'air PM10
fn get_pm10_category(pm10: f32) -> &'static str {
    match pm10 {
        x if x <= 54.0 => "Good",
        x if x <= 154.0 => "Moderate",
        x if x <= 254.0 => "Unhealthy for Sensitive Groups", 
        x if x <= 354.0 => "Unhealthy",
        x if x <= 424.0 => "Very Unhealthy",
        _ => "Hazardous"
    }
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_dust);
}
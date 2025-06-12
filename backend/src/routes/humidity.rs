use actix_web::{post, get, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use influxdb2::Client;
use chrono::Utc;
use serde_json::json;
use serde::{Serialize, Deserialize};
use crate::influxdb_client::CustomInfluxClient;
use crate::sensor_client::SensorCommunityClient;

#[derive(Serialize, Deserialize)]
struct DhtData {
    temperature: f32,
    humidity: f32,
}

#[post("/humidity")]
async fn insert_dht_data(
    _client: web::Data<Client>,
    custom_client: web::Data<CustomInfluxClient>,
    sensor_community_client: web::Data<SensorCommunityClient>,
    mut payload: Multipart
) -> impl Responder {
    // Extraction des données du form-data
    let mut temperature: Option<f32> = None;
    let mut humidity: Option<f32> = None;
    
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
                if name == "temperature" {
                    if let Ok(val) = val_str.parse::<f32>() {
                        temperature = Some(val);
                    }
                } else if name == "humidity" {
                    if let Ok(val) = val_str.parse::<f32>() {
                        humidity = Some(val);
                    }
                }
            }
        }
    }
    
    // Vérification et insertion des données
    match (temperature, humidity) {
        (Some(temp), Some(hum)) => {
            // Vérification des valeurs
            if temp < -40.0 || temp > 80.0 || hum < 0.0 || hum > 100.0 {
                return HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "Valeurs hors plage pour le DHT11",
                    "expected_ranges": {
                        "temperature": "-40°C à 80°C",
                        "humidity": "0% à 100%"
                    },
                    "received": {
                        "temperature": temp,
                        "humidity": hum
                    }
                }));
            }
            
            // Logger les données reçues
            println!("🌡️ Données DHT11 reçues: température={}°C, humidité={}%", temp, hum);
            
            // Variables pour tracker le succès des envois
            let mut influx_success = false;
            let mut sensor_community_success = false;
            
            // Écriture dans InfluxDB
            match custom_client.write_point(
                "dht11_sensor",
                &[("sensor_id", "DHT11"), ("location", "marseille"), ("sensor_type", "climate")],
                &[("temperature", temp as f64), ("humidity", hum as f64)]
            ).await {
                Ok(_) => {
                    println!("✅ Données température/humidité écrites dans InfluxDB avec succès");
                    influx_success = true;
                },
                Err(e) => {
                    println!("❌ Erreur lors de l'écriture dans InfluxDB: {}", e);
                }
            }
            
            // Envoi vers Sensor Community
            match sensor_community_client.send_climate_data(temp, hum).await {
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
                    "temperature": {
                        "value": temp,
                        "unit": "°C"
                    },
                    "humidity": {
                        "value": hum,
                        "unit": "%"
                    },
                    "sensor_type": "DHT11",
                    "location": "marseille"
                },
                "delivery_status": {
                    "influxdb": influx_success,
                    "sensor_community": sensor_community_success
                },
                "timestamp": Utc::now().to_rfc3339()
            }))
        },
        _ => {
            let missing_fields = match (temperature, humidity) {
                (None, None) => vec!["temperature", "humidity"],
                (None, Some(_)) => vec!["temperature"],
                (Some(_), None) => vec!["humidity"],
                _ => unreachable!(),
            };
            
            println!("❌ Champs manquants: {:?}", missing_fields);
            HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Champs requis manquants",
                "missing_fields": missing_fields,
                "expected_format": {
                    "temperature": "float (-40.0 à 80.0)",
                    "humidity": "float (0.0 à 100.0)"
                }
            }))
        }
    }
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_dht_data);
}
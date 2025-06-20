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
struct SoundData {
    sound_level: f32,
}

#[post("/sound")]
async fn insert_sound_data(
    _client: web::Data<Client>,
    custom_client: web::Data<CustomInfluxClient>,
    sensor_community_client: web::Data<SensorCommunityClient>,
    mut payload: Multipart
) -> impl Responder {
    // Extraction des données du form-data
    let mut sound_level: Option<f32> = None;
    
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
                if name == "sound_level" {
                    if let Ok(val) = val_str.parse::<f32>() {
                        sound_level = Some(val);
                    }
                }
            }
        }
    }
    
    // Vérification et insertion des données
    match sound_level {
        Some(level) => {
            // Vérification de la plage de valeurs
            if level < -60.0 || level > 120.0 {
                return HttpResponse::BadRequest().body(
                    "Valeur hors plage pour le niveau sonore (attendu: -60 à 120 dB)"
                );
            }
            
            // Logger les données reçues
            println!("🔊 Données de son reçues: niveau={} dB", level);
            
            // Variables pour tracker le succès des opérations
            let mut influx_success = false;
            let mut sensor_community_collection_success = false;
            
            // Récupérer les valeurs depuis les variables d'environnement
            let sensor_id = std::env::var("SOUND_SENSOR_ID")
                .expect("SOUND_SENSOR_ID non défini");
            let location = std::env::var("SENSOR_LOCATION")
                .expect("SENSOR_LOCATION non défini");
            
            // Écriture dans InfluxDB (toujours immédiate)
            match custom_client.write_point(
                "sound_sensor",
                &[("sensor_id", &sensor_id), ("location", &location), ("sensor_type", "microphone")],
                &[("sound_level", level as f64)]
            ).await {
                Ok(_) => {
                    println!("✅ Données son écrites dans InfluxDB avec succès");
                    influx_success = true;
                },
                Err(e) => {
                    println!("❌ Erreur lors de l'écriture dans InfluxDB: {}", e);
                }
            }
            
            // Collecte pour Sensor Community (pas d'envoi immédiat)
            match sensor_community_client.collect_sound_data(level).await {
                Ok(_) => {
                    println!("📊 Données sonores collectées pour envoi groupé vers Sensor Community");
                    sensor_community_collection_success = true;
                },
                Err(e) => {
                    println!("❌ Erreur lors de la collecte pour Sensor Community: {}", e);
                }
            }
            
            // Réponse basée sur le succès des opérations
            let (status, status_message) = match (influx_success, sensor_community_collection_success) {
                (true, true) => ("success", "Données stockées dans InfluxDB et collectées pour Sensor Community"),
                (true, false) => ("partial_success", "Données stockées dans InfluxDB uniquement (erreur collecte Sensor Community)"),
                (false, true) => ("partial_success", "Données collectées pour Sensor Community uniquement (erreur InfluxDB)"),
                (false, false) => ("error", "Erreur lors du stockage et de la collecte des données"),
            };
            
            HttpResponse::Ok().json(json!({
                "status": status,
                "message": status_message,
                "data": {
                    "sound_level": level,
                    "unit": "dB",
                    "sensor_type": sensor_id,
                    "location": location
                },
                "operations_status": {
                    "influxdb_storage": influx_success,
                    "sensor_community_collection": sensor_community_collection_success
                },
                "sensor_community_info": {
                    "data_collected": sensor_community_collection_success,
                    "send_endpoint": "/sensor-community/send",
                    "status_endpoint": "/sensor-community/status"
                },
                "timestamp": Utc::now().to_rfc3339()
            }))
        },
        None => {
            println!("❌ Champ 'sound_level' manquant ou invalide");
            HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Champ 'sound_level' manquant ou invalide",
                "expected": "sound_level: float (-60.0 à 120.0 dB)"
            }))
        }
    }
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_sound_data);
}
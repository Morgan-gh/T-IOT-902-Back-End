use actix_web::{post, get, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use influxdb2::Client;
use chrono::Utc;
use serde_json::json;
use serde::{Serialize, Deserialize};
use crate::influxdb_client::CustomInfluxClient;

#[derive(Serialize, Deserialize)]
struct DhtData {
    temperature: f32,
    humidity: f32,
}

#[get("/humidity")]
async fn get_dht_data(_client: web::Data<Client>) -> impl Responder {
    // Pour le moment, juste retourner une réponse simple
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Cette API retournera bientôt les données de température et humidité depuis InfluxDB",
        "data": []
    }))
}

#[post("/humidity")]
async fn insert_dht_data(
    _client: web::Data<Client>,
    custom_client: web::Data<CustomInfluxClient>,
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
                return HttpResponse::BadRequest().body(
                    "Valeurs hors plage pour le DHT11 (température: -40°C à 80°C, humidité: 0% à 100%)"
                );
            }
            
            // Logger les données reçues
            println!("Données DHT11 reçues: température={}, humidité={}", temp, hum);
            
            // Utiliser le client personnalisé pour écrire les données
            match custom_client.write_point(
                "dht11_sensor",
                &[("sensor_id", "DHT11")],
                &[("temperature", temp as f64), ("humidity", hum as f64)]
            ).await {
                Ok(_) => {
                    println!("Données température/humidité écrites dans InfluxDB avec succès");
                    HttpResponse::Ok().json(json!({
                        "status": "success",
                        "message": "Données DHT11 reçues et stockées dans InfluxDB",
                        "temperature": temp,
                        "humidity": hum,
                        "timestamp": Utc::now().to_rfc3339()
                    }))
                },
                Err(e) => {
                    println!("Erreur lors de l'écriture dans InfluxDB: {}", e);
                    HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": format!("Erreur lors du stockage des données: {}", e)
                    }))
                }
            }
        },
        _ => HttpResponse::BadRequest().body("Champs 'temperature' et 'humidity' requis")
    }
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_dht_data);
    cfg.service(get_dht_data);
}
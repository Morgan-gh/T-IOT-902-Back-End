use actix_web::{post, get, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use influxdb2::Client;
use chrono::Utc;
use serde_json::json;
use serde::{Serialize, Deserialize};
use crate::influxdb_client::CustomInfluxClient;

#[derive(Serialize, Deserialize)]
struct DustData {
    dust_concentration: f32,
}

#[get("/dust")]
async fn get_dust_data(_client: web::Data<Client>) -> impl Responder {
    // Pour le moment, juste retourner une réponse simple
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Cette API retournera bientôt les données de poussière depuis InfluxDB",
        "data": []
    }))
}

#[post("/dust")]
async fn insert_dust(
    _client: web::Data<Client>,
    custom_client: web::Data<CustomInfluxClient>,
    mut payload: Multipart
) -> impl Responder {
    // Extraction des données du form-data
    let mut dust_concentration: Option<f32> = None;
    
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        
        if let Some(name) = content_disposition.get_name() {
            if name == "dust_concentration" {
                let mut value = Vec::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(data) = chunk {
                        value.extend_from_slice(&data);
                    }
                }
                
                if let Ok(val_str) = std::str::from_utf8(&value) {
                    if let Ok(val) = val_str.parse::<f32>() {
                        dust_concentration = Some(val);
                    }
                }
            }
        }
    }
    
    // Vérification et insertion des données
    match dust_concentration {
        Some(dust_value) => {
            // Logger les données reçues
            println!("Données de poussière reçues: concentration={}", dust_value);
            
            // Utiliser le client personnalisé pour écrire les données
            match custom_client.write_point(
                "dust_sensor",
                &[("sensor_id", "dust_sensor")],
                &[("dust_concentration", dust_value as f64)]
            ).await {
                Ok(_) => {
                    println!("Données poussière écrites dans InfluxDB avec succès");
                    HttpResponse::Ok().json(json!({
                        "status": "success",
                        "message": "Données de poussière reçues et stockées dans InfluxDB",
                        "value": dust_value,
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
        None => HttpResponse::BadRequest().body("Champ 'dust_concentration' manquant ou invalide")
    }
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_dust);
    cfg.service(get_dust_data);
}
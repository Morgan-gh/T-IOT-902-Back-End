use actix_web::{post, get, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use influxdb2::Client;
use chrono::Utc;
use serde_json::json;
use serde::{Serialize, Deserialize};
use crate::influxdb_client::CustomInfluxClient;

#[derive(Serialize, Deserialize)]
struct SoundData {
    sound_level: f32,
}

#[get("/sound")]
async fn get_sound_data(_client: web::Data<Client>) -> impl Responder {
    // Pour le moment, juste retourner une réponse simple
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Cette API retournera bientôt les données de son depuis InfluxDB",
        "data": []
    }))
}

#[post("/sound")]
async fn insert_sound_data(
    _client: web::Data<Client>,
    custom_client: web::Data<CustomInfluxClient>,
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
            println!("Données de son reçues: niveau={}", level);
            
            // Utiliser le client personnalisé pour écrire les données
            match custom_client.write_point(
                "sound_sensor",
                &[("sensor_id", "SPH0645")],
                &[("sound_level", level as f64)]
            ).await {
                Ok(_) => {
                    println!("Données son écrites dans InfluxDB avec succès");
                    HttpResponse::Ok().json(json!({
                        "status": "success",
                        "message": "Données sonores reçues et stockées dans InfluxDB",
                        "sound_level": level,
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
        None => HttpResponse::BadRequest().body("Champ 'sound_level' manquant ou invalide")
    }
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_sound_data);
    cfg.service(get_sound_data);
}
use actix_web::{post, get, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use mysql_async::prelude::*;
use mysql_async::Pool;
use chrono::Utc;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SoundData {
    sound_level: f32,
}

#[derive(Serialize, Deserialize)]
struct SoundRecord {
    id: i32,
    sound_level: f32,
    timestamp: String,
}

// Fonction pour initialiser la table si elle n'existe pas
async fn ensure_table_exists(pool: &Pool) -> Result<(), mysql_async::Error> {
    let create_table_query = r"
    USE iot_sensors;
    CREATE TABLE IF NOT EXISTS sound_sensor (
        id INT AUTO_INCREMENT PRIMARY KEY,
        sound_level FLOAT NOT NULL,
        timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )";
    
    let mut conn = pool.get_conn().await?;
    conn.query_drop(create_table_query).await?;
    Ok(())
}

#[get("/sound")]
async fn get_sound_data(pool: web::Data<Pool>) -> impl Responder {
    // Vérifier si la table existe, sinon la créer
    if let Err(e) = ensure_table_exists(&pool).await {
        return HttpResponse::InternalServerError()
            .body(format!("Erreur lors de la vérification de la table: {}", e));
    }

    let query = "SELECT id, sound_level, timestamp FROM sound_sensor ORDER BY timestamp DESC";
    
    match pool.get_conn().await {
        Ok(mut conn) => {
            match conn.query_map(query, |(id, sound_level, timestamp)| {
                SoundRecord {
                    id,
                    sound_level,
                    timestamp,
                }
            }).await {
                Ok(sound_records) => {
                    HttpResponse::Ok().json(
                        json!({
                            "status": "success",
                            "count": sound_records.len(),
                            "data": sound_records
                        })
                    )
                },
                Err(e) => HttpResponse::InternalServerError()
                    .body(format!("Erreur lors de la récupération des données: {}", e))
            }
        },
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Erreur de connexion à la base de données: {}", e))
    }
}

#[post("/sound")]
async fn insert_sound_data(pool: web::Data<Pool>, mut payload: Multipart) -> impl Responder {
    // Vérifier si la table existe, sinon la créer
    if let Err(e) = ensure_table_exists(&pool).await {
        return HttpResponse::InternalServerError()
            .body(format!("Erreur lors de la création de la table: {}", e));
    }

    // Extraction des données du form-data
    let mut sound_level: Option<f32> = None;
    
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        
        if let Some(field_name) = content_disposition.get_name() {
            let name = field_name.to_string(); // Créer une copie du nom
            
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
            // Le SPH0645 est un capteur MEMS qui mesure généralement en dB
            // Plage de valeurs valides (typiquement -60 à +30 dB pour les microphones MEMS)
            if level < -60.0 || level > 120.0 {
                return HttpResponse::BadRequest().body(
                    "Valeur hors plage pour le SPH0645 (niveau sonore attendu: -60 à 120 dB)"
                );
            }
            
            // Préparation de la requête avec paramètres pour éviter les injections SQL
            let query = "INSERT INTO sound_sensor (sound_level) VALUES (?)";
            
            // Exécution de la requête avec paramètres
            match pool.get_conn().await {
                Ok(mut conn) => {
                    match conn.exec_drop(query, (level,)).await {
                        Ok(_) => HttpResponse::Ok().json(
                            json!({
                                "status": "success",
                                "message": "Données sonores insérées",
                                "sound_level": level,
                                "timestamp": Utc::now().to_rfc3339()
                            })
                        ),
                        Err(e) => HttpResponse::InternalServerError()
                            .body(format!("Erreur lors de l'insertion des données: {}", e))
                    }
                },
                Err(e) => HttpResponse::InternalServerError()
                    .body(format!("Erreur de connexion à la base de données: {}", e))
            }
        },
        None => HttpResponse::BadRequest().body("Champ 'sound_level' manquant ou invalide")
    }
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_sound_data);
    cfg.service(get_sound_data);
}
use actix_web::{post, get, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use mysql_async::prelude::*;
use mysql_async::{Pool, Row, from_row};
use chrono::Utc;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DustData {
    dust_concentration: f32,
}

#[derive(Serialize, Deserialize)]
struct DustRecord {
    id: i32,
    dust_concentration: f32,
    timestamp: String,
}

// Fonction pour initialiser la table si elle n'existe pas
async fn ensure_table_exists(pool: &Pool) -> Result<(), mysql_async::Error> {
    let create_table_query = r"
    USE iot_sensors;
    CREATE TABLE IF NOT EXISTS dust_sensor (
        id INT AUTO_INCREMENT PRIMARY KEY,
        dust_concentration FLOAT NOT NULL,
        timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )";
    
    let mut conn = pool.get_conn().await?;
    conn.query_drop(create_table_query).await?;
    Ok(())
}

#[get("/dust")]
async fn get_dust_data(pool: web::Data<Pool>) -> impl Responder {
    // Vérifier si la table existe, sinon la créer
    if let Err(e) = ensure_table_exists(&pool).await {
        return HttpResponse::InternalServerError()
            .body(format!("Erreur lors de la vérification de la table: {}", e));
    }

    let query = "SELECT id, dust_concentration, timestamp FROM dust_sensor ORDER BY timestamp DESC";
    
    match pool.get_conn().await {
        Ok(mut conn) => {
            match conn.query_map(query, |(id, dust_concentration, timestamp)| {
                DustRecord {
                    id,
                    dust_concentration,
                    timestamp,
                }
            }).await {
                Ok(dust_records) => {
                    HttpResponse::Ok().json(
                        json!({
                            "status": "success",
                            "count": dust_records.len(),
                            "data": dust_records
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

#[post("/dust")]
async fn insert_dust(pool: web::Data<Pool>, mut payload: Multipart) -> impl Responder {
    // Vérifier si la table existe, sinon la créer
    if let Err(e) = ensure_table_exists(&pool).await {
        return HttpResponse::InternalServerError()
            .body(format!("Erreur lors de la création de la table: {}", e));
    }

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
            // Préparation de la requête avec paramètres pour éviter les injections SQL
            let query = "INSERT INTO dust_sensor (dust_concentration) VALUES (?)";
            
            // Exécution de la requête avec paramètres
            match pool.get_conn().await {
                Ok(mut conn) => {
                    match conn.exec_drop(query, (dust_value,)).await {
                        Ok(_) => HttpResponse::Ok().json(
                            json!({
                                "status": "success",
                                "message": "Données de poussière insérées",
                                "value": dust_value,
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
        None => HttpResponse::BadRequest().body("Champ 'dust_concentration' manquant ou invalide")
    }
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_dust);
    cfg.service(get_dust_data);
}
use actix_web::{post, get, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use mysql_async::prelude::*;
use mysql_async::Pool;
use chrono::Utc;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DhtData {
    temperature: f32,
    humidity: f32,
}

#[derive(Serialize, Deserialize)]
struct DhtRecord {
    id: i32,
    temperature: f32,
    humidity: f32,
    timestamp: String,
}

// Fonction pour initialiser la table si elle n'existe pas
async fn ensure_table_exists(pool: &Pool) -> Result<(), mysql_async::Error> {
    let create_table_query = r"
    USE iot_sensors;
    CREATE TABLE IF NOT EXISTS dht11_sensor (
        id INT AUTO_INCREMENT PRIMARY KEY,
        temperature FLOAT NOT NULL,
        humidity FLOAT NOT NULL,
        timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )";
    
    let mut conn = pool.get_conn().await?;
    conn.query_drop(create_table_query).await?;
    Ok(())
}

#[get("/humidity")]
async fn get_dht_data(pool: web::Data<Pool>) -> impl Responder {
    // Vérifier si la table existe, sinon la créer
    if let Err(e) = ensure_table_exists(&pool).await {
        return HttpResponse::InternalServerError()
            .body(format!("Erreur lors de la vérification de la table: {}", e));
    }

    let query = "SELECT id, temperature, humidity, timestamp FROM dht11_sensor ORDER BY timestamp DESC";
    
    match pool.get_conn().await {
        Ok(mut conn) => {
            match conn.query_map(query, |(id, temperature, humidity, timestamp)| {
                DhtRecord {
                    id,
                    temperature,
                    humidity,
                    timestamp,
                }
            }).await {
                Ok(dht_records) => {
                    HttpResponse::Ok().json(
                        json!({
                            "status": "success",
                            "count": dht_records.len(),
                            "data": dht_records
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

#[post("/humidity")]
async fn insert_dht_data(pool: web::Data<Pool>, mut payload: Multipart) -> impl Responder {
    // Vérifier si la table existe, sinon la créer
    if let Err(e) = ensure_table_exists(&pool).await {
        return HttpResponse::InternalServerError()
            .body(format!("Erreur lors de la création de la table: {}", e));
    }

    // Extraction des données du form-data
    let mut temperature: Option<f32> = None;
    let mut humidity: Option<f32> = None;
    
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
            // Vérification des valeurs (DHT11 a des plages spécifiques)
            if temp < -40.0 || temp > 80.0 || hum < 0.0 || hum > 100.0 {
                return HttpResponse::BadRequest().body(
                    "Valeurs hors plage pour le DHT11 (température: -40°C à 80°C, humidité: 0% à 100%)"
                );
            }
            
            // Préparation de la requête avec paramètres pour éviter les injections SQL
            let query = "INSERT INTO dht11_sensor (temperature, humidity) VALUES (?, ?)";
            
            // Exécution de la requête avec paramètres
            match pool.get_conn().await {
                Ok(mut conn) => {
                    match conn.exec_drop(query, (temp, hum)).await {
                        Ok(_) => HttpResponse::Ok().json(
                            json!({
                                "status": "success",
                                "message": "Données DHT11 insérées",
                                "temperature": temp,
                                "humidity": hum,
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
        _ => HttpResponse::BadRequest().body("Champs 'temperature' et 'humidity' requis")
    }
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_dht_data);
    cfg.service(get_dht_data);
}
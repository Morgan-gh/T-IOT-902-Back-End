use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;
use crate::sensor_client::SensorCommunityClient;

#[post("/sensor-community/send")]
async fn send_all_to_sensor_community(
    sensor_community_client: web::Data<SensorCommunityClient>,
) -> impl Responder {
    println!("🚀 Déclenchement de l'envoi des dernières valeurs vers Sensor Community...");
    
    // Vérifier s'il y a des données collectées
    let has_data = sensor_community_client.get_collected_data_count() > 0;
    
    if !has_data {
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Aucune donnée collectée à envoyer",
            "has_data": false,
            "timestamp": Utc::now().to_rfc3339()
        }));
    }
    
    println!("📊 Données disponibles, envoi des dernières valeurs...");
    
    // Envoyer toutes les données collectées
    match sensor_community_client.send_all_collected_data().await {
        Ok(_) => {
            println!("✅ Envoi des dernières valeurs vers Sensor Community réussi");
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Dernières valeurs envoyées à Sensor Community",
                "data_sent": true,
                "timestamp": Utc::now().to_rfc3339()
            }))
        },
        Err(e) => {
            println!("❌ Erreur lors de l'envoi vers Sensor Community: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Erreur lors de l'envoi vers Sensor Community: {}", e),
                "data_sent": false,
                "timestamp": Utc::now().to_rfc3339()
            }))
        }
    }
}

#[post("/sensor-community/clear")]
async fn clear_collected_data(
    sensor_community_client: web::Data<SensorCommunityClient>,
) -> impl Responder {
    println!("🗑️  Vidage manuel des dernières valeurs collectées...");
    
    let had_data = sensor_community_client.get_collected_data_count() > 0;
    sensor_community_client.clear_collected_data();
    
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Dernières valeurs vidées avec succès",
        "had_data": had_data,
        "timestamp": Utc::now().to_rfc3339()
    }))
}

#[post("/sensor-community/status")]
async fn get_collection_status(
    sensor_community_client: web::Data<SensorCommunityClient>,
) -> impl Responder {
    let has_data = sensor_community_client.get_collected_data_count() > 0;
    
    HttpResponse::Ok().json(json!({
        "status": "success",
        "has_data": has_data,
        "data_types_available": has_data,
        "timestamp": Utc::now().to_rfc3339()
    }))
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(send_all_to_sensor_community)
        .service(clear_collected_data)
        .service(get_collection_status);
} 
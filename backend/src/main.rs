mod routes;
mod influxdb_client;
mod sensor_client;

use actix_web::{web, App, HttpServer};
use influxdb2::Client;
use crate::influxdb_client::CustomInfluxClient;
use crate::sensor_client::SensorCommunityClient;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Charger le fichier .env
    dotenv().ok();
    
    // Configuration InfluxDB
    let influxdb_url = std::env::var("INFLUXDB_URL")
        .unwrap_or_else(|_| std::env::var("INFLUXDB_URL_DEFAULT").expect("INFLUXDB_URL_DEFAULT non défini"));

    let influxdb_token = std::env::var("INFLUXDB_TOKEN")
        .expect("❌ INFLUXDB_TOKEN doit être défini dans le fichier .env");
    
    let influxdb_org = std::env::var("INFLUXDB_ORG")
        .unwrap_or_else(|_| std::env::var("INFLUXDB_ORG_DEFAULT").expect("INFLUXDB_ORG_DEFAULT non défini"));
        
    let influxdb_bucket = std::env::var("INFLUXDB_BUCKET")
        .unwrap_or_else(|_| std::env::var("INFLUXDB_BUCKET_DEFAULT").expect("INFLUXDB_BUCKET_DEFAULT non défini"));

    // Configuration Sensor Community
    let sensor_id = std::env::var("SENSOR_COMMUNITY_ID")
        .expect("❌ SENSOR_COMMUNITY_ID doit être défini dans le fichier .env");
    
    let sensor_pin = std::env::var("SENSOR_COMMUNITY_PIN")
        .expect("❌ SENSOR_COMMUNITY_PIN doit être défini dans le fichier .env");

    // Créer le client Sensor Community
    let sensor_community_client = SensorCommunityClient::new_single(sensor_id.clone(), sensor_pin.clone());

    // Configuration serveur
    let server_host = std::env::var("SERVER_HOST")
        .unwrap_or_else(|_| std::env::var("SERVER_HOST_DEFAULT").expect("SERVER_HOST_DEFAULT non défini"));
    
    let server_port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| std::env::var("SERVER_PORT_DEFAULT").expect("SERVER_PORT_DEFAULT non défini"));

    // Créer le client InfluxDB officiel
    let client = Client::new(influxdb_url.clone(), influxdb_token.clone(), influxdb_org.clone());
    
    // Créer notre client InfluxDB personnalisé
    let custom_client = CustomInfluxClient::new(
        influxdb_url.clone(),
        influxdb_token.clone(),
        influxdb_org.clone(),
        influxdb_bucket.clone()
    );
    
    // Créer le client Sensor Community
    let sensor_community_client_data = web::Data::new(sensor_community_client);
    
    // Configurer le niveau de log
    let rust_log = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| std::env::var("RUST_LOG_DEFAULT").expect("RUST_LOG_DEFAULT non défini"));
    
    std::env::set_var("RUST_LOG", &rust_log);
    env_logger::init();

    // Logs de démarrage
    log::info!("🚀 Starting IoT Backend Server");
    log::info!("🌐 Server: {}:{}", server_host, server_port);
    log::info!("📊 InfluxDB URL: {}", influxdb_url);
    log::info!("📊 InfluxDB Org: {}", influxdb_org);
    log::info!("🔑 InfluxDB Token: {}...", influxdb_token.chars().take(8).collect::<String>());
    log::info!("📊 InfluxDB Bucket: {}", influxdb_bucket);
    log::info!("📡 LoRa Device - Sensor Community ID: {}", sensor_id);
    log::info!("🔐 Sensor Community PIN: {}...", sensor_pin.chars().take(3).collect::<String>());
    
    // Logs de la nouvelle architecture
    log::info!("🔄 Nouvelle architecture activée:");
    log::info!("   📊 InfluxDB: Envoi immédiat pour chaque capteur");
    log::info!("   📡 Sensor Community: Collecte groupée, envoi via /sensor-community/send");
    log::info!("   🔗 Endpoints disponibles:");
    log::info!("      - POST /sound - Collecte données sonores");
    log::info!("      - POST /humidity - Collecte données climatiques");
    log::info!("      - POST /dust - Collecte données qualité air");
    log::info!("      - POST /sensor-community/send - Envoi groupé vers Sensor Community");
    log::info!("      - POST /sensor-community/status - Statut des données collectées");
    log::info!("      - POST /sensor-community/clear - Vidage des données collectées");

    // Construire l'adresse de bind
    let bind_address = format!("{}:{}", server_host, server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(custom_client.clone()))
            .app_data(sensor_community_client_data.clone())
            .configure(routes::config)
    })
    .bind(&bind_address)?
    .run()
    .await
}
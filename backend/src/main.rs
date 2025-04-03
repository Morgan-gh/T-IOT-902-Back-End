mod routes;
mod influxdb_client;
mod data_generator; 

use actix_web::{web, App, HttpServer};
use influxdb2::Client;
use crate::influxdb_client::CustomInfluxClient;
use crate::data_generator::start_data_generator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configuration InfluxDB
    let influxdb_url = std::env::var("INFLUXDB_URL")
        .unwrap_or_else(|_| "http://influxdb:8086".to_string());

    let influxdb_token = std::env::var("INFLUXDB_TOKEN")
        .unwrap_or_else(|_| "my-super-secret-token".to_string());
    
    let influxdb_org = std::env::var("INFLUXDB_ORG")
        .unwrap_or_else(|_| "iot-org".to_string());
        
    let influxdb_bucket = std::env::var("INFLUXDB_BUCKET")
        .unwrap_or_else(|_| "iot-data".to_string());

    // Créer le client InfluxDB officiel (à garder pour compatibilité)
    let client = Client::new(influxdb_url.clone(), influxdb_token.clone(), influxdb_org.clone());
    
    // Créer notre client personnalisé
    let custom_client = CustomInfluxClient::new(
        influxdb_url.clone(),
        influxdb_token.clone(),
        influxdb_org.clone(),
        influxdb_bucket.clone()
    );

    // FAKE DATA GENERATOR
    let data_gen_client = custom_client.clone();
    start_data_generator(data_gen_client).await;
    
    // Configurer le niveau de log
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    log::info!("Starting HTTP server on 0.0.0.0:8080");
    log::info!("InfluxDB URL: {}", influxdb_url);
    log::info!("InfluxDB Org: {}", influxdb_org);
    log::info!("InfluxDB Bucket: {}", influxdb_bucket);
    log::info!("InfluxDB Token: {}", influxdb_token.chars().take(5).collect::<String>() + "...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(custom_client.clone()))
            .configure(routes::config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
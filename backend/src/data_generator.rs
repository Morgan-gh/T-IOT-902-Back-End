use crate::influxdb_client::CustomInfluxClient;
use std::time::Duration;
use rand::{rngs::StdRng, Rng, SeedableRng};
use tokio::time;
use log::info;

pub async fn start_data_generator(custom_client: CustomInfluxClient) {
    // Utiliser StdRng qui implémente Send au lieu de ThreadRng
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        let mut rng = StdRng::seed_from_u64(seed);
        
        loop {
            interval.tick().await;
            
            // Générer des données aléatoires
            let temperature = 20.0 + rng.gen_range(-5.0..10.0);
            let humidity = 50.0 + rng.gen_range(-10.0..30.0);
            let sound_level = 40.0 + rng.gen_range(0.0..40.0);
            let dust_concentration = 20.0 + rng.gen_range(0.0..30.0);
            
            // Envoyer les données de température et humidité
            match custom_client.write_point(
                "dht11_sensor",
                &[("sensor_id", "DHT11")],
                &[("temperature", temperature), ("humidity", humidity)]
            ).await {
                Ok(_) => info!("Données factices DHT11 envoyées: temp={}, hum={}", temperature, humidity),
                Err(e) => eprintln!("Erreur lors de l'envoi des données DHT11: {}", e),
            }
            
            // Envoyer les données de son
            match custom_client.write_point(
                "sound_sensor",
                &[("sensor_id", "SPH0645")],
                &[("sound_level", sound_level)]
            ).await {
                Ok(_) => info!("Données factices son envoyées: niveau={}", sound_level),
                Err(e) => eprintln!("Erreur lors de l'envoi des données son: {}", e),
            }
            
            // Envoyer les données de poussière
            match custom_client.write_point(
                "dust_sensor",
                &[("sensor_id", "dust_sensor")],
                &[("dust_concentration", dust_concentration)]
            ).await {
                Ok(_) => info!("Données factices poussière envoyées: concentration={}", dust_concentration),
                Err(e) => eprintln!("Erreur lors de l'envoi des données poussière: {}", e),
            }
        }
    });
}
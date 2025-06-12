use reqwest::Client as HttpClient;
use serde::{Serialize, Deserialize};
use std::error::Error;

// Structure pour l'envoi vers Sensor Community
#[derive(Serialize)]
struct SensorCommunityData {
    software_version: String,
    sensordatavalues: Vec<SensorValue>,
}

#[derive(Serialize)]
struct SensorValue {
    value_type: String,
    value: String,
}

// Client pour Sensor Community
#[derive(Clone)]
pub struct SensorCommunityClient {
    http_client: HttpClient,
    sensor_id: String,
    sensor_pin: String,
    base_url: String,
}

impl SensorCommunityClient {
    pub fn new_single(sensor_id: String, sensor_pin: String) -> Self {
        Self {
            http_client: HttpClient::new(),
            sensor_id,
            sensor_pin,
            base_url: "https://api.sensor.community/v1/push-sensor-data/".to_string(),
        }
    }
    
    /// Envoie des données de niveau sonore vers Sensor Community
    pub async fn send_sound_data(&self, sound_level: f32) -> Result<(), Box<dyn Error>> {
        let data = SensorCommunityData {
            software_version: "rust-iot-backend-1.0".to_string(),
            sensordatavalues: vec![
                SensorValue {
                    value_type: "noise_LAeq".to_string(),
                    value: sound_level.to_string(),
                },
                SensorValue {
                    value_type: "noise_LA_min".to_string(),
                    value: sound_level.to_string(),
                },
                SensorValue {
                    value_type: "noise_LA_max".to_string(),
                    value: sound_level.to_string(),
                }
            ],
        };
        
        self.send_data(data).await
    }
    
    /// Envoie des données de température et humidité vers Sensor Community
    pub async fn send_climate_data(&self, temperature: f32, humidity: f32) -> Result<(), Box<dyn Error>> {
        let data = SensorCommunityData {
            software_version: "rust-iot-backend-1.0".to_string(),
            sensordatavalues: vec![
                SensorValue {
                    value_type: "temperature".to_string(),
                    value: temperature.to_string(),
                },
                SensorValue {
                    value_type: "humidity".to_string(),
                    value: humidity.to_string(),
                }
            ],
        };
        
        self.send_data(data).await
    }
    
    /// Envoie des données de qualité de l'air (PM2.5, PM10) vers Sensor Community
    pub async fn send_air_quality_data(&self, pm25: f32, pm10: f32) -> Result<(), Box<dyn Error>> {
        let data = SensorCommunityData {
            software_version: "rust-iot-backend-1.0".to_string(),
            sensordatavalues: vec![
                SensorValue {
                    value_type: "P2".to_string(), // PM2.5
                    value: pm25.to_string(),
                },
                SensorValue {
                    value_type: "P1".to_string(), // PM10
                    value: pm10.to_string(),
                }
            ],
        };
        
        self.send_data(data).await
    }
    
    /// Méthode générique pour envoyer des données à Sensor Community
    async fn send_data(&self, data: SensorCommunityData) -> Result<(), Box<dyn Error>> {
        println!("Envoi vers Sensor Community - Sensor ID: {}", self.sensor_id);
        
        let response = self.http_client
            .post(&self.base_url)
            .header("X-PIN", &self.sensor_pin)
            .header("X-Sensor", &self.sensor_id)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await?;
        
        if response.status().is_success() {
            println!("✅ Données envoyées à Sensor Community avec succès");
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await?;
            let error_msg = format!("❌ Erreur Sensor Community: status={}, body={}", status, body);
            println!("{}", error_msg);
            Err(error_msg.into())
        }
    }
    
    /// Méthode pour tester la connexion avec Sensor Community
    pub async fn test_connection(&self) -> Result<(), Box<dyn Error>> {
        println!("Test de connexion à Sensor Community...");
        
        // Envoie des données de test
        let test_data = SensorCommunityData {
            software_version: "rust-iot-backend-test".to_string(),
            sensordatavalues: vec![
                SensorValue {
                    value_type: "temperature".to_string(),
                    value: "20.0".to_string(),
                }
            ],
        };
        
        match self.send_data(test_data).await {
            Ok(_) => {
                println!("✅ Test de connexion réussi");
                Ok(())
            },
            Err(e) => {
                println!("❌ Test de connexion échoué: {}", e);
                Err(e)
            }
        }
    }
}

// Structure pour les erreurs personnalisées (optionnel)
#[derive(Debug)]
pub struct SensorCommunityError {
    pub message: String,
    pub status_code: Option<u16>,
}

impl std::fmt::Display for SensorCommunityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Sensor Community Error: {}", self.message)
    }
}

impl Error for SensorCommunityError {}
use reqwest::Client as HttpClient;
use serde::Serialize;
use std::error::Error;
use std::sync::Mutex;
use std::sync::Arc;

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

// Structure pour collecter les dernières données de tous les capteurs
#[derive(Debug, Clone)]
pub struct SensorData {
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub sound_level: Option<f32>,
    pub dust_concentration: Option<f32>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
impl SensorData {
    pub fn new() -> Self {
        Self {
            temperature: None,
            humidity: None,
            sound_level: None,
            dust_concentration: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn with_humidity(mut self, hum: f32) -> Self {
        self.humidity = Some(hum);
        self
    }

    pub fn with_sound_level(mut self, sound: f32) -> Self {
        self.sound_level = Some(sound);
        self
    }

    pub fn with_dust_concentration(mut self, dust: f32) -> Self {
        self.dust_concentration = Some(dust);
        self
    }

    pub fn has_data(&self) -> bool {
        self.temperature.is_some() || 
        self.humidity.is_some() || 
        self.sound_level.is_some() || 
        self.dust_concentration.is_some()
    }
}

// Client pour Sensor Community avec collecte des dernières valeurs
#[derive(Clone)]
pub struct SensorCommunityClient {
    http_client: HttpClient,
    sensor_id: String,
    sensor_pin: String,
    base_url: String,
    latest_data: Arc<Mutex<SensorData>>,
}

impl SensorCommunityClient {
    pub fn new_single(sensor_id: String, sensor_pin: String) -> Self {
        let base_url = std::env::var("SENSOR_COMMUNITY_URL")
            .expect("❌ SENSOR_COMMUNITY_URL doit être défini dans le fichier .env");
        
        println!("🔧 Initialisation Sensor Community Client:");
        println!("   Sensor ID: {}", sensor_id);
        println!("   Sensor PIN: {}", sensor_pin);
        println!("   Base URL: {}", base_url);
        
        Self {
            http_client: HttpClient::new(),
            sensor_id,
            sensor_pin,
            base_url,
            latest_data: Arc::new(Mutex::new(SensorData::new())),
        }
    }

    /// Collecte les données de température et humidité (ne les envoie pas immédiatement)
    pub async fn collect_climate_data(&self, temperature: f32, humidity: f32) -> Result<(), Box<dyn Error>> {
        let mut data = self.latest_data.lock().unwrap();
        data.temperature = Some(temperature);
        data.humidity = Some(humidity);
        data.timestamp = chrono::Utc::now();
        
        println!("📊 Données climatiques collectées: temp={}°C, hum={}%", temperature, humidity);
        Ok(())
    }

    /// Collecte les données de niveau sonore (ne les envoie pas immédiatement)
    pub async fn collect_sound_data(&self, sound_level: f32) -> Result<(), Box<dyn Error>> {
        let mut data = self.latest_data.lock().unwrap();
        data.sound_level = Some(sound_level);
        data.timestamp = chrono::Utc::now();
        
        println!("📊 Données sonores collectées: niveau={} dB", sound_level);
        Ok(())
    }

    /// Collecte les données de qualité de l'air (ne les envoie pas immédiatement)
    pub async fn collect_air_quality_data(&self, pm25: f32, pm10: f32) -> Result<(), Box<dyn Error>> {
        let mut data = self.latest_data.lock().unwrap();
        data.dust_concentration = Some(pm25);
        data.timestamp = chrono::Utc::now();
        
        println!("📊 Données qualité air collectées: PM2.5={}µg/m³, PM10={}µg/m³", pm25, pm10);
        Ok(())
    }

    /// Collecte les données de concentration de poussière (ne les envoie pas immédiatement)
    pub async fn collect_dust_data(&self, dust_concentration: f32) -> Result<(), Box<dyn Error>> {
        let mut data = self.latest_data.lock().unwrap();
        data.dust_concentration = Some(dust_concentration);
        data.timestamp = chrono::Utc::now();
        
        println!("📊 Données poussière collectées: concentration={}µg/m³", dust_concentration);
        Ok(())
    }

    /// Envoie toutes les données collectées vers Sensor Community en une seule requête
    pub async fn send_all_collected_data(&self) -> Result<(), Box<dyn Error>> {
        let data = self.latest_data.lock().unwrap();
        
        if !data.has_data() {
            println!("⚠️  Aucune donnée à envoyer vers Sensor Community");
            return Ok(());
        }
        
        println!("🚀 Envoi groupé vers Sensor Community: {:?}", data);
        
        // Créer le payload pour Sensor Community
        let mut sensordatavalues = Vec::new();
        
        if let Some(temp) = data.temperature {
            sensordatavalues.push(SensorValue {
                value_type: "temperature".to_string(),
                value: temp.to_string(),
            });
        }
        
        if let Some(hum) = data.humidity {
            sensordatavalues.push(SensorValue {
                value_type: "humidity".to_string(),
                value: hum.to_string(),
            });
        }
        
        if let Some(sound) = data.sound_level {
            sensordatavalues.push(SensorValue {
                value_type: "noise_LAeq".to_string(),
                value: sound.to_string(),
            });
        }
        
        if let Some(dust) = data.dust_concentration {
            sensordatavalues.push(SensorValue {
                value_type: "P2".to_string(), // PM2.5
                value: dust.to_string(),
            });
        }
        
        let sensor_community_data = SensorCommunityData {
            software_version: "rust-iot-backend-1.0".to_string(),
            sensordatavalues,
        };
        
        // Envoyer les données
        let result = self.send_data(sensor_community_data).await;
        
        result
    }

    /// Méthode pour vider manuellement les données collectées
    pub fn clear_collected_data(&self) {
        let mut data = self.latest_data.lock().unwrap();
        data.temperature = None;
        data.humidity = None;
        data.sound_level = None;
        data.dust_concentration = None;
        println!("🗑️  Données collectées vidées manuellement");
    }

    /// Méthode pour obtenir le nombre de sessions avec données
    pub fn get_collected_data_count(&self) -> usize {
        let data = self.latest_data.lock().unwrap();
        data.has_data() as usize
    }

    // Méthodes legacy pour compatibilité (dépréciées)
    #[deprecated(note = "Utilisez collect_climate_data et send_all_collected_data à la place")]
    #[allow(dead_code)]
    pub async fn send_climate_data(&self, temperature: f32, humidity: f32) -> Result<(), Box<dyn Error>> {
        self.collect_climate_data(temperature, humidity).await?;
        self.send_all_collected_data().await
    }
    
    #[deprecated(note = "Utilisez collect_sound_data et send_all_collected_data à la place")]
    #[allow(dead_code)]
    pub async fn send_sound_data(&self, sound_level: f32) -> Result<(), Box<dyn Error>> {
        self.collect_sound_data(sound_level).await?;
        self.send_all_collected_data().await
    }
    
    #[deprecated(note = "Utilisez collect_air_quality_data et send_all_collected_data à la place")]
    #[allow(dead_code)]
    pub async fn send_air_quality_data(&self, pm25: f32, pm10: f32) -> Result<(), Box<dyn Error>> {
        self.collect_air_quality_data(pm25, pm10).await?;
        self.send_all_collected_data().await
    }
    
    /// Méthode générique pour envoyer des données à Sensor Community
    async fn send_data(&self, data: SensorCommunityData) -> Result<(), Box<dyn Error>> {
        println!("Envoi vers Sensor Community - Sensor ID: {}", self.sensor_id);
        println!("URL: {}", self.base_url);
        
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
    #[allow(dead_code)]
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
}

impl std::fmt::Display for SensorCommunityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Sensor Community Error: {}", self.message)
    }
}

impl Error for SensorCommunityError {}
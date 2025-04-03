use reqwest::Client as HttpClient;
use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;

#[derive(Clone)]
pub struct CustomInfluxClient {
    http_client: HttpClient,
    url: String,
    token: String,
    org: String,
    bucket: String,
}

impl CustomInfluxClient {
    pub fn new(url: String, token: String, org: String, bucket: String) -> Self {
        Self {
            http_client: HttpClient::new(),
            url,
            token,
            org,
            bucket,
        }
    }
    
    pub async fn write_point(&self, measurement: &str, tags: &[(&str, &str)], fields: &[(&str, f64)]) -> Result<(), Box<dyn Error>> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as i64;
        
        // Construire la ligne en format Line Protocol
        let mut line = format!("{}", measurement);
        
        // Ajouter les tags
        for (key, value) in tags {
            line.push_str(&format!(",{}={}", key, value));
        }
        
        line.push_str(" ");
        
        // Ajouter les champs
        let mut first = true;
        for (key, value) in fields {
            if !first {
                line.push_str(",");
            }
            line.push_str(&format!("{}={}", key, value));
            first = false;
        }
        
        // Ajouter le timestamp
        line.push_str(&format!(" {}", timestamp));
        
        println!("Line Protocol: {}", line);
        
        // Construire l'URL pour l'écriture
        let write_url = format!("{}/api/v2/write?org={}&bucket={}", 
                              self.url, self.org, self.bucket);
        
        println!("Writing to URL: {}", write_url);
        
        // Envoyer la requête
        let response = self.http_client
            .post(&write_url)
            .header("Authorization", format!("Token {}", self.token))
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(line)
            .send()
            .await?;
        
        // Vérifier le statut de la réponse
        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(format!("InfluxDB error: status={}, body={}", status, body).into())
        }
    }
}
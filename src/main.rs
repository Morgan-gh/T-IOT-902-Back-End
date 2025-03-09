use actix_multipart::Multipart;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MessageData {
    message: String,
}

#[post("/echo")]
async fn echo(mut payload: Multipart) -> impl Responder {
    let mut message = String::new();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => return HttpResponse::BadRequest().body("Invalid form data"),
        };

        // Lire le contenu du champ
        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(data) => data,
                Err(_) => return HttpResponse::BadRequest().body("Error reading data"),
            };

            if field.name() == "message" {
                message.push_str(&String::from_utf8_lossy(&data));
            }
        }
    }

    if message.is_empty() {
        return HttpResponse::BadRequest().body("Missing 'message' field");
    }

    // Renvoie le message reçu en réponse JSON
    HttpResponse::Ok().json(MessageData { message })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

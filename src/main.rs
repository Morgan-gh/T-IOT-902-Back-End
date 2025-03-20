mod routes;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use mysql_async::Pool;
use prometheus::{Encoder, TextEncoder, register_counter, register_histogram, Counter, Histogram};

lazy_static! {
    static ref REQUEST_COUNTER: Counter = register_counter!(
        "http_requests_total",
        "Nombre total de requêtes HTTP"
    ).unwrap();

    static ref REQUEST_DURATION: Histogram = register_histogram!(
        "http_request_duration_seconds",
        "Durée des requêtes HTTP"
    ).unwrap();
}

#[derive(Serialize, Deserialize)]
struct MessageData {
    message: String,
}

#[get("/metrics")]
async fn metrics() -> impl Responder {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    let metric_families = prometheus::gather(); // ✅ Appel de Prometheus corrigé
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(String::from_utf8(buffer).unwrap())
}

#[get("/echo")]
async fn echo() -> impl Responder {
    REQUEST_COUNTER.inc(); // ✅ Correctement reconnu
    let timer = REQUEST_DURATION.start_timer(); // ✅ Correctement reconnu

    let response = HttpResponse::Ok().body("Echo!");

    timer.observe_duration();
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connexion à la base de données
    let pool = Pool::new("mysql://iot_user:iot_password@127.0.0.1/iot_sensors");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config) // 👈 Appel automatique des routes
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
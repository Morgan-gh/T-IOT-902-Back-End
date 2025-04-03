use actix_web::web;

mod sound;
mod humidity;
mod dust;

pub fn config(cfg: &mut web::ServiceConfig) {
    sound::register(cfg);
    humidity::register(cfg);
    dust::register(cfg);
}
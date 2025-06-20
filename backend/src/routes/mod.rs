pub mod sound;
pub mod humidity;
pub mod dust;
pub mod sensor_community;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    sound::register(cfg);
    humidity::register(cfg);
    dust::register(cfg);
    sensor_community::register(cfg);
}
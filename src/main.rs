use std::io::Error;
use actix_web::{App, HttpServer};
use actix_web::web::resource;

mod logger;
mod config;

#[actix_web::main]
async fn main() {
    logger::init();
    log::debug!("Logger initialized");
    log::info!("Starting racker");

    log::info!("Loading config");
    let config = match config::load() {
        Ok(config) => {
            log::info!("Config loaded successfully");
            config
        }
        Err(err) => {
            log::error!("Failed to load config: {}", err);
            log_error_and_panic(err)
        }
    };
    log::debug!("Loaded config: {:?}", config);


    HttpServer::new(|| {
        App::new().service(resource("/").to(|| async {
            r#"{ "message": "Hello, world!" }"#
        }))
    }).bind((config.network().host(), config.network().port())).unwrap().run().await.unwrap();
}

fn log_error_and_panic(err: Error) -> ! {
    log::error!("Received unrecoverable error: {}", err);
    log::error!("Exiting");
    panic!()
}

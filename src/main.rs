use std::io::Error;
use std::sync::Mutex;
use crate::config::Config;

mod logger;
mod config;
mod http;
mod head;
mod plugin;

pub(crate) struct RackerState {
    pub(crate) config: Config,
    pub(crate) heads: Mutex<head::Heads>,
}

#[actix_web::main]
async fn main() {
    logger::init();
    log::debug!("Logger initialized");
    log::info!("Starting racker");

    log::info!("Loading config");
    let config = match config::load() {
        Ok(config) => {
            log::debug!("Config loaded successfully");
            config
        }
        Err(err) => {
            log::error!("Failed to load config: {}", err);
            log_error_and_panic(err)
        }
    };
    log::debug!("Loaded config: {:?}", config);

    log::debug!("Creating state");
    let state = RackerState {
        config,
        heads: Mutex::from(head::default()),
    };
    log::debug!("State created");

    log::info!("Starting HTTP server");
    http::start(state).await;

    log::info!("HTTP server stopped");
}

fn log_error_and_panic(err: Error) -> ! {
    log::error!("Received unrecoverable error: {}", err);
    log::error!("Exiting");
    panic!()
}

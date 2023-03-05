use std::io::Error;

mod logger;
mod config;
mod http;

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

    log::info!("Starting HTTP server");
    http::start(config).await;

    log::info!("HTTP server stopped");
}

fn log_error_and_panic(err: Error) -> ! {
    log::error!("Received unrecoverable error: {}", err);
    log::error!("Exiting");
    panic!()
}

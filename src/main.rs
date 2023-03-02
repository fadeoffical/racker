use std::io::Error;

mod logger;
mod config;

fn main() {
    logger::init();
    log::info!("Starting racker");

    log::info!("Loading config");
    let config = match config::load() {
        Ok(config) => {
            log::info!("Config loaded successfully");
            config
        },
        Err(err) => {
            log::error!("Failed to load config: {}", err);
            log_error_and_panic(err)
        }
    };

    log::info!("Host: {}", config.network().host());
}

fn log_error_and_panic(err: Error) -> ! {
    log::error!("Received unrecoverable error: {}", err);
    log::error!("Exiting");
    panic!()
}

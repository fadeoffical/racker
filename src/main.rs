use crate::cli::Cli;
use clap::Parser;
use racker_plugin::PluginManager;
use std::error::Error;
use std::sync::Mutex;

use crate::config::Config;
use crate::user::{Permission, User};

mod cli;
mod config;
mod head;
mod http;
mod logger;
mod user;

pub(crate) struct RackerState {
    pub(crate) config: Config,
    pub(crate) heads: Mutex<head::Heads>,
    pub(crate) users: Mutex<Vec<User>>,
}

#[actix_web::main]
async fn main() {
    let cli = Cli::parse();

    logger::init(&cli);
    log::debug!("Logger initialized");
    log::info!("Starting racker");

    log::info!("Loading config");
    let config = load_config(cli.clone());

    log::info!("Loading plugins");
    let mut plugin_manager = PluginManager::create().load_plugins();

    log::debug!("Creating state");
    let state = RackerState {
        config,
        heads: Mutex::from(head::default()),
        users: Mutex::new(vec![User::new(
            "admin".to_string(),
            "password".to_string(),
            vec![Permission::All],
        )]),
    };
    log::debug!("State created");

    log::info!("Starting HTTP server");
    http::start(state).await;

    log::info!("HTTP server stopped");
}

fn load_config(arguments: Cli) -> Config {
    log::trace!("-> main::load_config()");
    let config = match config::load(arguments) {
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

    log::trace!("<- main::load_config()");
    config
}

fn log_error_and_panic(err: Box<dyn Error>) -> ! {
    log::error!("Received unrecoverable error: {}", err);
    log::error!("Exiting");
    panic!()
}

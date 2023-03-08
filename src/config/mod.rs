use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::config::cli::Cli;
use crate::log_error_and_panic;
use racker::common::data::Network;

pub(crate) mod cli;

const CONFIG_FILE_DEFAULT_CONTENTS: &str = include_str!("racker.json");

pub(crate) fn load(arguments: Cli) -> Result<Config, Box<dyn Error>> {
    log::trace!("Entered config::load()");

    let config_file_path = match arguments.config_file {
        None => PathBuf::from(cli::CONFIG_FILE_DEFAULT_PATH),
        Some(path) => path,
    };

    let config_file_path = match config_file_path.canonicalize() {
        Ok(path) => path,
        Err(err) => {
            log::error!("Failed to canonicalize config file path: {}", err);
            log::warn!("Using default config");

            log::trace!("Returning config::load()");
            return Err(Box::new(err));
        }
    };

    log::debug!("Config file path: {:?}", &config_file_path);

    if !&config_file_path.exists() {
        log::warn!("Config file not found at {:?}", &config_file_path);
        log::warn!("Creating default config file");

        let mut file = match File::create(&config_file_path) {
            Ok(file) => file,
            Err(err) => {
                log::error!(
                    "Failed to open config file at {:?}: {}",
                    &config_file_path,
                    err
                );

                log::trace!("Returning config::load()");
                return Err(Box::new(err));
            }
        };

        let config = match file.write_all(CONFIG_FILE_DEFAULT_CONTENTS.as_bytes()) {
            Ok(_) => Ok(Config::default()),
            Err(err) => {
                log::error!(
                    "Failed to write to config file at {:?}: {}",
                    &config_file_path,
                    err
                );

                log::trace!("Returning config::load()");
                Err(Box::new(err))
            }
        };

        // wtf
        return match config {
            Ok(config) => Ok(config),
            Err(err) => Err(Box::new(err)),
        };
    }

    let file = match File::open(&config_file_path) {
        Ok(file) => file,
        Err(err) => {
            log::error!(
                "Failed to open config file at {:?}: {}",
                &config_file_path,
                err
            );
            log::warn!("Using default config");

            log::trace!("Returning config::load()");
            return Ok(Config::default());
        }
    };

    let config = match serde_json::from_reader::<&File, Config>(&file) {
        Ok(mut config) => {
            if let Some(port) = &arguments.port {
                config.network.set_port(*port);
            }

            if let Some(host) = &arguments.host {
                config.network.set_host(host);
            }

            Ok(config)
        }
        Err(err) => {
            log::error!("Failed to parse config file: {}", err);
            log::warn!("Using default config");

            log::trace!("Returning config::load()");
            log_error_and_panic(Box::new(err));
        }
    };

    log::trace!("Returning config::load()");
    config
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Config {
    network: Network,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: Network::new("127.0.0.1", 527),
        }
    }
}

impl Config {
    pub(crate) fn network(&self) -> &Network {
        &self.network
    }
}

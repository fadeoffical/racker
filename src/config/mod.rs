use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
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

    log::debug!("Config file path: {:?}", &config_file_path);

    if !&config_file_path.exists() {
        log::warn!("Config file not found at {:?}", &config_file_path);
        log::warn!("Creating default config file");

        if let Some(config_file_dir) = config_file_path.parent() {
            log::debug!("Creating config file directory: {:?}", &config_file_dir);
            fs::create_dir_all(config_file_dir)?;
        };

        log::debug!("Creating config file: {:?}", &config_file_path);
        let mut file = match File::create(&config_file_path) {
            Ok(file) => file,
            Err(err) => {
                log::error!(
                    "Failed to create config file at {:?}: {}",
                    &config_file_path,
                    err
                );

                log::trace!("Returning config::load()");
                return Err(Box::new(err));
            }
        };

        log::debug!("Writing default config to config file");
        match file.write_all(CONFIG_FILE_DEFAULT_CONTENTS.as_bytes()) {
            Ok(_) => log::debug!("Successfully wrote default config to config file"),
            Err(err) => {
                log::error!(
                    "Failed to write to config file at {:?}: {}",
                    &config_file_path,
                    err
                );

                return Err(Box::new(err));
            }
        };
    }

    log::debug!("Opening config file: {:?}", &config_file_path);
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

    log::debug!("Parsing config file");
    let config = match serde_json::from_reader::<&File, Config>(&file) {
        Ok(mut config) => {
            if let Some(host) = &arguments.host {
                log::debug!("Setting host to {}", host);
                config.network.set_host(host);
            }

            if let Some(port) = &arguments.port {
                log::debug!("Setting port to {}", port);
                config.network.set_port(*port);
            }

            log::debug!("Successfully parsed config file");
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

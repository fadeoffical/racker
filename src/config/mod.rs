use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use serde::{Deserialize, Serialize};

use racker_common::data::Network;

const CONFIG_FILE_DEFAULT_PATH: &str = "racker.json";
const CONFIG_FILE_DEFAULT_CONTENTS: &str = include_str!("racker.json");

pub(crate) fn load() -> Result<Config, io::Error> {
    log::trace!("Entered config::load()");

    let path = Path::new(CONFIG_FILE_DEFAULT_PATH);

    if !path.exists() {
        log::warn!("Config file not found at {}", CONFIG_FILE_DEFAULT_PATH);
        log::warn!("Creating default config file");

        let mut file = match File::create(CONFIG_FILE_DEFAULT_PATH) {
            Ok(file) => file,
            Err(err) => {
                log::error!("Failed to open config file at {}: {}", CONFIG_FILE_DEFAULT_PATH, err);

                log::trace!("Returning config::load()");
                return Err(err);
            }
        };

        let config = match file.write_all(CONFIG_FILE_DEFAULT_CONTENTS.as_bytes()) {
            Ok(_) => Ok(Config::default()),
            Err(err) => {
                log::error!("Failed to write to config file at {}: {}", CONFIG_FILE_DEFAULT_PATH, err);

                log::trace!("Returning config::load()");
                Err(err)
            }
        };

        return config;
    }

    let file = match File::open(CONFIG_FILE_DEFAULT_PATH) {
        Ok(file) => file,
        Err(err) => {
            log::error!("Failed to open config file at {}: {}", CONFIG_FILE_DEFAULT_PATH, err);
            log::warn!("Using default config");

            log::trace!("Returning config::load()");
            return Ok(Config::default());
        }
    };

    let config = match serde_json::from_reader(&file) {
        Ok(config) => Ok(config),
        Err(err) => {
            log::error!("Failed to parse config file: {}", err);
            log::warn!("Using default config");

            log::trace!("Returning config::load()");
            return Ok(Config::default());
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

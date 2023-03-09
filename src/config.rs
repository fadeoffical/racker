use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::cli;
use crate::cli::Cli;
use racker::common::data::Network;

const CONFIG_FILE_DEFAULT_CONTENTS: &str = include_str!("include/racker.json");

pub(crate) fn load(cli: Cli) -> Result<Config, Box<dyn Error>> {
    log::trace!("-> config::load()");

    // update config file path if specified via cli arguments
    let config_file_path = match cli.config_file {
        None => PathBuf::from(cli::CONFIG_FILE_DEFAULT_PATH),
        Some(ref path) => path.to_path_buf(),
    };
    log::debug!("Config file path: {:?}", &config_file_path);

    let file = match config_file_from_path(&config_file_path) {
        Ok(file) => file,
        Err(err) => {
            log::error!(
                "Failed to open config file at {:?}: {}",
                &config_file_path,
                err
            );

            log::trace!("<- config::load()");
            return Err(err);
        }
    };

    log::debug!("Parsing config file");
    return match parse_config_file(&file) {
        Ok(mut config) => {
            log::debug!("Successfully parsed config file");

            cli::update_config(&cli, &mut config);

            log::trace!("<- config::load()");
            Ok(config)
        }
        Err(err) => Err(err),
    };
}

fn parse_config_file(file: &File) -> Result<Config, Box<dyn Error>> {
    log::trace!("-> config::parse_config_file()");

    let config = match serde_json::from_reader::<&File, Config>(file) {
        Ok(config) => config,
        Err(err) => {
            log::error!("Failed to parse config file: {}", err);

            log::trace!("<- config::parse_config_file()");
            return Err(Box::new(err));
        }
    };

    log::trace!("<- config::parse_config_file()");
    Ok(config)
}

fn config_file_from_path(path: &Path) -> Result<File, Box<dyn Error>> {
    log::trace!("-> config::config_file_from_path()");

    if !path.exists() {
        log::warn!("Config file not found at {:?}", &path);
        log::warn!("Creating default config file");

        let mut file = match create_config_file(path) {
            Ok(file) => file,
            Err(err) => {
                log::error!("Failed to create config file at {:?}: {}", &path, err);
                log::trace!("<- config::config_file_from_path()");
                return Err(err);
            }
        };

        log::debug!("Writing default config to config file");
        if let Err(err) = write_default_config_file(&mut file) {
            log::error!("Failed to write to config file at {:?}: {}", &path, err);
            log::trace!("<- config::config_file_from_path()");
            return Err(err);
        };
    }

    let file = File::open(path)?;

    log::trace!("<- config::config_file_from_path()");
    Ok(file)
}

fn create_config_file(path: &Path) -> Result<File, Box<dyn Error>> {
    log::trace!("-> config::create_config_file()");

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            log::debug!("Creating config file directory: {:?}", &parent);
            fs::create_dir_all(parent)?;
        }
    }
    let file = File::create(path)?;

    log::trace!("<- config::create_config_file()");
    Ok(file)
}

fn write_default_config_file(file: &mut File) -> Result<(), Box<dyn Error>> {
    log::trace!("-> config::write_default_config_file()");

    file.write_all(CONFIG_FILE_DEFAULT_CONTENTS.as_bytes())?;

    log::trace!("<- config::write_default_config_file()");
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Config {
    pub(crate) network: Network,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: Network::new("0.0.0.0", 527),
        }
    }
}

impl Config {
    pub(crate) fn network(&self) -> &Network {
        &self.network
    }
}

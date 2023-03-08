use std::path::PathBuf;

// use is public so that the parse method can be called outside of this module
// and we dont have to clap::Parser but instead can use this one (looks cleaner)
pub(crate) use clap::Parser;
use crate::config::Config;

pub(crate) const CONFIG_FILE_DEFAULT_PATH: &str = "/etc/racker/racker.json";

#[derive(Debug, Clone, Parser)]
#[clap(version = "0.1.0", author = "fade")]
pub(crate) struct Cli {
    #[clap(short, long, default_value = CONFIG_FILE_DEFAULT_PATH)]
    pub(crate) config_file: Option<PathBuf>,

    #[clap(long = "Chost")]
    pub(crate) host: Option<String>,

    #[clap(long = "Cport")]
    pub(crate) port: Option<u16>,
}

pub(crate) fn update_config(cli: &Cli, config: &mut Config) {
    log::trace!("-> config::cli::update_config()");

    if let Some(host) = &cli.host {
        log::debug!("Setting host to {}", host);
        config.network.set_host(host);
    }

    if let Some(port) = &cli.port {
        log::debug!("Setting port to {}", port);
        config.network.set_port(*port);
    }

    log::trace!("<- config::cli::update_config()");
}

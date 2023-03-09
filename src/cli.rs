use std::path::PathBuf;

// use is public so that the parse method can be called outside of this module
// and we dont have to clap::Parser but instead can use this one (looks cleaner)
use crate::config::Config;
pub(crate) use clap::Parser;

pub(crate) const CONFIG_FILE_DEFAULT_PATH: &str = "/etc/racker/racker.json";

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
pub(crate) struct Cli {
    #[clap(short, long, default_value = CONFIG_FILE_DEFAULT_PATH, help = "Path to the config file")]
    pub(crate) config_file: Option<PathBuf>,

    #[clap(
        long,
        short,
        default_value = "info",
        help = "Log level: trace, debug, info, warn, error, off"
    )]
    pub(crate) log_level: Option<log::LevelFilter>,

    #[clap(long, help = "Host to bind to. Overrides the config file")]
    pub(crate) host: Option<String>,

    #[clap(long, help = "Port to bind to. Overrides the config file")]
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

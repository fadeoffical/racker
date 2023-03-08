use std::path::PathBuf;

// use is public so that the parse method can be called outside of this module
// and we dont have to clap::Parser but instead can use this one (looks cleaner)
pub(crate) use clap::Parser;

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

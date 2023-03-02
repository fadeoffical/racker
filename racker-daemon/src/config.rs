use std::path::PathBuf;

pub(crate) struct RackerConfig {

    /// The path to the config file
    config_file: Option<PathBuf>,
}

impl RackerConfig {
    pub(crate) fn new() -> Self {
        Self {
            config_file: Some(PathBuf::from("/etc/racker/rackerd.json")),
        }
    }
}

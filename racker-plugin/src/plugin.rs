use std::path::PathBuf;
use libloading::Library;

use crate::Plugin;
use crate::meta::PluginMeta;

pub type PluginId = usize;

pub enum PluginState {
    /// The plugin is loaded and initialized.
    Loaded,

    /// The plugin experienced an error while loading.
    Error(PluginLoadError),

    /// The plugin is unloaded.
    Unloaded,
}

pub enum PluginLoadError {
    /// The plugin could not be loaded.
    LoadError(String),

    /// The plugin could not be initialized.
    InitError(String),
}

pub struct PluginContainer {
    id: PluginId,
    plugin: Option<Box<dyn Plugin>>,

    state: PluginState,

    tmp_dir: PathBuf,
    tmp_file: PathBuf,

    meta: Option<PluginMeta>,
    library: Option<Library>,
}

impl PluginContainer {
    pub(crate) fn init(tmp_dir: PathBuf, tmp_file: PathBuf) -> Self {
        Self {
            id: 0,
            plugin: None,

            state: PluginState::Unloaded,

            tmp_dir,
            tmp_file,

            meta: None,
            library: None,
        }
    }

    pub fn id(&self) -> PluginId {
        self.id
    }

    pub fn state(&self) -> &PluginState {
        &self.state
    }

    pub fn meta(&self) -> PluginMeta {
        self.meta.as_ref().unwrap().clone()
    }

    pub fn tmp_dir(&self) -> PathBuf {
        self.tmp_dir.clone()
    }

    pub fn tmp_file(&self) -> PathBuf {
        self.tmp_file.clone()
    }

    pub fn plugin_reference(&self) -> PluginReference {
        PluginReference { id: self.id }
    }

    pub fn plugin(&self) -> &dyn Plugin {
        self.plugin.as_ref().unwrap().as_ref()
    }
}

pub struct PluginReference {
    /// The internal plugin id of the plugin.
    /// This is used to identify the plugin in the plugin manager.
    id: PluginId,
}

impl PluginReference {
    pub fn id(&self) -> PluginId {
        self.id
    }
}

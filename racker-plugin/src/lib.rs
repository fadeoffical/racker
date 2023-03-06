pub mod plugin;
pub mod manifest;

use std::any::Any;
use std::{fs, io};
use std::path::PathBuf;
use zip::ZipArchive;
use crate::manifest::PluginManifest;

use crate::plugin::{PluginContainer, PluginState};

/// Declares a plugin.
#[macro_export]
macro_rules! racker_plugin {
    ($pl_t:ty, $pl_c:path) => {
        #[no_mangle]
        pub extern "C" fn _racker_plugin_create() -> *mut dyn $crate::Plugin {
            let constructor: fn() -> $pl_t = $pl_c;
            let plugin = constructor();
            Box::into_raw(Box::new(plugin))
        }
    };
}

pub trait Plugin: Any + Send + Sync {
    /// Called when the plugin is loaded.
    fn on_load(&self);

    /// Called when the plugin is unloaded.
    fn on_unload(&self);
}

pub struct PluginManager {
    plugins: Vec<PluginContainer>,
}

const PLUGIN_DIR: &str = "plugins";
const PLUGIN_EXT: &str = "zip";
const PLUGIN_RUN_DIR: &str = "temp/plugins";

const PLUGIN_MANIFEST_FILE: &str = "plugin.json";

impl PluginManager {
    pub fn create() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load_plugins(mut self) -> Self {
        self.init().unwrap();
        self.unzip_plugins().unwrap();
        self.load();
        self
    }
}

impl PluginManager {
    fn init(&mut self) -> Result<(), io::Error> {
        let plugin_dir = get_plugin_dir()?;
        let plugin_run_dir = get_temp_dir()?;

        let plugin_dir = fs::canonicalize(plugin_dir)?;
        let plugin_files = fs::read_dir(&plugin_dir)?;

        for plugin_file in plugin_files {
            let plugin_file = plugin_file?;

            let plugin_path = plugin_file.path();
            let path_str = plugin_path.to_str().unwrap();

            if !plugin_path.is_file() {
                continue;
            }

            if !plugin_path.extension().unwrap().eq(PLUGIN_EXT) {
                log::warn!("Invalid plugin file: {}", &path_str);
                continue;
            }

            log::info!("Loading plugin: {}", &plugin_file.file_name().to_str().unwrap());

            // temp/plugins/ + plugin.zip = temp/plugins/plugin.zip
            let tmp_plugin_path = plugin_run_dir.join(plugin_path.file_name().unwrap());

            let tmp_plugin_dir = create_dir(tmp_plugin_path.to_str().unwrap())?;

            // temp/plugins/plugin.zip + plugin.zip = temp/plugins/plugin.zip/plugin.zip
            let tmp_plugin_file_path = tmp_plugin_dir.join(plugin_path.file_name().unwrap());

            fs::copy(&plugin_path, &tmp_plugin_file_path)?;

            let plugin = PluginContainer::init(tmp_plugin_dir, tmp_plugin_file_path);
            self.plugins.push(plugin);
        }

        Ok(())
    }

    fn unzip_plugins(&mut self) -> Result<(), io::Error> {
        self.plugins.iter()
            .map(|container| (container.tmp_dir(), container.tmp_file()))
            .for_each(|(_dir, file)| {
                let zip_file = fs::File::open(&file).unwrap();
                let mut zip = ZipArchive::new(zip_file).unwrap();

                let plugin_file = match zip.by_name(PLUGIN_MANIFEST_FILE) {
                    Ok(file) => file,
                    Err(_) => {
                        log::error!("Plugin has no manifest: {}", &file.file_name().unwrap().to_str().unwrap());
                        return;
                    }
                };
                let plugin_manifest: PluginManifest = serde_json::from_reader(plugin_file).unwrap();

                log::info!("Plugin: {:?}", plugin_manifest);

            });

        Ok(())
    }

    fn load(&mut self) {
        self.plugins.iter_mut()
            .for_each(|container| {
                // container.plugin().on_load(); todo
                container.set_state(PluginState::Loaded)
            });
    }

    // pub fn load_plugin_from_file(&mut self, path: &str) {
    //     unsafe {
    //         let lib = Library::new(path).unwrap();
    //         let create: Symbol<unsafe fn() -> *mut dyn Plugin> = lib.get(b"_racker_plugin_create").unwrap();
    //         let plugin = create();
    //         let plugin = Box::from_raw(plugin);
    //
    //         plugin.on_load();
    //     }
    // }
}

fn create_dir(path: &str) -> Result<PathBuf, io::Error> {
    let dir = PathBuf::from(path);
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

fn get_plugin_dir() -> Result<PathBuf, io::Error> {
    let plugin_dir = create_dir(PLUGIN_DIR)?;
    Ok(plugin_dir)
}

fn get_temp_dir() -> Result<PathBuf, io::Error> {
    let temp_dir = create_dir(PLUGIN_RUN_DIR)?;
    Ok(temp_dir)
}

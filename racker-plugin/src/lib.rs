use std::any::Any;
use libloading::{Library, Symbol};

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
    /// Returns the plugin's metadata.
    fn meta(&self) -> PluginMeta;

    /// Called when the plugin is loaded.
    fn on_load(&self);

    /// Called when the plugin is unloaded.
    fn on_unload(&self);
}

pub struct PluginMeta {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn create() -> Self {
        Self { plugins: Vec::new(), loaded_libraries: Vec::new() }
    }

    pub fn load_plugin_from_file(&mut self, path: &str) {
        unsafe {
            let lib = Library::new(path).unwrap();
            let create: Symbol<unsafe fn() -> *mut dyn Plugin> = lib.get(b"_racker_plugin_create").unwrap();
            let plugin = create();
            let plugin = Box::from_raw(plugin);

            plugin.on_load();

            self.plugins.push(plugin);
            self.loaded_libraries.push(lib);
        }
    }

    pub fn load_plugin(&mut self, plugin: Box<dyn Plugin>) {
        plugin.on_load();
        self.plugins.push(plugin);
    }

    pub fn unload_plugin(&mut self, name: &str) {
        let mut index = None;
        for (i, plugin) in self.plugins.iter().enumerate() {
            if plugin.meta().name == name {
                index = Some(i);
                break;
            }
        }

        if let Some(i) = index {
            self.plugins[i].on_unload();
            self.plugins.remove(i);
        }
    }
}

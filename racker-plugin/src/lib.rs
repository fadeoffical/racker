use std::any::Any;
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

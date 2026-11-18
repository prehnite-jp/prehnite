use tauri::{
    Manager, Runtime,
    plugin::{Builder, TauriPlugin},
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
compile_error!("mobile platform does not support.");

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::PrehniteData;
#[cfg(mobile)]
use mobile::PrehniteData;
use prehnite_data::manager::managed::PrehniteManaged;
use prehnite_data::models::document::Document;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the prehnite-data APIs.
pub trait PrehniteDataExt<R: Runtime> {
    fn prehnite_data(&self) -> &PrehniteData<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PrehniteDataExt<R> for T {
    fn prehnite_data(&self) -> &PrehniteData<R> {
        self.state::<PrehniteData<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("prehnite-data")
        .invoke_handler(tauri::generate_handler![commands::ping])
        .setup(|app, api| {
            #[cfg(desktop)]
            let prehnite_data = desktop::init(app, api)?;
            app.manage(prehnite_data);
            Ok(())
        })
        .build()
}

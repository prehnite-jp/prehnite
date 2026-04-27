#![cfg_attr(feature = "release", windows_subsystem = "windows")]

use crate::app::settings::GlobalSettings;
use crate::util::alert::AlertResult;
use crate::windows::main_window::menu::main_window_menu_bar;
use crate::windows::main_window::PrehniteApp;
use dioxus::desktop::{Config, WindowBuilder};

pub mod app;
pub mod assets;
pub mod components;
pub mod style;
pub mod util;
pub mod windows;

#[tracing::instrument]
fn initializer() -> anyhow::Result<()> {
    #[tokio::main]
    async fn initializer_() -> anyhow::Result<()> {
        app::db::initialize_global_db_pool().await?;
        prehnite_core::log::initialize_logger()?;
        Ok(())
    }
    initializer_()
}

fn fetch_global_settings() -> anyhow::Result<GlobalSettings> {
    #[tokio::main]
    async fn func() -> anyhow::Result<GlobalSettings> {
        Ok(GlobalSettings::fetch().await?)
    }
    func()
}

#[cfg(feature = "desktop")]
fn main() -> anyhow::Result<()> {
    initializer().unwrap_or_alert();

    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default()
                .with_menu(main_window_menu_bar().get_menu().clone())
                .with_window(WindowBuilder::new().with_title("Prehnite")),
        )
        .with_context(fetch_global_settings()?)
        .launch(PrehniteApp);

    Ok(())
}

#[cfg(not(feature = "desktop"))]
fn main() -> anyhow::Result<()> {
    compile_error!("This application only supported desktop.");

    Ok(())
}

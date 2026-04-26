#![cfg_attr(feature = "release", windows_subsystem = "windows")]

use crate::app::settings;
use crate::util::alert::AlertResult;
use crate::windows::main_window::menu::main_window_menu_bar;
use crate::windows::main_window::PrehniteApp;
use dioxus::desktop::{Config, WindowBuilder};
use std::ops::Deref;

pub mod app;
pub mod assets;
pub mod components;
pub mod util;
pub mod windows;

#[tracing::instrument]
fn initializer() -> anyhow::Result<()> {
    #[tokio::main]
    async fn initializer_() -> anyhow::Result<()> {
        app::db::initialize_global_db_pool().await?;
        prehnite_core::log::initialize_logger()?;
        settings::load().await?;
        Ok(())
    }
    initializer_()
}

#[cfg(feature = "desktop")]
fn main() -> anyhow::Result<()> {
    initializer().unwrap_or_alert();

    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default()
                .with_menu(
                    main_window_menu_bar()
                        .deref()
                        .as_ref()
                        .map(|x| x.get_menu().clone()),
                )
                .with_window(WindowBuilder::new().with_title("Prehnite")),
        )
        .launch(PrehniteApp);

    Ok(())
}

#[cfg(not(feature = "desktop"))]
fn main() -> anyhow::Result<()> {
    compile_error!("This application only supported desktop.");

    Ok(())
}

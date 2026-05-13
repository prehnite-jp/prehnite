#![cfg_attr(feature = "release", windows_subsystem = "windows")]

use crate::app::settings::fetch::Fetch;
use crate::util::alert::AlertResult;
use crate::window::main_window::menu::main_window_menu_bar;
use crate::window::main_window::PrehniteApp;
use app::settings::setting_schema::GlobalSettings;
use dioxus::desktop::{Config, WindowBuilder};
use std::env;
use std::path::Path;
use dioxus_desktop::LogicalSize;

pub mod app;
pub mod assets;
pub mod components;
pub mod components_;
pub mod custom_dx_components;
pub mod style;
pub mod util;
pub mod window;

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

    let config = Config::new()
        .with_menu(main_window_menu_bar().get_menu().clone())
        .with_window(
            WindowBuilder::new()
                .with_inner_size(LogicalSize::new(1024, 760))
                .with_title("Prehnite"),
        );

    #[cfg(all(target_os = "windows", debug_assertions))]
    let config = config.with_data_directory(
        Path::new(&env::var("LOCALAPPDATA").expect("エラー: LOCALAPPDATAが取得できません。"))
            .join("prehnite-dev"),
    );

    dioxus::LaunchBuilder::new()
        .with_cfg(config)
        .with_context(fetch_global_settings()?)
        .launch(PrehniteApp);

    Ok(())
}

#[cfg(not(feature = "desktop"))]
fn main() -> anyhow::Result<()> {
    compile_error!("This application only supported desktop.");

    Ok(())
}

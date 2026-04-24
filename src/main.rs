#![cfg_attr(feature = "release", windows_subsystem = "windows")]

use crate::app::{initializer, PrehniteApp};
use crate::util::alert::AlertResult;

pub mod app;
pub mod pages;
pub mod util;

#[tracing::instrument]
fn initializer() -> anyhow::Result<()> {
    #[tokio::main]
    async fn initializer_() -> anyhow::Result<()> {
        app::db::initialize_global_db_pool().await?;
        prehnite_core::log::initialize_logger()?;
        initializer::load_settings().await?;
        Ok(())
    }
    initializer_()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    initializer().unwrap_or_alert();

    #[cfg(feature = "desktop")]
    dioxus::launch(PrehniteApp);

    Ok(())
}

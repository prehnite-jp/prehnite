#![cfg_attr(feature = "release", windows_subsystem = "windows")]

use crate::application::PrehniteApp;

pub mod api;
pub mod application;

#[tokio::main]
#[tracing::instrument]
async fn initializer() -> anyhow::Result<()> {
    application::db::initialize_global_db_pool().await?;
    prehnite_core::log::initialize_logger()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    initializer()?;

    #[cfg(feature = "desktop")]
    dioxus::launch(PrehniteApp);

    Ok(())
}

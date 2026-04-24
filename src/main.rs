#![cfg_attr(feature = "release", windows_subsystem = "windows")]

use crate::application::PrehniteApp;

pub mod application;
pub mod backend;
pub mod frontend;

#[tokio::main]
#[tracing::instrument]
async fn initializer() -> anyhow::Result<()> {
    #[cfg(feature = "server")]
    async fn initialize_server() -> anyhow::Result<()> {
        backend::db::initialize_global_db_pool().await?;
        prehnite_core::log::initialize_logger()?;
        Ok(())
    }

    #[cfg(feature = "server")]
    initialize_server().await?;

    #[cfg(feature = "desktop")]
    async fn desktop_initializer() -> anyhow::Result<()> {
        Ok(())
    }

    #[cfg(feature = "desktop")]
    desktop_initializer().await?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    initializer()?;

    #[cfg(feature = "desktop")]
    dioxus::launch(PrehniteApp);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        let router = dioxus::server::router(PrehniteApp);
        Ok(router)
    });

    //noinspection RsUnreachableCode
    Ok(())
}

#![cfg_attr(feature = "release", windows_subsystem = "windows")]
pub mod backend;
pub mod frontend;

#[tokio::main]
#[tracing::instrument]
async fn initializer() {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    initializer();

    Ok(())
}

use gpui_kit::App;
use std::error::Error;

pub mod constants;
pub mod env;
pub mod time;
pub mod util;

pub fn init_shared(_cx: &mut App) -> Result<(), Box<dyn Error + Send + Sync>> {
    Ok(())
}

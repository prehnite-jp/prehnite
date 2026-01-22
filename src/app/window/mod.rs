pub mod main_window;
mod menubar;
pub mod resources;
pub mod version_info_window;

use crate::app::window::main_window::MainWindowMessage;
use crate::app::window::resources::APP_ICON_PNG;
use crate::app::window::version_info_window::VersionInfoWindowMessage;
use iced::window::icon::from_file_data;
use iced::{window, Element, Task};
use std::fmt::Debug;
use tracing::error;

pub fn app_default_window_settings() -> window::Settings {
    window::Settings {
        icon: from_file_data(APP_ICON_PNG, Some(image::ImageFormat::Png))
            .or_else(|e| {
                error!("Failed to setup app icon. Error: {e:#?}");
                Err(e)
            })
            .ok(),
        ..Default::default()
    }
}

#[derive(Clone, Debug)]
pub enum WindowMessage {
    MainWindowMessage(MainWindowMessage),
    AboutWindowMessage(VersionInfoWindowMessage),
}

pub trait Window: Debug {
    fn new() -> (Box<dyn Window>, Task<WindowMessage>)
    where
        Self: Sized;

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage>;

    fn view(&'_ self) -> Element<'_, WindowMessage>;

    fn title(&'_ self) -> String;

    fn set_window_id(&mut self, window_id: window::Id);

    fn window_settings() -> window::Settings
    where
        Self: Sized,
    {
        app_default_window_settings()
    }
}

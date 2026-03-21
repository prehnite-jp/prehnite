pub mod editor_window;
pub mod license_info_window;
pub mod main_window;
pub mod new_item_prompt_window;
pub mod setting_window;
pub mod version_info_window;

use crate::app::resources::APP_ICON_PNG;
use crate::app::window::editor_window::EditorWindowMessage;
use crate::app::window::license_info_window::LicenseInfoWindowMessage;
use crate::app::window::main_window::MainWindowMessage;
use crate::app::window::new_item_prompt_window::NewItemPromptWindowMessage;
use crate::app::window::setting_window::SettingWindowMessage;
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
    VersionInfoWindowMessage(VersionInfoWindowMessage),
    SettingWindowMessage(SettingWindowMessage),
    LicenseInfoWindowMessage(LicenseInfoWindowMessage),
    EditorWindowMessage(EditorWindowMessage),
    NewItemPromptWindowMessage(NewItemPromptWindowMessage),
    ReloadFont,
    ReloadLanguage,
}

pub trait Window: Debug {
    fn new() -> Self
    where
        Self: Sized;

    fn init_task() -> Task<WindowMessage>
    where
        Self: Sized,
    {
        Task::none()
    }

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

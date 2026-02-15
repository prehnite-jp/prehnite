use crate::app::window::{Window, WindowMessage};
use iced::window::{Id, Settings};
use iced::{Element, Task};
use prehnite_core::i18n::i18n;

#[derive(Clone, Debug)]
pub enum SettingWindowMessage {}

#[derive(Debug)]
pub struct SettingWindow {
    window_id: Option<Id>,
}

impl Window for SettingWindow {
    fn new() -> SettingWindow
    where
        Self: Sized,
    {
        Self { window_id: None }
    }

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage> {
        Task::none()
    }

    fn view(&'_ self) -> Element<'_, WindowMessage> {
        prehnite_core::i18n::i18n_w("wip").into()
    }

    fn title(&'_ self) -> String {
        i18n("settings")
    }

    fn set_window_id(&mut self, window_id: Id) {
        self.window_id = Some(window_id)
    }

    fn window_settings() -> Settings
    where
        Self: Sized,
    {
        Settings {
            size: (720f32, 560f32).into(),
            minimizable: false,
            resizable: false,
            ..Settings::default()
        }
    }
}

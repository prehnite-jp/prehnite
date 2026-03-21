use crate::app::window::{app_default_window_settings, Window, WindowMessage};
use iced::window::Settings;
use iced::{window, Element, Size, Task};
use prehnite_core::i18n::{i18n, i18n_w};

#[derive(Clone, Debug)]
pub enum NewItemPromptWindowMessage {
    Close,
    Initialize(window::Id),
}

#[derive(Debug)]
pub struct NewItemPromptWindow {
    window_id: Option<window::Id>,
    caller_window_id: Option<window::Id>,
}

impl NewItemPromptWindow {
    fn update_impl(
        &mut self,
        message: NewItemPromptWindowMessage,
    ) -> Task<NewItemPromptWindowMessage> {
        match message {
            NewItemPromptWindowMessage::Close => return window::close(self.window_id.unwrap()),
            NewItemPromptWindowMessage::Initialize(caller) => self.caller_window_id = Some(caller),
        }
        Task::none()
    }

    fn view_impl(&'_ self) -> Element<'_, NewItemPromptWindowMessage> {
        i18n_w("wip").into()
    }
}

impl Window for NewItemPromptWindow {
    fn new() -> Self
    where
        Self: Sized,
    {
        NewItemPromptWindow {
            window_id: None,
            caller_window_id: None,
        }
    }

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage> {
        if let WindowMessage::NewItemPromptWindowMessage(msg) = message {
            self.update_impl(msg)
                .map(WindowMessage::NewItemPromptWindowMessage)
        } else {
            Task::none()
        }
    }

    fn view(&'_ self) -> Element<'_, WindowMessage> {
        self.view_impl()
            .map(WindowMessage::NewItemPromptWindowMessage)
    }

    fn title(&'_ self) -> String {
        i18n("wip")
    }

    fn set_window_id(&mut self, window_id: window::Id) {
        self.window_id = Some(window_id)
    }

    fn window_settings() -> Settings
    where
        Self: Sized,
    {
        Settings {
            size: Size::new(400.0f32, 400.0f32),
            resizable: false,
            minimizable: false,
            ..app_default_window_settings()
        }
    }
}

use crate::app::window::resources::APP_ICON_PNG;
use crate::app::window::{Window, WindowMessage};
use crate::util::app_version_info;
use iced::alignment::Horizontal;
use iced::widget::image::Handle;
use iced::widget::{button, text, Container};
use iced::{window, Element, Length, Size, Task};
use prehnite_core::i18n::{i18n, i18n_w};

#[derive(Clone, Debug)]
pub enum VersionInfoWindowMessage {
    Close,
}

#[derive(Debug)]
pub struct VersionInfoWindow {
    window_id: Option<window::Id>,
}

impl VersionInfoWindow {
    fn update_impl(&mut self, message: VersionInfoWindowMessage) -> Task<VersionInfoWindowMessage> {
        match message {
            VersionInfoWindowMessage::Close => window::close(self.window_id.unwrap()),
        }
    }

    fn view_impl(&'_ self) -> Element<'_, VersionInfoWindowMessage> {
        Container::new(
            iced::widget::column![
                iced::widget::image(Handle::from_bytes(APP_ICON_PNG)),
                text(app_version_info()),
                Element::new(iced::widget::space().height(10)),
                button(i18n_w("close")).on_press(VersionInfoWindowMessage::Close)
            ]
            .align_x(Horizontal::Center),
        )
        .center(Length::Fill)
        .into()
    }
}

impl Window for VersionInfoWindow {
    fn new() -> (Box<dyn Window>, Task<WindowMessage>)
    where
        Self: Sized,
    {
        (
            Box::new(VersionInfoWindow { window_id: None }),
            Task::none(),
        )
    }

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage> {
        if let WindowMessage::AboutWindowMessage(msg) = message {
            self.update_impl(msg).map(WindowMessage::AboutWindowMessage)
        } else {
            Task::none()
        }
    }

    fn view(&'_ self) -> Element<'_, WindowMessage> {
        self.view_impl().map(WindowMessage::AboutWindowMessage)
    }

    fn title(&'_ self) -> String {
        i18n("version-info")
    }

    fn set_window_id(&mut self, window_id: window::Id) {
        self.window_id = Some(window_id)
    }

    fn default_resizable() -> bool
    where
        Self: Sized,
    {
        false
    }

    fn default_size() -> Size
    where
        Self: Sized,
    {
        Size::new(400.0f32, 400.0f32)
    }
}

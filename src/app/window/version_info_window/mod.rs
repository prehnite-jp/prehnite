use crate::app::resources::app_icon_handle;
use crate::app::window::{app_default_window_settings, Window, WindowMessage};
use crate::util::{app_build_features, app_build_profile, app_build_target, app_version_info};
use iced::alignment::Horizontal;
use iced::widget::text::Wrapping;
use iced::widget::{button, Container};
use iced::window::Settings;
use iced::{window, Element, Length, Size, Task};
use prehnite_core::i18n::{i18n, i18n_w};
use prehnite_core::widget::font::ftext;

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
                iced::widget::image(app_icon_handle()),
                iced::widget::scrollable(
                    Container::new(
                        ftext(format!(
                            "{}.{}.{}{}",
                            app_version_info(),
                            app_build_profile(),
                            app_build_target(),
                            if app_build_features().is_empty() {
                                "".to_string()
                            } else {
                                format!(".[{}]", app_build_features())
                            }
                        ))
                        .wrapping(Wrapping::None)
                    )
                    .padding(15)
                )
                .width(300)
                .horizontal(),
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
    fn new() -> Self
    where
        Self: Sized,
    {
        VersionInfoWindow { window_id: None }
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

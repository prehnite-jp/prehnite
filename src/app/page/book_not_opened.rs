use crate::app::window::main_window::BookOpenerMessage;
use iced::widget::{button, center};
use iced::{Alignment, Element};
use prehnite_core::i18n::i18n_w;

#[derive(Debug, Default, Clone)]
pub struct BookNotOpened;

impl BookNotOpened {
    pub fn view<'a>() -> Element<'a, BookOpenerMessage> {
        center(
            iced::widget::column![
                button(i18n_w("open-file")).on_press(BookOpenerMessage::Open),
                button(i18n_w("new-file")).on_press(BookOpenerMessage::New)
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        )
        .into()
    }
}

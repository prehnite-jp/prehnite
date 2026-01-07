use crate::util::book_opener::{BookOpener, BookOpenerMessage};
use iced::widget::{button, center, text};
use iced::{Alignment, Element, Task};
use prehnite_core::i18n::i18n;

#[derive(Clone, Debug)]
pub enum BookNotOpenedMessage {
    BookOpener(BookOpenerMessage),
}

#[derive(Debug)]
pub enum BookNotOpenedActions {
    None,
    BookOpener(Task<BookNotOpenedMessage>),
}

#[derive(Debug, Default)]
pub struct BookNotOpened;

impl BookNotOpened {
    pub fn update(&mut self, msg: BookNotOpenedMessage) -> BookNotOpenedActions {
        match msg {
            BookNotOpenedMessage::BookOpener(v) => {
                return BookNotOpenedActions::BookOpener(
                    BookOpener::update(v).map(BookNotOpenedMessage::BookOpener),
                );
            }
        }
        BookNotOpenedActions::None
    }

    pub fn view(&self) -> Element<BookNotOpenedMessage> {
        center(
            iced::widget::column![
                button(text(i18n("open-file"))).on_press(BookNotOpenedMessage::BookOpener(
                    BookOpenerMessage::OpenBook
                )),
                button(text(i18n("new-file")))
                    .on_press(BookNotOpenedMessage::BookOpener(BookOpenerMessage::NewBook))
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        )
        .into()
    }
}

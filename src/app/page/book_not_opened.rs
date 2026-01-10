use crate::util::book_opener::{BookOpener, BookOpenerMessage};
use iced::widget::{button, center, text};
use iced::{Alignment, Element, Task};
use prehnite_core::i18n::i18n;

#[derive(Clone, Debug)]
pub enum BookNotOpenedMessage {
    BookOpener(BookOpenerMessage),
}

impl From<BookOpenerMessage> for BookNotOpenedMessage {
    fn from(value: BookOpenerMessage) -> Self {
        BookNotOpenedMessage::BookOpener(value)
    }
}

#[derive(Debug)]
pub enum BookNotOpenedActions {
    BookOpener(Task<BookNotOpenedMessage>),
    BookOpened,
}

#[derive(Debug, Default)]
pub struct BookNotOpened;

impl BookNotOpened {
    pub fn update(&mut self, msg: BookNotOpenedMessage) -> BookNotOpenedActions {
        match msg {
            BookNotOpenedMessage::BookOpener(v) => match v {
                BookOpenerMessage::BookOpened => BookNotOpenedActions::BookOpened,
                _ => BookNotOpenedActions::BookOpener(
                    BookOpener::update(v).map(BookNotOpenedMessage::BookOpener),
                ),
            },
        }
    }

    pub fn view(&'_ self) -> Element<'_, BookNotOpenedMessage> {
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

mod background_info;
mod draft;
mod headline;
mod item_list;
mod paragraph;

use crate::util::book_opener::{BookOpener, BookOpenerMessage};
use iced::widget::{button, center, text};
use iced::{Element, Task};
use prehnite_core::i18n::i18n;

#[derive(Debug)]
pub struct PrehniteApp {}

#[derive(Clone, Debug)]
enum RootMessage {
    None,
    BookOpener(BookOpenerMessage),
}

impl PrehniteApp {
    pub fn run() -> Result<(), iced::Error> {
        iced::application(Self::new, Self::update, Self::view).run()
    }

    fn new() -> (Self, Task<RootMessage>) {
        (Self {}, Task::none())
    }

    #[tracing::instrument]
    fn update(&mut self, message: RootMessage) -> Task<RootMessage> {
        match message {
            RootMessage::None => {}
            RootMessage::BookOpener(v) => {
                return BookOpener::update(v).map(RootMessage::BookOpener);
            }
        }
        Task::none()
    }

    #[tracing::instrument]
    fn view(&'_ self) -> Element<'_, RootMessage> {
        center(iced::widget::column![
            button(text(i18n("open-file")))
                .on_press(RootMessage::BookOpener(BookOpenerMessage::OpenBook)),
            button(text(i18n("new-file")))
                .on_press(RootMessage::BookOpener(BookOpenerMessage::NewBook))
        ])
        .into()
    }
}

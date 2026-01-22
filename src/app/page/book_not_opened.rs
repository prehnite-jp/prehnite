use iced::widget::{button, center};
use iced::{window, Alignment, Element, Task};
use prehnite_core::i18n::i18n_w;
use prehnite_core::util::file_dialog::{select_and_open_prehnite_book_file, FileOpe};

impl Into<FileOpe> for BookNotOpenedMessage {
    fn into(self) -> FileOpe {
        match self {
            BookNotOpenedMessage::New => FileOpe::New,
            _ => FileOpe::Open,
        }
    }
}

#[derive(Clone, Debug)]
pub enum BookNotOpenedMessage {
    Open,
    New,
    Opened,
    NotOpened,
}

#[derive(Debug)]
pub enum BookNotOpenedActions {
    Run(Task<BookNotOpenedMessage>),
    Opened,
    NotOpened,
}

#[derive(Debug, Default, Clone)]
pub struct BookNotOpened;

impl BookNotOpened {
    fn open_or_new_file(window_id: window::Id, msg: BookNotOpenedMessage) -> BookNotOpenedActions {
        BookNotOpenedActions::Run(
            select_and_open_prehnite_book_file(window_id, msg.into()).map(|v| {
                if v.is_success() {
                    BookNotOpenedMessage::Opened
                } else {
                    BookNotOpenedMessage::NotOpened
                }
            }),
        )
    }

    pub fn update(
        &mut self,
        window_id: window::Id,
        msg: BookNotOpenedMessage,
    ) -> BookNotOpenedActions {
        match msg {
            BookNotOpenedMessage::Open | BookNotOpenedMessage::New => {
                Self::open_or_new_file(window_id, msg)
            }
            BookNotOpenedMessage::Opened => BookNotOpenedActions::Opened,
            BookNotOpenedMessage::NotOpened => BookNotOpenedActions::NotOpened,
        }
    }

    pub fn view(&'_ self) -> Element<'_, BookNotOpenedMessage> {
        center(
            iced::widget::column![
                button(i18n_w("open-file")).on_press(BookNotOpenedMessage::Open),
                button(i18n_w("new-file")).on_press(BookNotOpenedMessage::New)
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        )
        .into()
    }
}

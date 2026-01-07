use iced::Task;
use native_dialog::{MessageDialogBuilder, MessageLevel};
use prehnite_core::db::get_database;
use prehnite_core::i18n::i18n;
use prehnite_core::util::file_dialog::{dialog_new_prehnite_book, dialog_open_prehnite_book};
use std::path::PathBuf;
use tracing::{debug, error};

#[derive(Clone, Debug)]
pub enum BookOpe {
    New,
    Open,
}

#[derive(Clone, Debug)]
pub enum BookOpenerMessage {
    None,
    NewBook,
    OpenBook,
    BookSelected((BookOpe, Option<PathBuf>)),
    BookOpening((BookOpe, PathBuf)),
    BookOpened,
}

#[derive(Debug)]
pub struct BookOpener;

impl BookOpener {
    #[tracing::instrument]
    async fn book_selected((ope, book_path): (BookOpe, PathBuf)) -> BookOpenerMessage {
        match ope {
            BookOpe::New => match tokio::fs::remove_file(book_path.clone()).await {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to remove file ({book_path:#?}): {e}");
                    return BookOpenerMessage::None;
                }
            },
            BookOpe::Open => {}
        }
        match get_database().lock().await.open_book(book_path).await {
            Ok(_) => BookOpenerMessage::BookOpened,
            Err(e) => {
                error!("Failed to open the book. {}", e);
                MessageDialogBuilder::default()
                    .set_title(i18n("error"))
                    .set_text(i18n("book-open-error"))
                    .set_level(MessageLevel::Error);
                BookOpenerMessage::None
            }
        }
    }
    pub fn update(message: BookOpenerMessage) -> Task<BookOpenerMessage> {
        match message {
            BookOpenerMessage::None => {}

            BookOpenerMessage::BookSelected((ope, book_path)) => {
                return match book_path {
                    None => Task::none(),
                    Some(v) => Task::done(BookOpenerMessage::BookOpening((ope, v))),
                };
            }
            BookOpenerMessage::BookOpening(v) => {
                debug!("Opening the book: {:#?}", v);
                return Task::future(Self::book_selected(v));
            }
            BookOpenerMessage::BookOpened => {}
            BookOpenerMessage::NewBook => {
                return Task::future(dialog_new_prehnite_book())
                    .map(move |v| BookOpenerMessage::BookSelected((BookOpe::New, v)));
            }
            BookOpenerMessage::OpenBook => {
                return Task::future(dialog_open_prehnite_book())
                    .map(move |v| BookOpenerMessage::BookSelected((BookOpe::Open, v)));
            }
        }
        Task::none()
    }
}

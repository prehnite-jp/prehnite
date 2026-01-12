use iced::Task;
use prehnite_core::db::schema::Setting;
use prehnite_core::db::{acquire_err_handled, get_database, query, DBType};
use prehnite_core::opt_unwrap_or_return;
use prehnite_core::settings::SettingKey;
use prehnite_core::util::alert::{alert_i18n, alert_i18n_spawn};
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
                    if tokio::fs::try_exists(book_path.clone())
                        .await
                        .unwrap_or(true)
                    {
                        error!("Failed to remove file ({book_path:#?}): {e}");
                        alert_i18n_spawn(("error", "permission-denied")).await;
                        return BookOpenerMessage::None;
                    }
                }
            },
            BookOpe::Open => {
                if !tokio::fs::try_exists(book_path.clone())
                    .await
                    .unwrap_or(false)
                {
                    error!("File does not exist ({book_path:#?})");
                    alert_i18n_spawn(("error", "file-notfound")).await;
                    match Setting::restore(
                        opt_unwrap_or_return!(
                            acquire_err_handled(DBType::AppGlobal).await,
                            BookOpenerMessage::None
                        )
                        .as_mut(),
                        SettingKey::LastOpened,
                    )
                    .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            error!(
                                "Failed to update setting. [AppGlobal::LastOpened] Error: {e:#?}"
                            );
                        }
                    };
                    return BookOpenerMessage::None;
                }
            }
        };
        let result = get_database()
            .write()
            .await
            .open_book(book_path.clone())
            .await;
        match result {
            Ok(_) => {
                match query::update_setting(
                    opt_unwrap_or_return!(
                        acquire_err_handled(DBType::AppGlobal).await,
                        BookOpenerMessage::None
                    )
                    .as_mut(),
                    SettingKey::LastOpened,
                    book_path.to_str().map(|v| v.to_string()),
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Failed to update setting. [AppGlobal::LastOpened] Error: {e:#?}");
                    }
                };
                BookOpenerMessage::BookOpened
            }
            Err(e) => {
                error!("Failed to open the book. {}", e);
                alert_i18n(("error", "book-open-error"))
                    .spawn()
                    .await
                    .unwrap_or_else(|v| error!("Spawn alert error: Error: {v:#?}"));
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
            BookOpenerMessage::BookOpened => return Task::none(),
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

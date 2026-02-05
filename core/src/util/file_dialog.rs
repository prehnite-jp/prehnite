use crate::db::schema::Setting;
use crate::db::{acquire_err_handled, open_book_err_handled, DBType};
use crate::i18n::i18n;
use crate::opt_unwrap_or_return;
use crate::settings::SettingKey;
use crate::util::alert::alert_i18n_spawn;
use iced::window::raw_window_handle::HasWindowHandle;
use iced::{window, Task};
use native_dialog::{FileDialogBuilder, MessageLevel};
use std::path::PathBuf;
use tracing::{debug, error, trace};

fn prehnite_file_dialog_builder(
    title_i18n_id: &str,
    owner: &Option<&dyn HasWindowHandle>,
) -> FileDialogBuilder {
    let v = FileDialogBuilder::default()
        .set_title(i18n(title_i18n_id))
        .add_filter("prehnite book", ["prehnite"]);
    match owner {
        None => v,
        Some(w) => v.set_owner(w),
    }
}

fn unwrap_dialog_result(value: native_dialog::Result<Option<PathBuf>>) -> Option<PathBuf> {
    match value {
        Ok(v) => match v {
            None => {
                trace!("File select canceled.");
                None
            }
            Some(v) => Some(v),
        },
        Err(e) => {
            error!("Failed to get file path. {}", e);
            None
        }
    }
}

fn dialog_new_prehnite_book(window_id: window::Id) -> Task<Option<PathBuf>> {
    window::run(window_id, |w| {
        prehnite_file_dialog_builder("new-file", &Some(w)).save_single_file()
    })
    .then(|v| Task::future(async { unwrap_dialog_result(v.spawn().await) }))
}

fn dialog_open_prehnite_book(window_id: window::Id) -> Task<Option<PathBuf>> {
    window::run(window_id, |w| {
        prehnite_file_dialog_builder("open-file", &Some(w)).open_single_file()
    })
    .then(|v| Task::future(async { unwrap_dialog_result(v.spawn().await) }))
}

#[derive(Clone, Debug)]
pub enum FileOpe {
    New,
    Open,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpenPrehniteBookStatus {
    Success,
    Failed,
    NotSelected,
}

impl OpenPrehniteBookStatus {
    pub fn is_success(&self) -> bool {
        OpenPrehniteBookStatus::Success == *self
    }
}

async fn prehnite_book_file_process(book_path: PathBuf, ope: FileOpe) -> OpenPrehniteBookStatus {
    match ope {
        FileOpe::New => match tokio::fs::remove_file(book_path.clone()).await {
            Ok(_) => {}
            Err(e) => {
                if tokio::fs::try_exists(book_path.clone())
                    .await
                    .unwrap_or(true)
                {
                    error!("Failed to remove file ({book_path:#?}): {e}");
                    alert_i18n_spawn(("error", "permission-denied"), MessageLevel::Error).await;
                    return OpenPrehniteBookStatus::Failed;
                }
            }
        },
        FileOpe::Open => {
            if !tokio::fs::try_exists(book_path.clone())
                .await
                .unwrap_or(false)
            {
                error!("File does not exist ({book_path:#?})");
                alert_i18n_spawn(("error", "file-notfound"), MessageLevel::Error).await;
                Setting::restore(
                    opt_unwrap_or_return!(
                        acquire_err_handled(DBType::AppGlobal).await,
                        OpenPrehniteBookStatus::Failed
                    )
                    .as_mut(),
                    SettingKey::GLastOpened,
                )
                .await
                .unwrap_or_default();
                return OpenPrehniteBookStatus::Failed;
            }
        }
    }
    if open_book_err_handled(book_path).await {
        OpenPrehniteBookStatus::Success
    } else {
        OpenPrehniteBookStatus::Failed
    }
}

#[tracing::instrument]
pub fn select_and_open_prehnite_book_file(
    window_id: window::Id,
    ope: FileOpe,
) -> Task<OpenPrehniteBookStatus> {
    let file_dialog_result = match &ope {
        FileOpe::New => dialog_new_prehnite_book(window_id),
        FileOpe::Open => dialog_open_prehnite_book(window_id),
    };
    file_dialog_result.then(move |book_path| {
        let book_path =
            opt_unwrap_or_return!(book_path, Task::done(OpenPrehniteBookStatus::NotSelected));
        debug!("Opening the book: {:#?}", book_path);
        Task::future(prehnite_book_file_process(book_path, ope.clone()))
    })
}

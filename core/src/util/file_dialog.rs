#![doc = "Prehniteブックを開く"]

use crate::db::open_book_or_alert;
use crate::i18n::i18n;
use crate::settings::get_global;
use crate::util::alert::alert_i18n_spawn;
use crate::{opt_unwrap_or_return, settings};
use iced::window::raw_window_handle::HasWindowHandle;
use iced::{window, Task};
use native_dialog::{FileDialogBuilder, MessageLevel};
use std::path::PathBuf;
use tracing::{debug, error, trace};
use tracing_unwrap::ResultExt;

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
    match value.ok_or_log() {
        Some(v) => {
            if v.is_none() {
                trace!("File select canceled.");
            }
            v
        }
        None => None,
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
/// ファイルダイアログの種類
pub enum FileOpe {
    New,
    Open,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// ダイアログの結果
pub enum OpenPrehniteBookStatus {
    Success,
    Failed,
    NotSelected,
}

impl OpenPrehniteBookStatus {
    /// ファイルパスの取得に成功したかどうか
    pub fn is_success(&self) -> bool {
        OpenPrehniteBookStatus::Success == *self
    }
}

fn prehnite_book_file_process(
    book_path: PathBuf,
    ope: FileOpe,
) -> impl Future<Output = OpenPrehniteBookStatus> + Send {
    async move {
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
                    let x = settings::get_global();
                    if let Some(x) = x.write().ok_or_log().as_mut() {
                        x.get_tmp_registry().set_last_opened_file(None);
                    };

                    let registry = x.read().ok_or_log().map(|x| x.clone());
                    if let Some(reg) = registry {
                        reg.save().await.ok_or_log();
                    }
                    return OpenPrehniteBookStatus::Failed;
                }
            }
        }
        if open_book_or_alert(book_path) {
            let registry = get_global().read().ok_or_log().map(|x| x.clone());
            if let Some(reg) = registry {
                reg.save().await.ok_or_log();
            }
            OpenPrehniteBookStatus::Success
        } else {
            OpenPrehniteBookStatus::Failed
        }
    }
}

#[tracing::instrument]
/// [`Task`]として非同期にファイルダイアログを開きPrehniteブックを開きます。
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

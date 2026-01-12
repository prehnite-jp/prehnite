use crate::db::schema::Setting;
use crate::db::{acquire_err_handled, get_database, DBType};
use crate::i18n::i18n;
use crate::opt_unwrap_or_return;
use crate::settings::SettingKey;
use crate::util::alert::{alert_i18n_spawn};
use native_dialog::FileDialogBuilder;
use std::path::PathBuf;
use tracing::{debug, error, trace};

fn prehnite_file_dialog_builder(title_i18n_id: &str) -> FileDialogBuilder {
    FileDialogBuilder::default()
        .set_title(i18n(title_i18n_id))
        .add_filter("prehnite book", ["prehnite"])
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

async fn dialog_new_prehnite_book() -> Option<PathBuf> {
    unwrap_dialog_result(
        prehnite_file_dialog_builder("new-file")
            .save_single_file()
            .spawn()
            .await,
    )
}

async fn dialog_open_prehnite_book() -> Option<PathBuf> {
    unwrap_dialog_result(
        prehnite_file_dialog_builder("open-file")
            .open_single_file()
            .spawn()
            .await,
    )
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
        match self {
            OpenPrehniteBookStatus::Success => true,
            _ => false,
        }
    }
}

#[tracing::instrument]
pub async fn select_and_open_prehnite_book_file(ope: FileOpe) -> OpenPrehniteBookStatus {
    let book_path = opt_unwrap_or_return!(
        match &ope {
            FileOpe::New => dialog_new_prehnite_book().await,
            FileOpe::Open => dialog_open_prehnite_book().await,
        },
        OpenPrehniteBookStatus::NotSelected
    );
    debug!("Opening the book: {:#?}", book_path);
    match &ope {
        FileOpe::New => match tokio::fs::remove_file(book_path.clone()).await {
            Ok(_) => {}
            Err(e) => {
                if tokio::fs::try_exists(book_path.clone())
                    .await
                    .unwrap_or(true)
                {
                    error!("Failed to remove file ({book_path:#?}): {e}");
                    alert_i18n_spawn(("error", "permission-denied")).await;
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
                alert_i18n_spawn(("error", "file-notfound")).await;
                Setting::restore(
                    opt_unwrap_or_return!(
                        acquire_err_handled(DBType::AppGlobal).await,
                        OpenPrehniteBookStatus::Failed
                    )
                    .as_mut(),
                    SettingKey::LastOpened,
                )
                .await
                .unwrap_or_default();
                return OpenPrehniteBookStatus::Failed;
            }
        }
    }
    match get_database()
        .write()
        .await
        .open_book(book_path.clone())
        .await
    {
        Ok(_) => OpenPrehniteBookStatus::Success,
        Err(e) => {
            error!("Failed to open the book. {}", e);
            alert_i18n_spawn(("error", "book-open-error")).await;
            OpenPrehniteBookStatus::Failed
        }
    }
}

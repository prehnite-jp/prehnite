use crate::i18n::i18n;
use native_dialog::FileDialogBuilder;
use std::path::PathBuf;
use tracing::{error, trace};

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

pub async fn dialog_new_prehnite_book() -> Option<PathBuf> {
    unwrap_dialog_result(
        prehnite_file_dialog_builder("new-file")
            .save_single_file()
            .spawn()
            .await,
    )
}

pub async fn dialog_open_prehnite_book() -> Option<PathBuf> {
    unwrap_dialog_result(
        prehnite_file_dialog_builder("open-file")
            .open_single_file()
            .spawn()
            .await,
    )
}

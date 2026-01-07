use crate::i18n::i18n;
use native_dialog::FileDialogBuilder;
use std::path::PathBuf;
use tracing::error;

fn prehnite_file_dialog_builder() -> FileDialogBuilder {
    FileDialogBuilder::default()
        .set_title(i18n("open-file"))
        .add_filter("prehnite book", ["prehnite"])
}

fn unwrap_dialog_result(value: native_dialog::Result<Option<PathBuf>>) -> Option<PathBuf> {
    match value {
        Ok(v) => match v {
            None => {
                error!("Failed to get file path.");
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
    unwrap_dialog_result(prehnite_file_dialog_builder().save_single_file().spawn().await)
}

pub async fn dialog_open_prehnite_book() -> Option<PathBuf> {
    unwrap_dialog_result(prehnite_file_dialog_builder().open_single_file().spawn().await)
}

pub mod app_global;
pub mod file_dialog;

use crate::db::DatabaseError;
use native_dialog::{MessageDialogBuilder, MessageLevel};
use sys_locale::get_locale;
use unic_langid::LanguageIdentifier;
use crate::i18n::DEFAULT_LANG_ID;

const FATAL_JA: &str = "致命的なエラー";

const FATAL_EN: &str = "Fatal error";

const FATAL_INIT_DB_ERROR_MESSAGE_JA: &str = "アプリ設定用のデータベースが作成できません。
アプリ用ディレクトリが決定できませんでした。
PREHNITE_GLOBAL_DIR_PATH 環境変数を以下のように指定してください。
例(実行時のディレクトリに作成): PREHNITE_GLOBAL_DIR_PATH = \".\"";

const FATAL_INIT_DB_ERROR_MESSAGE_EN: &str = "Unable to create database for app settings.
Could not determine directory for app.
Specify the PREHNITE_GLOBAL_DIR_PATH environment variable as follows:
Example (created in the runtime directory): PREHNITE_GLOBAL_DIR_PATH = \".\"";

const FATAL_INIT_APP_ERROR_MESSAGE_JA: &str = "アプリケーションの初期化に失敗しました。";

const FATAL_INIT_APP_ERROR_MESSAGE_EN: &str = "Application initialization failed.";


fn lang_id() -> String {
    match get_locale()
        .unwrap_or(DEFAULT_LANG_ID.into())
        .parse::<LanguageIdentifier>()
    {
        Ok(v) => v.language.to_string(),
        Err(_) => "en".to_string(),
    }
}

fn alert(msg: (&str, &str)) {
    let (title, msg) = msg;
    MessageDialogBuilder::default()
        .set_level(MessageLevel::Error)
        .set_title(title)
        .set_text(msg)
        .alert()
        .show()
        .unwrap();
}

fn fatal_init_db_error_msg() -> (&'static str, &'static str) {
    match lang_id().as_str() {
        "ja" => (FATAL_JA, FATAL_INIT_DB_ERROR_MESSAGE_JA),
        &_ => (FATAL_EN, FATAL_INIT_DB_ERROR_MESSAGE_EN),
    }
}

pub fn fatal_init_db_error() {
    alert(fatal_init_db_error_msg())
}

pub fn fatal_initialize_app_error_msg() -> (&'static str, &'static str) {
    match lang_id().as_str() {
        "ja" => (FATAL_JA, FATAL_INIT_APP_ERROR_MESSAGE_JA),
        &_ => (FATAL_EN, FATAL_INIT_APP_ERROR_MESSAGE_EN),
    }
}

pub fn fatal_initialize_app_error_db(e: DatabaseError) {
    let (title, err_msg) = fatal_initialize_app_error_msg();
    alert((
        title,
        format!(
            "{err_msg}\ndetails:\n{}",
            match e {
                DatabaseError::DBError(v) => format!("Database Error: {:#?}", v),
                DatabaseError::MigrateError(v) => format!("Migration Error: {:#?}", v),
            }
        )
        .as_str(),
    ))
}

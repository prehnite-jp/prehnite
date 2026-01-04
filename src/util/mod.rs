use native_dialog::{MessageDialogBuilder, MessageLevel};
use sys_locale::get_locale;
use unic_langid::LanguageIdentifier;

const FATAL_JA: &str = "致命的なエラー";

const FATAL_EN: &str = "Fatal error";

const MESSAGE_JA: &str = "アプリ設定用のデータベースが作成できません。
アプリ用ディレクトリが決定できませんでした。
APP_GLOBAL_DATABASE_PATH 環境変数を以下のように指定してください。
例(実行時のディレクトリに作成): APP_GLOBAL_DATABASE_PATH = \"app_global.db\"";

const MESSAGE_EN: &str = "Unable to create database for app settings.
Could not determine directory for app.
Specify the APP_GLOBAL_DATABASE_PATH environment variable as follows:
Example (created in the runtime directory): APP_GLOBAL_DATABASE_PATH = \"app_global.db\"";

fn fatal_init_db_error_msg() -> (&'static str, &'static str) {
    let lang_id = match get_locale()
        .unwrap_or("en-US".into())
        .parse::<LanguageIdentifier>()
    {
        Ok(v) => v.language.to_string(),
        Err(_) => "en".to_string(),
    };
    match lang_id.as_str() {
        "ja" => (FATAL_JA, MESSAGE_JA),
        &_ => (FATAL_EN, MESSAGE_EN),
    }
}

pub fn fatal_init_db_error() {
    let (title, msg) = fatal_init_db_error_msg();
    MessageDialogBuilder::default()
        .set_level(MessageLevel::Error)
        .set_title(title)
        .set_text(msg)
        .alert();
}

#[macro_export]
macro_rules! fatal_init_db_error {
    () => {
        fatal_init_db_error();
        panic!();
    };
}
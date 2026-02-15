use crate::db::DatabaseError;
use crate::i18n::get_locale_language;
use iced::{window, Task};
use native_dialog::{MessageAlert, MessageLevel};
use std::fmt::Debug;
use tracing::error;

pub trait UnwrapOrErrorAlert<T> {
    fn unwrap_or_alert(self) -> T;
}

mod builder {
    use crate::i18n::i18n;
    use iced::wgpu::rwh::HasWindowHandle;
    use native_dialog::{MessageAlert, MessageConfirm, MessageDialogBuilder, MessageLevel};

    fn msg_dialog_builder(
        owner: &Option<&dyn HasWindowHandle>,
        (title, msg): (impl ToString, impl ToString),
        level: MessageLevel,
    ) -> MessageDialogBuilder {
        let v = MessageDialogBuilder::default()
            .set_level(level)
            .set_title(title)
            .set_text(msg);
        match owner {
            None => v,
            Some(w) => v.set_owner(w),
        }
    }

    fn msg_dialog_builder_i18n(
        owner: &Option<&dyn HasWindowHandle>,
        (title, msg): (&'static str, &'static str),
        level: MessageLevel,
    ) -> MessageDialogBuilder {
        msg_dialog_builder(owner, (i18n(title), i18n(msg)), level)
    }

    pub fn alert_(
        owner: &Option<&dyn HasWindowHandle>,
        content: (impl ToString, impl ToString),
        level: MessageLevel,
    ) -> MessageAlert {
        msg_dialog_builder(owner, content, level).alert()
    }

    pub fn alert_i18n_(
        owner: &Option<&dyn HasWindowHandle>,
        content: (&'static str, &'static str),
        level: MessageLevel,
    ) -> MessageAlert {
        msg_dialog_builder_i18n(owner, content, level).alert()
    }

    pub fn confirm_(
        owner: &Option<&dyn HasWindowHandle>,
        content: (impl ToString, impl ToString),
        level: MessageLevel,
    ) -> MessageConfirm {
        msg_dialog_builder(owner, content, level).confirm()
    }

    pub fn confirm_i18n_(
        owner: &Option<&dyn HasWindowHandle>,
        content: (&'static str, &'static str),
        level: MessageLevel,
    ) -> MessageConfirm {
        msg_dialog_builder_i18n(owner, content, level).confirm()
    }
}

trait DialogResult {
    fn result(self) -> bool;
}

impl DialogResult for () {
    fn result(self) -> bool {
        false
    }
}

impl DialogResult for bool {
    fn result(self) -> bool {
        self
    }
}

macro_rules! dialog_spawner {
    ($dialog_body:ident) => {
        Task::future(async {
            $dialog_body
                .spawn()
                .await
                .map(DialogResult::result)
                .unwrap_or_else(|e| {
                    error!("Spawn alert error: Error: {e:#?}");
                    false
                })
        })
    };
}

macro_rules! show_dialog {
    ($owner_window_id:ident, $content:ident, $level:ident, $builder_method:path) => {
        match $owner_window_id {
            None => {
                let v = $builder_method(&None, $content, $level);
                dialog_spawner!(v)
            }
            Some(owner_window_id) => window::run(owner_window_id, move |w| {
                $builder_method(&Some(w), $content, $level)
            })
            .then(|v| dialog_spawner!(v)),
        }
    };
}

#[tracing::instrument]
pub fn alert<T>(
    owner_window_id: Option<window::Id>,
    (title, msg): (impl ToString + Debug, impl ToString + Debug),
    level: MessageLevel,
) -> Task<T>
where
    T: 'static + Send,
{
    let content = (title.to_string(), msg.to_string());
    show_dialog!(owner_window_id, content, level, builder::alert_).discard()
}

#[tracing::instrument]
pub fn alert_i18n<T>(
    owner_window_id: Option<window::Id>,
    content: (&'static str, &'static str),
    level: MessageLevel,
) -> Task<T>
where
    T: 'static + Send,
{
    show_dialog!(owner_window_id, content, level, builder::alert_i18n_).discard()
}

#[tracing::instrument]
pub fn alert_show(
    (title, msg): (impl ToString + Debug, impl ToString + Debug),
    level: MessageLevel,
) {
    let content = (title.to_string(), msg.to_string());
    builder::alert_(&None, content, level)
        .show()
        .unwrap_or_else(|e| {
            error!("Show alert error: Error: {e:#?}");
        });
}

#[tracing::instrument]
pub async fn alert_spawn(
    (title, msg): (impl ToString + Debug, impl ToString + Debug),
    level: MessageLevel,
) {
    let content = (title.to_string(), msg.to_string());
    builder::alert_(&None, content, level)
        .spawn()
        .await
        .unwrap_or_else(|e| {
            error!("Spawn alert error: Error: {e:#?}");
        });
}

#[tracing::instrument]
pub fn alert_i18n_show(content: (&'static str, &'static str), level: MessageLevel) {
    builder::alert_i18n_(&None, content, level)
        .show()
        .unwrap_or_else(|e| {
            error!("Show alert error: Error: {e:#?}");
        });
}

#[tracing::instrument]
pub async fn alert_i18n_spawn(content: (&'static str, &'static str), level: MessageLevel) {
    builder::alert_i18n_(&None, content, level)
        .spawn()
        .await
        .unwrap_or_else(|e| {
            error!("Spawn alert error: Error: {e:#?}");
        });
}

#[tracing::instrument]
pub fn confirm(
    owner_window_id: Option<window::Id>,
    (title, msg): (impl ToString + Debug, impl ToString + Debug),
    level: MessageLevel,
) -> Task<bool> {
    let content = (title.to_string(), msg.to_string());
    show_dialog!(owner_window_id, content, level, builder::confirm_)
}

#[tracing::instrument]
pub fn confirm_i18n(
    owner_window_id: Option<window::Id>,
    content: (&'static str, &'static str),
    level: MessageLevel,
) -> Task<bool> {
    show_dialog!(owner_window_id, content, level, builder::confirm_i18n_)
}

#[tracing::instrument]
pub fn confirm_show(
    (title, msg): (impl ToString + Debug, impl ToString + Debug),
    level: MessageLevel,
) {
    let content = (title.to_string(), msg.to_string());
    builder::confirm_(&None, content, level)
        .show()
        .unwrap_or_else(|e| {
            error!("Show alert error: Error: {e:#?}");
            false
        });
}

#[tracing::instrument]
pub async fn confirm_spawn(
    (title, msg): (impl ToString + Debug, impl ToString + Debug),
    level: MessageLevel,
) {
    let content = (title.to_string(), msg.to_string());
    builder::confirm_(&None, content, level)
        .spawn()
        .await
        .unwrap_or_else(|e| {
            error!("Spawn alert error: Error: {e:#?}");
            false
        });
}

#[tracing::instrument]
pub fn confirm_i18n_show(content: (&'static str, &'static str), level: MessageLevel) {
    builder::confirm_i18n_(&None, content, level)
        .show()
        .unwrap_or_else(|e| {
            error!("Show alert error: Error: {e:#?}");
            false
        });
}

#[tracing::instrument]
pub async fn confirm_i18n_spawn(content: (&'static str, &'static str), level: MessageLevel) {
    builder::confirm_i18n_(&None, content, level)
        .spawn()
        .await
        .unwrap_or_else(|e| {
            error!("Spawn alert error: Error: {e:#?}");
            false
        });
}

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
pub fn fatal_init_db_error() -> MessageAlert {
    builder::alert_(
        &None,
        match get_locale_language().as_str() {
            "ja" => (FATAL_JA, FATAL_INIT_DB_ERROR_MESSAGE_JA),
            &_ => (FATAL_EN, FATAL_INIT_DB_ERROR_MESSAGE_EN),
        },
        MessageLevel::Error,
    )
}

const FATAL_INIT_APP_ERROR_MESSAGE_JA: &str = "アプリケーションの初期化に失敗しました。";
const FATAL_INIT_APP_ERROR_MESSAGE_EN: &str = "Application initialization failed.";
pub fn fatal_initialize_app_error_db(e: DatabaseError) -> MessageAlert {
    let (title, err_msg) = match get_locale_language().as_str() {
        "ja" => (FATAL_JA, FATAL_INIT_APP_ERROR_MESSAGE_JA),
        &_ => (FATAL_EN, FATAL_INIT_APP_ERROR_MESSAGE_EN),
    };
    builder::alert_(
        &None,
        (
            title,
            format!(
                "{err_msg}\ndetails:\n{}",
                match e {
                    DatabaseError::DBError(v) => format!("Database Error: {:#?}", v),
                    DatabaseError::MigrateError(v) => format!("Migration Error: {:#?}", v),
                }
            )
            .as_str(),
        ),
        MessageLevel::Error,
    )
}

const FATAL_INIT_SETTING_REGISTRY_ERROR_MESSAGE_JA: &str =
    "設定レジストリの読み込みに失敗しました。";
const FATAL_INIT_SETTING_REGISTRY_ERROR_MESSAGE_EN: &str = "Failed to load settings registry.";
pub fn fatal_initialize_setting_registry_error() -> MessageAlert {
    let (title, err_msg) = match get_locale_language().as_str() {
        "ja" => (FATAL_JA, FATAL_INIT_SETTING_REGISTRY_ERROR_MESSAGE_JA),
        &_ => (FATAL_EN, FATAL_INIT_SETTING_REGISTRY_ERROR_MESSAGE_EN),
    };
    builder::alert_(&None, (title, err_msg), MessageLevel::Error)
}

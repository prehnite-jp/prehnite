#![allow(unused)]
#![doc = "多言語対応"]
use crate::settings;
use crate::settings::GlobalSettings;
use crate::widget::font::ftext;
use easy_settings::sqlite::SettingManager;
use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::{FluentArgs, FluentError, FluentResource};
use iced::widget::Text;
use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use std::sync::{Arc, LazyLock, LockResult, RwLock, RwLockWriteGuard};
use strum::{Display, EnumString, IntoStaticStr, VariantArray};
use sys_locale::get_locale;
use thiserror::Error;
use tracing::{debug, error};
use tracing_unwrap::ResultExt;
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

/// 対応言語
pub const SUPPORTED_LANG_ID: &[&str] = &["en-US", "ja-JP"];

#[derive(
    Clone, EnumString, IntoStaticStr, VariantArray, Deserialize, Serialize, Display, Debug,
)]
pub enum SupportedLanguages {
    EnUS,
    JaJP,
}

/// デフォルトの言語
pub const DEFAULT_LANG_ID: SupportedLanguages = SupportedLanguages::EnUS;

static CURRENT_RESOURCE_BUNDLE: LazyLock<Arc<RwLock<CurrentI18nBundle>>> =
    LazyLock::new(|| Arc::new(RwLock::new(CurrentI18nBundle::new(None))));

/// ローカルの`lang_id`を取得します。
pub fn get_locale_lang_id() -> String {
    get_locale().unwrap_or(DEFAULT_LANG_ID.to_string())
}

/// ローカルの言語を取得します。
pub fn get_locale_language() -> String {
    if let Ok(v) = get_locale_lang_id().parse::<LanguageIdentifier>() {
        v.language.to_string()
    } else {
        "en".to_string()
    }
}

/// 現在読み込まれている言語バンドル
pub struct CurrentI18nBundle {
    bundle: Option<FluentBundle<FluentResource>>,
}

impl CurrentI18nBundle {
    fn new(bundle: Option<FluentBundle<FluentResource>>) -> Self {
        Self { bundle }
    }

    fn set_bundle(&mut self, bundle: Option<FluentBundle<FluentResource>>) {
        self.bundle = bundle;
    }

    /// 現在読み込まれている[`FluentBundle`]を取得する。
    pub fn get_bundle(&self) -> Option<&FluentBundle<FluentResource>> {
        self.bundle.as_ref()
    }
}

#[derive(Error, Debug)]
/// 多言語対応のエラー
pub enum I18nError {
    #[error("Invalid lang id received")]
    FailedToParseLangId(#[from] LanguageIdentifierError),
    #[error("Invalid ftl syntax")]
    FailedToParseFTL((FluentResource, Vec<fluent_syntax::parser::ParserError>)),
    #[error("Failed to add resource")]
    FailedToAddResource(Vec<FluentError>),
    #[error("Failed to execute statements")]
    DbError(#[from] sqlx::Error),
    #[error("Failed to apply settings")]
    FailedToApplySetting,
}

impl From<(FluentResource, Vec<fluent_syntax::parser::ParserError>)> for I18nError {
    fn from(value: (FluentResource, Vec<fluent_syntax::parser::ParserError>)) -> Self {
        I18nError::FailedToParseFTL(value)
    }
}

impl From<Vec<FluentError>> for I18nError {
    fn from(value: Vec<FluentError>) -> Self {
        I18nError::FailedToAddResource(value)
    }
}

#[derive(Error, Debug)]
enum TryGetFtlPathError {
    #[error("language resource not found")]
    LangNotFound,
}

fn try_get_ftl_str(lang_id: &str) -> Result<String, TryGetFtlPathError> {
    Ok(match lang_id {
        "ja-JP" | "ja" => include_str!("../../assets/locales/ja-JP.ftl"),
        "en-US" | "en" => include_str!("../../assets/locales/en-US.ftl"),
        _ => return Err(TryGetFtlPathError::LangNotFound),
    }
    .to_string())
}

fn get_ftl_str(lang_id: &str) -> String {
    try_get_ftl_str(lang_id).unwrap_or_else(|_| {
        try_get_ftl_str(DEFAULT_LANG_ID.into()).expect_or_log("Default locale not found.")
    })
}

fn parse_lang_bundle(lang_id: &str) -> Result<FluentBundle<FluentResource>, I18nError> {
    let language_identifier: LanguageIdentifier = lang_id.parse()?;
    let resource = FluentResource::try_new(get_ftl_str(lang_id))?;

    let mut bundle = FluentBundle::new_concurrent(vec![language_identifier]);
    bundle.add_resource(resource)?;
    bundle.set_use_isolating(false);
    Ok(bundle)
}

/// 言語バンドルを差し替えます。
pub fn change_lang_bundle(
    arg_lang_id_str: &str,
) -> impl Future<Output = Result<(), I18nError>> {
    let bundle = CURRENT_RESOURCE_BUNDLE.clone();
    let settings = settings::get_global();
    async move {
        let (lang_id, lang_id_str): (LanguageIdentifier, &str) = match arg_lang_id_str.parse() {
            Ok(v) => (v, arg_lang_id_str),
            Err(e) => {
                debug!("Parse failed!! set default lang_id ...");
                debug!("Error: {e:#?}");
                (
                    (Into::<&str>::into(DEFAULT_LANG_ID)).parse()?,
                    DEFAULT_LANG_ID.into(),
                )
            }
        };
        if bundle
            .read()
            .as_mut()
            .unwrap_or_log()
            .get_bundle()
            .is_none_or(|v| !v.locales.contains(&lang_id))
        {
            bundle
                .write()
                .as_mut()
                .unwrap_or_log()
                .set_bundle(Some(parse_lang_bundle(lang_id_str)?));
            match settings.write().ok_or_log().as_mut() {
                Some(mut x) => {
                    x.get_tmp_registry()
                        .set_locale(lang_id.to_string().parse().ok_or_log());
                    if x.save_and_apply().await.ok_or_log().is_some() {
                        return Ok(());
                    }
                }
                None => {
                    panic!()
                }
            }

            return Err(I18nError::FailedToApplySetting);
        }
        Ok(())
    }
}

pub(crate) async fn change_lang_bundle_with_conn(
    conn: &mut SqliteConnection,
    arg_lang_id_str: &str,
) -> Result<(), I18nError> {
    let (lang_id, lang_id_str): (LanguageIdentifier, &str) = match arg_lang_id_str.parse() {
        Ok(v) => (v, arg_lang_id_str),
        Err(e) => {
            debug!("Parse failed!! set default lang_id ...");
            debug!("Error: {e:#?}");
            (
                <&str>::from(DEFAULT_LANG_ID).parse()?,
                DEFAULT_LANG_ID.into(),
            )
        }
    };
    if get_lang_bundle()
        .read()
        .unwrap_or_log()
        .get_bundle()
        .is_none_or(|v| !v.locales.contains(&lang_id))
    {
        get_lang_bundle()
            .write()
            .unwrap_or_log()
            .set_bundle(Some(parse_lang_bundle(lang_id_str)?));
        if !match settings::get_global().write().ok_or_log().as_mut() {
            None => false,
            Some(x) => {
                x.get_tmp_registry()
                    .set_locale(lang_id.to_string().parse().ok_or_log());
                x.save_and_apply().await.ok_or_log().is_some()
            }
        } {
            return Err(I18nError::FailedToApplySetting);
        };
    }
    Ok(())
}

/// 現在の言語バンドルを取得します。
#[inline]
pub fn get_lang_bundle() -> Arc<RwLock<CurrentI18nBundle>> {
    CURRENT_RESOURCE_BUNDLE.clone()
}

/// i18nキーから表示内容を取得します。
pub fn i18n(id: &str) -> String {
    i18n_fmt(id, None)
}

/// i18nキーから表示内容を[`Text`]として取得します。
pub fn i18n_w(id: &str) -> Text<'_> {
    ftext(i18n_fmt(id, None))
}

/// i18nキーとフォーマットを使用し表示内容を[`Text`]として取得します。
pub fn i18n_fmt_w<'a>(id: &str, args: Option<&FluentArgs<'_>>) -> Text<'a> {
    ftext(i18n_fmt(id, args))
}

#[tracing::instrument]
/// i18nキーとフォーマットを使用し表示内容を取得します。
pub fn i18n_fmt(id: &str, args: Option<&FluentArgs<'_>>) -> String {
    #[derive(Debug)]
    enum Error {
        DoesNotBeInitialized,
        MessageDoesNotExists,
        FailedToFetchMessage,
    }
    fn func<'a>(id: &str, args: Option<&FluentArgs<'_>>) -> Result<String, Error> {
        let mut errors = vec![];
        Ok(get_lang_bundle()
            .read()
            .unwrap_or_log()
            .get_bundle()
            .ok_or(Error::DoesNotBeInitialized)?
            .format_pattern(
                get_lang_bundle()
                    .read()
                    .unwrap_or_log()
                    .get_bundle()
                    .ok_or(Error::DoesNotBeInitialized)?
                    .get_message(id)
                    .ok_or(Error::MessageDoesNotExists)?
                    .value()
                    .ok_or(Error::FailedToFetchMessage)?,
                args,
                &mut errors,
            )
            .to_string())
    }
    func(id, args).unwrap_or_else(|e| {
        match e {
            Error::DoesNotBeInitialized => error!("i18n does not be initialized."),
            Error::MessageDoesNotExists => error!("Message {id} does not exist."),
            Error::FailedToFetchMessage => error!("Unable to get value for message {id}."),
        }
        id.to_string()
    })
}

/// i18nを初期化します。
pub async fn initialize_i18n_from_settings() -> Result<(), sqlx::Error> {
    let lang_id = settings::get_global()
        .read()
        .ok_or_log()
        .map(|x| x.get_registry().get_locale().to_string());
    change_lang_bundle(lang_id.unwrap_or(get_locale_lang_id()).as_str())
        .await
        .expect_or_log("lang_id not found.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::i18n::{
        change_lang_bundle_with_conn, get_lang_bundle, parse_lang_bundle, try_get_ftl_str,
        SUPPORTED_LANG_ID,
    };
    use sqlx::SqlitePool;

    #[test]
    fn valid_check_get_for_all_supported_languages() {
        for i in SUPPORTED_LANG_ID {
            try_get_ftl_str(i).unwrap();
        }
    }

    #[sqlx::test(migrator = "crate::db::migrate::app_global::MIGRATOR")]
    async fn valid_check_ftl(pool: SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        for i in SUPPORTED_LANG_ID {
            change_lang_bundle_with_conn(&mut *conn, i)
                .await
                .expect(format!("Failed to parse ftl: {}", i).as_str());
            assert!(
                get_lang_bundle()
                    .read()
                    .expect("Failed to read lock lang bundle.")
                    .get_bundle()
                    .expect("Failed to get lang bundle.")
                    .locales
                    .contains(&i.parse().unwrap())
            )
        }
    }

    #[test]
    #[should_panic]
    fn invalid_check_unsupported() {
        try_get_ftl_str("AYgAV6Lky").unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_lang_id() {
        parse_lang_bundle("AYgAV6Lky").unwrap();
    }
}

use crate::db::query;
use crate::settings::{GlobalSettingKey, SettingKey};
use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::{FluentArgs, FluentError, FluentResource};
use sqlx::SqliteConnection;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, LazyLock, PoisonError, RwLock, RwLockReadGuard};
use sys_locale::get_locale;
use tracing::{debug, error};
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

pub const SUPPORTED_LANG_ID: &[&str] = &["en-US", "ja-JP"];

pub const DEFAULT_LANG_ID: &str = "en-US";

static CURRENT_RESOURCE_BUNDLE: LazyLock<Arc<RwLock<CurrentI18nBundle>>> =
    LazyLock::new(|| Arc::new(RwLock::new(CurrentI18nBundle::new(None))));

pub fn get_locale_lang_id() -> String {
    get_locale().unwrap_or(DEFAULT_LANG_ID.into())
}

pub fn get_locale_language() -> String {
    if let Ok(v) = get_locale_lang_id().parse::<LanguageIdentifier>() {
        v.language.to_string()
    } else {
        "en".to_string()
    }
}

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

    pub fn get_bundle(&self) -> Option<&FluentBundle<FluentResource>> {
        self.bundle.as_ref()
    }
}

#[derive(Debug)]
pub enum I18nError {
    FailedToParseLangId(LanguageIdentifierError),
    FailedToParseFTL((FluentResource, Vec<fluent_syntax::parser::ParserError>)),
    FailedToAddResource(Vec<FluentError>),
    DbError(sqlx::Error),
    FailedToLockLangBundle,
}

impl Display for I18nError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "i18n: Failed to load language. \n\tI18nError = {:#?}",
            self
        )
    }
}

impl Error for I18nError {}

impl From<LanguageIdentifierError> for I18nError {
    fn from(value: LanguageIdentifierError) -> Self {
        I18nError::FailedToParseLangId(value)
    }
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

impl From<sqlx::Error> for I18nError {
    fn from(value: sqlx::Error) -> Self {
        I18nError::DbError(value)
    }
}

#[derive(Debug)]
enum TryGetFtlPathError {
    LangNotFound,
}

impl Display for TryGetFtlPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "i18n: Failed to get ftl path. \n\tTryGetFtlPathError = {:#?}",
            self
        )
    }
}

impl Error for TryGetFtlPathError {}

fn try_get_ftl_str(lang_id: &str) -> Result<String, TryGetFtlPathError> {
    Ok(match lang_id {
        "ja-JP" | "ja" => include_str!("../assets/locales/ja-JP.ftl"),
        "en-US" | "en" => include_str!("../assets/locales/en-US.ftl"),
        _ => return Err(TryGetFtlPathError::LangNotFound),
    }
    .to_string())
}

fn get_ftl_str(lang_id: &str) -> String {
    try_get_ftl_str(lang_id)
        .unwrap_or_else(|_| try_get_ftl_str("en").expect("Default locale not found."))
}

fn parse_lang_bundle(lang_id: &str) -> Result<FluentBundle<FluentResource>, I18nError> {
    let language_identifier: LanguageIdentifier = lang_id.parse()?;
    let resource = FluentResource::try_new(get_ftl_str(lang_id))?;

    let mut bundle = FluentBundle::new_concurrent(vec![language_identifier]);
    bundle.add_resource(resource)?;
    bundle.set_use_isolating(false);
    Ok(bundle)
}

macro_rules! error_failed_to_lock_lang_bundle {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to read lock lang bundle. {:#?}", e);
                return Err(I18nError::FailedToLockLangBundle);
            }
        }
    };
}

#[tracing::instrument]
pub async fn change_lang_bundle(
    conn: &mut SqliteConnection,
    arg_lang_id_str: &str,
) -> Result<(), I18nError> {
    let (lang_id, lang_id_str): (LanguageIdentifier, &str) = match arg_lang_id_str.parse() {
        Ok(v) => (v, arg_lang_id_str),
        Err(e) => {
            debug!("Parse failed!! set default lang_id ...");
            debug!("Error: {e:#?}");
            (DEFAULT_LANG_ID.parse()?, DEFAULT_LANG_ID)
        }
    };
    if error_failed_to_lock_lang_bundle!(CURRENT_RESOURCE_BUNDLE.read())
        .get_bundle()
        .is_none_or(|v| !v.locales.contains(&lang_id))
    {
        error_failed_to_lock_lang_bundle!(CURRENT_RESOURCE_BUNDLE.write())
            .set_bundle(Some(parse_lang_bundle(lang_id_str)?));
        query::update_setting(conn, SettingKey::Global(GlobalSettingKey::Locale), Some(lang_id.to_string())).await?
    }
    Ok(())
}

pub fn get_lang_bundle() -> Arc<RwLock<CurrentI18nBundle>> {
    CURRENT_RESOURCE_BUNDLE.clone()
}

pub fn i18n(id: &str) -> String {
    i18n_fmt(id, None)
}

pub fn i18n_w(id: &str) -> iced::widget::Text<'_> {
    iced::widget::text(i18n_fmt(id, None))
}

pub fn i18n_fmt_w<'a>(id: &str, args: Option<&FluentArgs<'_>>) -> iced::widget::Text<'a> {
    iced::widget::text(i18n_fmt(id, args))
}

#[tracing::instrument]
pub fn i18n_fmt(id: &str, args: Option<&FluentArgs<'_>>) -> String {
    #[derive(Debug)]
    enum Error {
        ReadLockLangBundleError(String),
        DoesNotBeInitialized,
        MessageDoesNotExists,
        FailedToFetchMessage,
    }
    impl From<PoisonError<RwLockReadGuard<'_, CurrentI18nBundle>>> for Error {
        fn from(value: PoisonError<RwLockReadGuard<'_, CurrentI18nBundle>>) -> Self {
            Error::ReadLockLangBundleError(format!("{:#?}", value))
        }
    }
    fn func<'a>(id: &str, args: Option<&FluentArgs<'_>>) -> Result<String, Error> {
        let mut errors = vec![];
        Ok(get_lang_bundle()
            .read()?
            .get_bundle()
            .ok_or(Error::DoesNotBeInitialized)?
            .format_pattern(
                get_lang_bundle()
                    .read()?
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
            Error::ReadLockLangBundleError(e) => error!("Failed to read lock lang bundle. {e}"),
            Error::DoesNotBeInitialized => error!("i18n does not be initialized."),
            Error::MessageDoesNotExists => error!("Message {id} does not exist."),
            Error::FailedToFetchMessage => error!("Unable to get value for message {id}."),
        }
        id.to_string()
    })
}

pub async fn initialize_i18n_from_db(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    let lang_id = query::fetch_setting(conn, SettingKey::Global(GlobalSettingKey::Locale))
        .await?
        .map(|v| v.setting_value)
        .unwrap_or(None);
    change_lang_bundle(conn, lang_id.unwrap_or(get_locale_lang_id()).as_str())
        .await
        .expect("lang_id not found.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::i18n::{
        change_lang_bundle, get_lang_bundle, parse_lang_bundle, try_get_ftl_str, SUPPORTED_LANG_ID,
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
            change_lang_bundle(&mut conn, i)
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

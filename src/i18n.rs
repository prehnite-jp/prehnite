use crate::db::schema::Setting;
use crate::settings::SettingKey;
use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::{FluentArgs, FluentError, FluentResource};
use sqlx::SqliteConnection;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, LazyLock, RwLock};
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

pub const SUPPORTED_LANG_ID: &[&str] = &["en-US", "ja-JP"];

pub const DEFAULT_LANG_ID: &str = "en-US";

static CURRENT_RESOURCE_BUNDLE: LazyLock<Arc<RwLock<CurrentI18nBundle>>> =
    LazyLock::new(|| Arc::new(RwLock::new(CurrentI18nBundle::new(None))));

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
    FailedToParseFTL,
    FailedToAddResource(Vec<FluentError>),
    DbError(sqlx::Error),
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

fn parse_language_identifier(lang_id: &str) -> Result<LanguageIdentifier, I18nError> {
    match lang_id.parse() {
        Ok(v) => Ok(v),
        Err(e) => Err(I18nError::FailedToParseLangId(e)),
    }
}

fn parse_lang_bundle(lang_id: &str) -> Result<FluentBundle<FluentResource>, I18nError> {
    let language_identifier: LanguageIdentifier = parse_language_identifier(lang_id)?;
    let resource = match FluentResource::try_new(get_ftl_str(lang_id)) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{:#?}", e);
            return Err(I18nError::FailedToParseFTL);
        }
    };

    let mut bundle = FluentBundle::new_concurrent(vec![language_identifier]);
    match bundle.add_resource(resource) {
        Ok(_) => {}
        Err(e) => return Err(I18nError::FailedToAddResource(e)),
    }
    Ok(bundle)
}

pub async fn change_lang_bundle(
    conn: &mut SqliteConnection,
    lang_id_str: &str,
) -> Result<(), I18nError> {
    let lang_id = parse_language_identifier(lang_id_str)
        .unwrap_or(parse_language_identifier(DEFAULT_LANG_ID)?);
    if match CURRENT_RESOURCE_BUNDLE
        .read()
        .expect("Failed to read lock lang bundle.")
        .get_bundle()
    {
        None => true,
        Some(v) => !v.locales.contains(&lang_id),
    } {
        CURRENT_RESOURCE_BUNDLE
            .write()
            .expect("Failed to write lock lang bundle.")
            .set_bundle(Some(parse_lang_bundle(lang_id_str)?));
        match Setting::update_setting(conn, SettingKey::Locale, Some(lang_id.to_string())).await {
            Ok(_) => {}
            Err(e) => return Err(I18nError::DbError(e)),
        }
    }
    Ok(())
}

pub fn get_lang_bundle() -> Arc<RwLock<CurrentI18nBundle>> {
    CURRENT_RESOURCE_BUNDLE.clone()
}

pub fn i18n(id: &str) -> String {
    i18n_fmt(id, None)
}

// TODO: エラーハンドリング
pub fn i18n_fmt(id: &str, args: Option<&FluentArgs<'_>>) -> String {
    let mut errors = vec![];
    get_lang_bundle()
        .read()
        .expect("Failed to read lock lang bundle.")
        .get_bundle()
        .expect("Failed to get lang bundle.")
        .format_pattern(
            get_lang_bundle()
                .read()
                .expect("Failed to read lock lang bundle.")
                .get_bundle()
                .expect("Failed to get lang bundle.")
                .get_message(id)
                .expect(format!("Failed to get message. id: {}", id).as_str())
                .value()
                .expect("Failed to get message value"),
            args,
            &mut errors,
        )
        .to_string()
}

pub async fn initialize_i18n_locale_from_db(
    conn: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    let lang_id = Setting::fetch_setting(conn, SettingKey::Locale).await?;
    change_lang_bundle(
        conn,
        lang_id.unwrap_or(DEFAULT_LANG_ID.to_string()).as_str(),
    )
    .await
    .expect("lang_id not found.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::i18n::{
        SUPPORTED_LANG_ID, change_lang_bundle, get_lang_bundle, parse_lang_bundle, try_get_ftl_str,
    };
    use sqlx::SqlitePool;

    use crate::i18n::parse_language_identifier;

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
                    .contains(&parse_language_identifier(i).unwrap())
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

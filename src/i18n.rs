use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::{FluentError, FluentResource};
use std::error::Error;
use std::fmt::{Display, Formatter};
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

pub const SUPPORTED_LANG_ID: &[&str] = &["en-US", "ja-JP"];

#[derive(Debug)]
pub enum I18nError {
    FailedToParseLangId(LanguageIdentifierError),
    FailedToParseFTL,
    FailedToAddResource(Vec<FluentError>),
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

pub fn parse_lang_bundle(lang_id: &str) -> Result<FluentBundle<FluentResource>, I18nError> {
    let language_identifier: LanguageIdentifier = match lang_id.parse() {
        Ok(v) => v,
        Err(e) => return Err(I18nError::FailedToParseLangId(e)),
    };
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

#[cfg(test)]
mod tests {
    use crate::i18n::{parse_lang_bundle, try_get_ftl_str, SUPPORTED_LANG_ID};

    #[test]
    fn valid_check_get_for_all_supported_languages() {
        for i in SUPPORTED_LANG_ID {
            try_get_ftl_str(i).unwrap();
        }
    }

    #[test]
    fn valid_check_ftl() {
        for i in SUPPORTED_LANG_ID {
            parse_lang_bundle(i).expect(format!("Failed to parse ftl: {}", i).as_str());
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

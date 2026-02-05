use crate::db::query;
use crate::db::schema::Setting;
use sqlx::SqliteConnection;
use std::fmt::Display;

// G: global, B: book
const G_KEY_LOCALE: &str = "locale";
const G_KEY_LAST_OPENED: &str = "last-opened-file";

#[derive(Clone, Debug)]
pub enum SettingKey {
    GLocale,
    GLastOpened,
}

impl Display for SettingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SettingKey::GLocale => G_KEY_LOCALE.to_string(),
                SettingKey::GLastOpened => G_KEY_LAST_OPENED.to_string(),
            }
        )
    }
}

impl SettingKey {
    fn default_setting_value(&self) -> Option<String> {
        match self {
            SettingKey::GLocale => sys_locale::get_locale(),
            SettingKey::GLastOpened => None,
        }
    }
}

impl Setting {
    pub async fn restore(conn: &mut SqliteConnection, key: SettingKey) -> Result<(), sqlx::Error> {
        query::update_setting(conn, key.clone(), key.default_setting_value()).await?;
        Ok(())
    }

    pub async fn retore_all(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
        Self::restore(conn, SettingKey::GLocale).await?;
        Self::restore(conn, SettingKey::GLastOpened).await?;
        Ok(())
    }
}

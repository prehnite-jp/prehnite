use crate::db::schema::Setting;
use sqlx::SqliteConnection;
use std::fmt::Display;
use crate::db::query::{SETTING_FETCH_SQL, SETTING_UPDATE_SQL};

const KEY_LOCALE: &str = "locale";
const KEY_LAST_OPENED: &str = "last-opened-file";

#[derive(Clone)]
pub enum SettingKey {
    Locale,
    LastOpened,
}

impl Display for SettingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SettingKey::Locale => KEY_LOCALE.to_string(),
                SettingKey::LastOpened => KEY_LAST_OPENED.to_string(),
            }
        )
    }
}

impl SettingKey {
    fn default_setting_value(&self) -> Option<String> {
        match self {
            SettingKey::Locale => sys_locale::get_locale(),
            SettingKey::LastOpened => None,
        }
    }
}

impl Setting {
    pub async fn restore(conn: &mut SqliteConnection, key: SettingKey) -> Result<(), sqlx::Error> {
        Self::update_setting(conn, key.clone(), key.default_setting_value()).await?;
        Ok(())
    }

    pub async fn retore_all(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
        Self::restore(conn, SettingKey::Locale).await?;
        Self::restore(conn, SettingKey::LastOpened).await?;
        Ok(())
    }

    pub async fn update_setting(
        conn: &mut SqliteConnection,
        key: SettingKey,
        value: Option<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(SETTING_UPDATE_SQL)
            .bind(key.to_string())
            .bind(value)
            .execute(conn)
            .await?;
        Ok(())
    }

    pub async fn fetch_setting(
        conn: &mut SqliteConnection,
        key: SettingKey,
    ) -> Result<Option<String>, sqlx::Error> {
        Ok(
            match sqlx::query_as::<_, Setting>(SETTING_FETCH_SQL)
                .bind(key.to_string())
                .fetch_optional(conn)
                .await?
            {
                None => key.default_setting_value(),
                Some(v) => v.setting_value,
            },
        )
    }
}

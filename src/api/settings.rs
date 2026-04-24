use crate::application::settings::GlobalSettings;
use dioxus::prelude::{get, put};
use easy_settings::Registry;
use prehnite_core::db::schema::Setting;
use std::sync::LazyLock;
use tokio::sync::RwLock;

#[cfg(feature = "server")]
static CACHED_REGISTRY: LazyLock<RwLock<GlobalSettings>> =
    LazyLock::new(|| RwLock::new(Default::default()));

#[get("/api/global/settings")]
pub async fn fetch_all_settings() -> anyhow::Result<GlobalSettings> {
    let mut conn = crate::backend::db::acquire_global().await?;
    let mut result = GlobalSettings::default();
    result.set_from_row_vec(
        Setting::select_all(&mut *conn)
            .await?
            .into_iter()
            .map(|x| x.to_setting_row())
            .collect(),
    );
    *CACHED_REGISTRY.write().await = result.clone();
    Ok(result)
}

#[put("/api/global/settings")]
pub async fn save_all_settings(settings: GlobalSettings) -> anyhow::Result<()> {
    use sqlx::Acquire;
    let mut conn = crate::backend::db::acquire_global().await?;
    let mut tx = conn.begin().await?;
    {
        let cached = CACHED_REGISTRY.read().await;
        for (key, val) in settings
            .items()
            .iter()
            .filter(|x| cached.get(x.0).unwrap() != x.1)
        {
            sqlx::query("INSERT INTO settings(setting_key, setting_value) VALUES (?1, ?2) ON CONFLICT DO UPDATE SET setting_value = ?2")
                .bind(key.to_string())
                .bind(val.raw_string())
                .execute(&mut *tx)
                .await?;
        }
    }
    tx.commit().await?;
    *CACHED_REGISTRY.write().await = settings;
    Ok(())
}

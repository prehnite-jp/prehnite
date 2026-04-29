use crate::app::db::acquire_global;
use easy_settings::Registry;
use prehnite_core::db::schema::Setting;

pub trait Fetch: Registry {
    async fn fetch() -> anyhow::Result<Self> {
        let mut conn = acquire_global().await?;
        let mut result = Self::default();
        result.set_from_row_vec(Setting::select_all(&mut *conn).await?);
        Ok(result)
    }
}

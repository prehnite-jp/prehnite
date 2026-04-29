use crate::app::db::acquire_global;
use crate::app::settings::fetch::Fetch;
use easy_settings::Registry;

#[derive(PartialEq, Clone)]
pub struct GlobalRegistry<T>
where
    T: Registry + Fetch + PartialEq,
{
    registry: T,
    version: u64,
}

impl<T> GlobalRegistry<T>
where
    T: Registry + Fetch + PartialEq,
{
    pub(super) fn new() -> Self {
        Self {
            registry: T::default(),
            version: 0,
        }
    }

    pub fn set_applied(self, registry: T) -> Self {
        Self {
            registry,
            version: self.version.wrapping_add(1),
        }
    }

    pub async fn load(self) -> anyhow::Result<Self> {
        Ok(self.set_applied(T::fetch().await?))
    }

    pub async fn save(self, registry: T) -> anyhow::Result<Self> {
        use sqlx::Acquire;
        let mut conn = acquire_global().await?;
        let mut tx = conn.begin().await?;
        {
            for (key, val) in registry
                .items()
                .iter()
                .filter(|x| self.registry.get(x.0).unwrap() != x.1)
            {
                sqlx::query("INSERT INTO settings(setting_key, setting_value) VALUES (?1, ?2) ON CONFLICT DO UPDATE SET setting_value = ?2")
                .bind(key.to_string())
                .bind(val.raw_string())
                .execute(&mut *tx)
                .await?;
            }
        }
        tx.commit().await?;
        Ok(self.load().await?)
    }

    pub fn registry(&self) -> &T {
        &self.registry
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn is_require_refresh(&self, loaded_version: u64) -> bool {
        self.version != loaded_version
    }
}

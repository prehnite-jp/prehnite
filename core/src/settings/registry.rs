use crate::db::schema::Setting;
use crate::db::DBType;
use crate::settings::value::{SettingValue, SettingValueType};
use crate::settings::{
    BookSettingKey, GlobalSettingKey, SettingCategory, SettingEntry, SettingKey,
};
use sqlx::Acquire;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};
use tracing::error;

fn registry() -> SettingRegistry {
    SettingRegistry::default().add_category(
        SettingCategory::new("settings.category.general")
            .add(
                SettingEntry::new(
                    GlobalSettingKey::Locale.into(),
                    sys_locale::get_locale().into(),
                )
                .display_key("settings.entry.locale"),
            )
            .add(SettingEntry::new(
                GlobalSettingKey::Font.into(),
                Option::<bool>::None.into(),
            ))
            .add(
                SettingEntry::new(
                    GlobalSettingKey::LastOpened.into(),
                    Option::<String>::None.into(),
                )
                .visibility(false),
            )
            .add(
                SettingEntry::new(GlobalSettingKey::AutoOpenLastOpened.into(), true.into())
                    .display_key("settings.entry.auto-open-last-opened-file"),
            ),
    )
}

#[derive(Default, Debug)]
pub struct SettingRegistry {
    pub categories: Vec<SettingCategory>,
    pub entries: HashMap<SettingKey, SettingEntry>,
    pub values: HashMap<SettingKey, SettingValue>,
}

static REGISTRY: LazyLock<Arc<RwLock<SettingRegistry>>> =
    LazyLock::new(|| Arc::new(RwLock::new(registry())));

impl SettingRegistry {
    fn add_category(mut self, category: SettingCategory) -> Self {
        category.entries.iter().for_each(|e| {
            self.values.insert(
                e.setting_key.clone(),
                SettingValue::new(e.default_value.clone()),
            );
        });
        self.categories.push(category);
        if let Some(c) = self.categories.last() {
            c.entries.iter().for_each(|e| {
                self.entries.insert(e.setting_key, e.clone());
            });
        }
        self
    }

    #[tracing::instrument]
    async fn impl_load(&mut self, target: DBType) -> bool {
        let mut conn = match crate::db::acquire_err_handled(target).await {
            None => {
                return false;
            }
            Some(conn) => conn,
        };
        match Setting::select_all(&mut *conn).await {
            Ok(v) => {
                for x in v.into_iter() {
                    let raw_key = x.setting_key;
                    if let Ok(key) = match target {
                        DBType::AppGlobal => {
                            GlobalSettingKey::try_from(raw_key.as_str()).map(|v| v.into())
                        }
                        DBType::PrehniteBook => {
                            BookSettingKey::try_from(raw_key.as_str()).map(|v| v.into())
                        }
                    } {
                        self.values
                            .insert(key, SettingValue::new(x.setting_value.into()));
                    }
                }
                true
            }
            Err(e) => {
                error!("Failed to fetch settings. E: {e:?}");
                false
            }
        }
    }

    async fn impl_save(&mut self, target: DBType) -> bool {
        let mut conn = match crate::db::acquire_err_handled(target).await {
            None => {
                return false;
            }
            Some(conn) => conn,
        };
        let mut tx = match conn.begin().await {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to begin transaction. E: {e:?}");
                return false;
            }
        };
        for entry in self
            .entries
            .values()
            .filter(|v| match v.setting_key {
                SettingKey::Global(_) => target == DBType::AppGlobal,
                SettingKey::Book(_) => target == DBType::PrehniteBook,
            })
            .into_iter()
        {
            // 保存
            if !self
                .values
                .get_mut(&entry.setting_key)
                .expect("Failed to get value")
                .apply(&mut *tx, entry.setting_key)
                .await
            {
                if let Err(e) = tx.rollback().await {
                    error!("Rollback failed!! E: {e:?}")
                }
                return false;
            }
        }
        if let Err(e) = tx.commit().await {
            error!("Commit failed!! E: {e:?}");
            return false;
        }
        true
    }

    #[tracing::instrument]
    fn impl_get_applied(&self, key: &SettingKey) -> Option<SettingValueType> {
        Some(self.values.get(key)?.applied.clone())
    }

    #[tracing::instrument]
    fn impl_set_value(&mut self, key: &SettingKey, value: SettingValueType) -> bool {
        self.values
            .get_mut(key)
            .and_then(|v| {
                v.temporary = value;
                Some(())
            })
            .is_some()
    }

    fn impl_restore_default(&mut self, key: SettingKey) {
        let default = self
            .entries
            .get(&key)
            .expect("Failed to get setting entry.")
            .default_value
            .clone();
        match self.values.get_mut(&key) {
            None => {
                self.values.insert(key, SettingValue::new(default));
            }
            Some(v) => v.set(default),
        };
    }
}

impl SettingRegistry {
    #[tracing::instrument]
    pub fn get_applied(key: &SettingKey) -> Option<SettingValueType> {
        Self::get_registry()
            .read()
            .map(|v| v.impl_get_applied(key))
            .inspect_err(|e| error!("Failed to lock setting registry. E: {:?}", e))
            .unwrap_or_default()
    }

    #[tracing::instrument]
    pub fn set_value(key: &SettingKey, value: SettingValueType) -> bool {
        Self::get_registry()
            .write()
            .map(|mut v| v.impl_set_value(key, value))
            .inspect_err(|e| error!("Failed to lock setting registry. E: {:?}", e))
            .unwrap_or_default()
    }

    pub fn get_registry() -> Arc<RwLock<SettingRegistry>> {
        REGISTRY.clone()
    }

    #[tracing::instrument]
    pub async fn load(target: DBType) -> bool {
        match Self::get_registry().write() {
            Ok(mut v) => v.impl_load(target).await,
            Err(e) => {
                error!("Failed to lock setting registry. E: {:?}", e);
                false
            }
        }
    }

    #[tracing::instrument]
    pub async fn save(target: DBType) -> bool {
        match Self::get_registry().write() {
            Ok(mut v) => v.impl_save(target).await,
            Err(e) => {
                error!("Failed to lock setting registry. E: {:?}", e);
                false
            }
        }
    }

    #[tracing::instrument]
    pub fn restore_default(key: SettingKey) -> bool {
        match Self::get_registry().write() {
            Ok(mut v) => v.impl_restore_default(key),
            Err(e) => {
                error!("Failed to lock setting registry. E: {:?}", e);
                return false;
            }
        };
        true
    }
}

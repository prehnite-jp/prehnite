use crate::db::schema::Setting;
use crate::db::{acquire_err_handled, DBType};
use crate::i18n::{DEFAULT_LANG_ID, SUPPORTED_LANG_ID};
use crate::opt_unwrap_or_return;
use crate::settings::value::SettingValueType;
use crate::settings::{
    BookSettingKey, GlobalSettingKey, SettingCategory, SettingEntry, SettingKey,
};
use sqlx::{Acquire, SqliteConnection};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};
use tracing_unwrap::{OptionExt, ResultExt};

fn fallback_locale() -> String {
    sys_locale::get_locale()
        .and_then(|v| {
            if SUPPORTED_LANG_ID.contains(&v.as_str()) {
                Some(v)
            } else {
                None
            }
        })
        .unwrap_or(DEFAULT_LANG_ID.to_string())
}

fn registry() -> SettingRegistry {
    SettingRegistry::default().add_category(
        SettingCategory::new("settings_category_general")
            .add(
                SettingEntry::new(GlobalSettingKey::Locale.into(), fallback_locale().into())
                    .display_key("settings_entry_locale")
                    .selectable_values(SUPPORTED_LANG_ID),
            )
            .add(
                SettingEntry::new(GlobalSettingKey::Font.into(), Option::<String>::None.into())
                    .selectable_values(&[])
                    .display_key("settings_entry_font"),
            )
            .add(
                SettingEntry::new(
                    GlobalSettingKey::LastOpened.into(),
                    Option::<String>::None.into(),
                )
                .visibility(false),
            )
            .add(
                SettingEntry::new(GlobalSettingKey::AutoOpenLastOpened.into(), true.into())
                    .display_key("settings_entry_auto-open-last-opened-file"),
            ),
    )
}

#[derive(Default, Debug)]
pub struct SettingRegistry {
    pub categories: Vec<SettingCategory>,
    pub entries: HashMap<SettingKey, SettingEntry>,
    values: RwLock<HashMap<SettingKey, SettingValueType>>,
}

static REGISTRY: LazyLock<Arc<SettingRegistry>> = LazyLock::new(|| Arc::new(registry()));

impl SettingRegistry {
    fn add_category(mut self, category: SettingCategory) -> Self {
        category.entries.iter().for_each(|e| {
            self.values
                .write()
                .unwrap_or_log()
                .insert(e.setting_key.clone(), e.default_value.clone());
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
    pub fn get(key: &SettingKey) -> Option<SettingValueType> {
        Some(REGISTRY.values.read().unwrap_or_log().get(key)?.clone())
    }

    #[tracing::instrument]
    pub async fn load(target: DBType) -> bool {
        let values = opt_unwrap_or_return!(
            async {
                let mut values: HashMap<SettingKey, SettingValueType> = HashMap::new();
                let mut conn = match acquire_err_handled(target).await {
                    None => {
                        return None;
                    }
                    Some(conn) => conn,
                };
                match Setting::select_all(&mut *conn).await.ok_or_log() {
                    Some(v) => {
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
                                values.insert(
                                    key,
                                    match REGISTRY.entries.get(&key) {
                                        None => SettingValueType::String(x.setting_value),
                                        Some(v) => v.default_value.converter(x.setting_value),
                                    },
                                );
                            }
                        }
                        Some(values)
                    }
                    None => None,
                }
            }
            .await,
            false
        );
        match REGISTRY.values.write().ok_or_log() {
            Some(mut v) => {
                v.extend(values);
                true
            }
            None => false,
        }
    }

    #[tracing::instrument]
    pub async fn save(target: DBType) -> bool {
        let mut conn = match acquire_err_handled(target).await {
            None => return false,
            Some(conn) => conn,
        };
        let mut tx = match conn.begin().await.ok_or_log() {
            Some(v) => v,
            None => return false,
        };
        for entry in REGISTRY
            .entries
            .values()
            .filter(|v| match v.setting_key {
                SettingKey::Global(_) => target == DBType::AppGlobal,
                SettingKey::Book(_) => target == DBType::PrehniteBook,
            })
            .into_iter()
        {
            // 保存
            if Self::save_by_key_with_conn(&mut *tx, entry.setting_key)
                .await
                .is_err()
            {
                tx.rollback().await.ok_or_log();
                return false;
            }
        }
        tx.commit().await.ok_or_log() != None
    }

    #[tracing::instrument]
    pub async fn save_by_key(key: SettingKey) -> sqlx::Result<()> {
        Self::save_by_key_with_conn(
            &mut *match key.get_conn().await {
                None => return Ok(()),
                Some(v) => v,
            },
            key,
        )
        .await
    }

    pub async fn save_by_key_with_conn(
        conn: &mut SqliteConnection,
        key: SettingKey,
    ) -> sqlx::Result<()> {
        let v = REGISTRY
            .values
            .read()
            .unwrap_or_log()
            .get(&key)
            .expect_or_log(&format!("setting value missing!! Key: `{}`", key))
            .clone();
        v.save(&mut *conn, key).await
    }

    #[tracing::instrument]
    pub fn get_default(key: SettingKey) -> Option<SettingValueType> {
        REGISTRY.entries.get(&key).map(|v| v.default_value.clone())
    }

    pub async fn immediate_apply(key: SettingKey, value: SettingValueType) -> sqlx::Result<()> {
        match REGISTRY.values.write().unwrap_or_log().get_mut(&key) {
            None => {}
            Some(v) => *v = value,
        };
        Self::save_by_key(key).await
    }

    pub async fn immediate_apply_with_conn(
        conn: &mut SqliteConnection,
        key: SettingKey,
        value: SettingValueType,
    ) -> sqlx::Result<()> {
        match REGISTRY.values.write().unwrap_or_log().get_mut(&key) {
            None => {}
            Some(v) => *v = value,
        };
        Self::save_by_key_with_conn(conn, key).await
    }

    pub fn get_values() -> HashMap<SettingKey, SettingValueType> {
        REGISTRY.values.read().unwrap().clone()
    }

    pub fn get_categories() -> &'static Vec<SettingCategory> {
        &REGISTRY.categories
    }

    pub fn get_entries() -> &'static HashMap<SettingKey, SettingEntry> {
        &REGISTRY.entries
    }
}

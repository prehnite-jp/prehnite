pub mod registry;
pub mod value;

use crate::db::{acquire_err_handled, DBType};
use log::error;
use sqlx::pool::PoolConnection;
use sqlx::Sqlite;
use std::fmt::{Display, Formatter};

macro_rules! key_impl {
    ($x:ty) => {
        impl $x {
            fn as_str(&self) -> &'static str {
                ""
            }
        }

        impl TryFrom<&str> for $x {
            type Error = ();

            fn try_from(_value: &str) -> Result<Self, Self::Error> {
                Err(())
            }
        }
    };
    ($x:ty, $(($v:path, $key:ident)),*) => {
        impl $x {
            fn as_str(&self) -> &'static str {
                match self {
                    $(
                    $v => $key,
                    )*
                }
            }
        }

        impl TryFrom<&str> for $x {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    $(
                    $key => Ok($v),
                    )*
                    _=> Err(())
                }
            }
        }
    }
}

// G: global
const G_KEY_LOCALE: &str = "locale";
const G_KEY_FONT: &str = "font";
const G_KEY_AUTO_OPEN_LAST_OPENED: &str = "auto-open-last-opened-file";
const G_KEY_LAST_OPENED: &str = "last-opened-file";

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy, Hash)]
pub enum GlobalSettingKey {
    Locale,
    Font,
    LastOpened,
    AutoOpenLastOpened,
}

key_impl!(
    GlobalSettingKey,
    (GlobalSettingKey::Locale, G_KEY_LOCALE),
    (GlobalSettingKey::Font, G_KEY_FONT),
    (GlobalSettingKey::LastOpened, G_KEY_LAST_OPENED),
    (
        GlobalSettingKey::AutoOpenLastOpened,
        G_KEY_AUTO_OPEN_LAST_OPENED
    )
);

// B: book

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy, Hash)]
pub enum BookSettingKey {}

key_impl!(BookSettingKey);

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy, Hash)]
pub enum SettingKey {
    Global(GlobalSettingKey),
    Book(BookSettingKey),
}

impl Into<DBType> for SettingKey {
    fn into(self) -> DBType {
        match self {
            SettingKey::Global(_) => DBType::AppGlobal,
            SettingKey::Book(_) => DBType::PrehniteBook,
        }
    }
}

impl SettingKey {
    pub async fn get_conn(self) -> Option<PoolConnection<Sqlite>> {
        acquire_err_handled(self.into()).await
    }
}

impl From<GlobalSettingKey> for SettingKey {
    fn from(value: GlobalSettingKey) -> Self {
        Self::Global(value)
    }
}

impl From<BookSettingKey> for SettingKey {
    fn from(value: BookSettingKey) -> Self {
        Self::Book(value)
    }
}

impl SettingKey {
    fn as_str(&self) -> &'static str {
        match self {
            SettingKey::Global(g_key) => g_key.as_str(),
            SettingKey::Book(b_key) => b_key.as_str(),
        }
    }
}

impl Display for SettingKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

use crate::i18n::i18n;
use crate::settings::value::SettingValueType;

#[derive(Default, Debug, Clone)]
pub struct SettingCategory {
    category_name_i18n_key: &'static str,
    entries: Vec<SettingEntry>,
}

impl SettingCategory {
    pub fn category_name(&self) -> String {
        i18n(self.category_name_i18n_key)
    }

    pub fn entries(&self) -> &'_ Vec<SettingEntry> {
        &self.entries
    }

    fn new(category_name_i18n_key: &'static str) -> Self {
        Self {
            category_name_i18n_key,
            ..Default::default()
        }
    }

    fn add(mut self, entry: SettingEntry) -> Self {
        self.entries.push(entry);
        self
    }
}

#[derive(Debug, Clone)]
pub struct SettingEntry {
    setting_key: SettingKey,
    display_key: &'static str,
    // この値によって設定値の型が決定されます。
    default_value: SettingValueType,
    is_visible: bool,
    // 選択可能な値リスト
    selectable_values: Option<Vec<String>>,
}

impl From<SettingEntry> for String {
    fn from(value: SettingEntry) -> Self {
        value.get_display()
    }
}

impl Display for SettingEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_display())
    }
}

impl SettingEntry {
    fn new(setting_key: SettingKey, default_value: SettingValueType) -> Self {
        SettingEntry {
            setting_key,
            display_key: setting_key.as_str(),
            default_value,
            is_visible: true,
            selectable_values: None,
        }
    }

    pub fn get_setting_key(&self) -> SettingKey {
        self.setting_key
    }

    pub fn get_display(&self) -> String {
        i18n(self.display_key)
    }

    pub fn get_is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn get_selectable_values(&self) -> Option<Vec<String>> {
        self.selectable_values.clone()
    }

    pub fn default_value(&self) -> &SettingValueType {
        &self.default_value
    }

    fn visibility(mut self, is_visible: bool) -> Self {
        self.is_visible = is_visible;
        self
    }

    fn display_key(mut self, display_key: &'static str) -> Self {
        self.display_key = display_key;
        self
    }

    #[tracing::instrument]
    fn selectable_values(mut self, values: &'static [&str]) -> Self {
        if let SettingValueType::String(_) = self.default_value {
            self.selectable_values = Some(values.iter().map(|v| v.to_string()).collect());
        } else {
            error!("Selectable Values Not Allowed!!");
            panic!("Selectable Values Not Allowed!!")
        }
        self
    }
}

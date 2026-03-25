#![doc = "アプリの設定関連"]
pub mod registry;
pub mod value;

use crate::db::{acquire_or_log, DBType};
use log::error;
use sqlx::pool::PoolConnection;
use sqlx::Sqlite;
use std::fmt::{Display, Formatter};
use strum::{EnumString, IntoStaticStr};

// G: global
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy, Hash, EnumString, IntoStaticStr)]
/// グローバル設定のキー
pub enum GlobalSettingKey {
    #[strum(serialize = "locale")]
    /// 言語と地域
    Locale,
    #[strum(serialize = "font")]
    /// フォント
    Font,
    #[strum(serialize = "last-opened-file")]
    /// 最後に開いたファイルのパス
    LastOpened,
    #[strum(serialize = "auto-open-last-opened-file")]
    /// 最後に開いたファイルを自動で開く
    AutoOpenLastOpened,
}

// B: book

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy, Hash, EnumString, IntoStaticStr)]
/// ブック設定のキー
pub enum BookSettingKey {
    #[strum(serialize = "locked")]
    /// 使用されません
    Todo,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy, Hash)]
/// アプリケーション全体の設定キー
pub enum SettingKey {
    /// グローバル設定
    Global(GlobalSettingKey),
    /// ブック設定
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
    /// 設定キーからdbコネクションを取得します。
    pub async fn get_conn(self) -> Option<PoolConnection<Sqlite>> {
        acquire_or_log(self.into()).await
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
            SettingKey::Global(g_key) => g_key.into(),
            SettingKey::Book(b_key) => b_key.into(),
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
/// 設定のカテゴリ
pub struct SettingCategory {
    category_name_i18n_key: &'static str,
    entries: Vec<SettingEntry>,
}

impl SettingCategory {
    /// カテゴリ名を取得します。
    #[inline]
    pub fn category_name(&self) -> String {
        i18n(self.category_name_i18n_key)
    }

    /// 設定項目のリストを取得します。
    #[inline]
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
/// 設定項目
pub struct SettingEntry {
    setting_key: SettingKey,
    display_key: &'static str,
    /// 初期値
    ///
    /// この値によって設定値の型が決定されます。
    default_value: SettingValueType,
    /// 設定画面での可視性
    is_visible: bool,
    /// 選択可能な値リスト
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

    /// 設定キーを取得する。
    #[inline]
    pub fn get_setting_key(&self) -> SettingKey {
        self.setting_key
    }

    /// 設定名を取得する。
    #[inline]
    pub fn get_display(&self) -> String {
        i18n(self.display_key)
    }

    //noinspection RsUnnecessaryReturn
    /// 設定画面での可視性を取得する。
    #[inline]
    pub fn get_is_visible(&self) -> bool {
        #[cfg(feature = "debug")]
        return true;
        #[cfg(not(feature = "debug"))]
        return self.is_visible;
    }

    /// コンボボックス用の値リストを取得する。
    #[inline]
    pub fn get_selectable_values(&self) -> Option<Vec<String>> {
        self.selectable_values.clone()
    }

    /// 初期値を取得する。
    #[inline]
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

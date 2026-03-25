#![doc = "設定の項目値"]
use crate::db::query;
use crate::settings::SettingKey;
use sqlx::SqliteConnection;

#[derive(Clone, Debug)]
/// 設定の項目値を型付きで表現します。
pub enum SettingValueType {
    Bool(Option<bool>),
    Int(Option<i64>),
    Float(Option<f64>),
    String(Option<String>),
}

impl From<SettingValueType> for Option<bool> {
    fn from(value: SettingValueType) -> Self {
        value.bool()
    }
}

impl From<SettingValueType> for Option<i64> {
    fn from(value: SettingValueType) -> Self {
        value.int()
    }
}

impl From<SettingValueType> for Option<String> {
    fn from(value: SettingValueType) -> Self {
        value.string()
    }
}

impl From<SettingValueType> for Option<f64> {
    fn from(value: SettingValueType) -> Self {
        value.float()
    }
}

macro_rules! setting_value_type_to_string(
    ($v:ident) => {
        $v.map(|v| v.to_string())
    }
);

macro_rules! auto_impl_from_non_option(
    ($($t:ty),*) => {
        $(
        impl From<$t> for SettingValueType {
            fn from(value: $t) -> Self {
                Some(value).into()
            }
        }
        )*
    }
);

auto_impl_from_non_option!(bool, i64, i32, i16, i8, f64, f32, String, &str);

macro_rules! auto_impl_from_option {
    ($(($t:ty, $req_t:ty, $p:path)),*) => {
        $(impl From<Option<$t>> for SettingValueType {
            fn from(value: Option<$t>) -> Self {
                $p(value.map(|v| v as $req_t))
            }
        })*
    };
    ($(($t:ty, $p:path)),*)=>{
        auto_impl_from_option!($(($t, $t, $p)),*);
    };
}

auto_impl_from_option!(
    (bool, SettingValueType::Bool),
    (i64, SettingValueType::Int),
    (f64, SettingValueType::Float),
    (String, SettingValueType::String)
);

auto_impl_from_option!(
    (i32, i64, SettingValueType::Int),
    (i16, i64, SettingValueType::Int),
    (i8, i64, SettingValueType::Int),
    (f32, f64, SettingValueType::Float)
);

impl From<Option<&str>> for SettingValueType {
    fn from(value: Option<&str>) -> Self {
        value.map(|v| v.to_string()).into()
    }
}

impl SettingValueType {
    /// 文字列をパースし、設定項目の値に変更します。
    pub fn converter(&self, value: Option<String>) -> Self {
        match self {
            SettingValueType::Bool(_) => value.and_then(|v| v.parse::<bool>().ok()).into(),
            SettingValueType::Int(_) => value.and_then(|v| v.parse::<i64>().ok()).into(),
            SettingValueType::Float(_) => value.and_then(|v| v.parse::<f64>().ok()).into(),
            SettingValueType::String(_) => value.into(),
        }
    }

    /// 設定項目の値を設定します。
    pub fn set(mut self, v: SettingValueType) {
        self = v;
    }

    #[tracing::instrument]
    /// 設定項目を保存します。
    pub async fn save(
        &self,
        conn: &mut SqliteConnection,
        setting_key: SettingKey,
    ) -> sqlx::Result<()> {
        query::update_setting(conn, setting_key, self.clone().to_opt_string()).await
    }

    /// 設定値を取得します。
    pub fn get<T>(self) -> Option<T>
    where
        Option<T>: From<SettingValueType>,
    {
        Option::<T>::from(self)
    }

    /// 設定値を文字列として取得します。
    pub fn to_opt_string(self) -> Option<String> {
        match self {
            SettingValueType::Bool(v) => setting_value_type_to_string!(v),
            SettingValueType::Int(v) => setting_value_type_to_string!(v),
            SettingValueType::Float(v) => setting_value_type_to_string!(v),
            SettingValueType::String(v) => v,
        }
    }

    /// [`bool`]の設定値を取得します。[`bool`]以外の場合は[`None`]を返します。
    fn bool(self) -> Option<bool> {
        if let Self::Bool(v) = self { v } else { None }
    }

    /// [`i64`]の設定値を取得します。[`i64`]以外の場合は[`None`]を返します。
    fn int(self) -> Option<i64> {
        if let Self::Int(v) = self { v } else { None }
    }

    /// [`String`]の設定値を取得します。[`String`]以外の場合は[`None`]を返します。
    fn string(self) -> Option<String> {
        if let Self::String(v) = self { v } else { None }
    }

    /// [`f64`]の設定値を取得します。[`f64`]以外の場合は[`None`]を返します。
    fn float(self) -> Option<f64> {
        if let Self::Float(v) = self { v } else { None }
    }
}

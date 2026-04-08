#![doc="ユーティリティ"]
pub mod alert;
pub mod app_global;
pub mod file_dialog;

#[allow(unused)]
#[macro_export]
/// オプションから値を取り出します。Noneの場合、`return $ret_val` します。
macro_rules! opt_unwrap_or_return {
    ($value:expr, $ret_val:expr) => {
        match $value {
            Some(v) => v,
            None => return $ret_val,
        }
    };
}

#[allow(unused)]
#[macro_export]
/// オプションから値を取り出します。Noneの場合、`continue` します。
macro_rules! opt_unwrap_or_continue {
    ($value: expr) => {
        match $value {
            Some(v) => v,
            None => continue,
        }
    };
}

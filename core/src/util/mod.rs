pub mod app_global;
pub mod file_dialog;
pub mod alert;

#[allow(unused)]
#[macro_export]
macro_rules! opt_unwrap_or_return {
    ($value:expr, $ret_val:expr) => {
        match $value {
            Some(v) => v,
            None => return $ret_val
        }
    };
}

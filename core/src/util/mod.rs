pub mod app_global;
pub mod file_dialog;
pub mod alert;

#[allow(unused)]
#[macro_export]
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
macro_rules! opt_unwrap_or_continue {
    ($value: expr) => {
        match $value {
            Some(v) => v,
            None => continue
        }
    };
}

#[allow(unused)]
#[macro_export]
macro_rules! on_error_logging {
    ($result:ident) => {
        if $result.is_err() {
            let e = $result.err().unwrap();
            error!("Error: {e:#?}");
            return Err(From::from(e));
        }
    };
}

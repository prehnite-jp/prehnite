use dioxus::desktop::tao::rwh_06::HasWindowHandle;
use dioxus::desktop::window;
use native_dialog::{FileDialogBuilder, MessageDialogBuilder, MessageLevel};
use std::fmt;
use std::fmt::Debug;
use tracing_unwrap::{OptionExt, ResultExt};

pub fn message_dialog_builder() -> MessageDialogBuilder {
    match window().window_handle().ok_or_log().as_ref() {
        None => MessageDialogBuilder::default(),
        Some(x) => MessageDialogBuilder::default().set_owner(x),
    }
}

pub fn file_dialog_builder() -> FileDialogBuilder {
    match window().window_handle().ok_or_log().as_ref() {
        None => FileDialogBuilder::default(),
        Some(x) => FileDialogBuilder::default().set_owner(x),
    }
}

pub trait AlertResult<T, E> {
    fn unwrap_or_alert(self) -> T
    where
        E: fmt::Debug;

    fn ok_or_alert(self) -> Option<T>
    where
        E: fmt::Debug;
}

impl<T, E> AlertResult<T, E> for Result<T, E> {
    fn unwrap_or_alert(self) -> T
    where
        E: Debug,
    {
        self.ok_or_alert().unwrap()
    }

    fn ok_or_alert(self) -> Option<T>
    where
        E: Debug,
    {
        if let Err(err) = self.as_ref() {
            message_dialog_builder()
                .set_level(MessageLevel::Error)
                .set_title("Error")
                .set_text(format!("{:?}", err))
                .alert()
                .show()
                .ok_or_log();
        }
        self.ok_or_log()
    }
}

pub trait AlertOption<T> {
    fn unwrap_or_alert(self) -> T;
}

impl<T> AlertOption<T> for Option<T> {
    fn unwrap_or_alert(self) -> T {
        if let None = self.as_ref() {
            message_dialog_builder()
                .set_level(MessageLevel::Error)
                .set_title("Error")
                .set_text("Failed to unwrap option. because option is None.")
                .alert()
                .show()
                .ok_or_log();
        }
        self.unwrap_or_log()
    }
}

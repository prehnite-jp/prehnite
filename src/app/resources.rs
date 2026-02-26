use iced::widget::image::Handle;
use std::sync::OnceLock;

pub const APP_ICON_PNG: &[u8] = include_bytes!("../../assets/icon/icon.png");

pub fn app_icon_handle() -> Handle {
    static HANDLE: OnceLock<Handle> = OnceLock::new();
    HANDLE
        .get_or_init(|| Handle::from_bytes(APP_ICON_PNG))
        .clone()
}

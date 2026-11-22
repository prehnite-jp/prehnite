mod logger;

use gpui_kit::*;
use crate::logger::initialize_logger;

rust_i18n::i18n!("locales", fallback = "en");
fn init_i18n() {
    use gpui_kit::component as gpui_component;
    rust_i18n::extend!(gpui_component);
}

fn main() {
    initialize_logger().expect("failed to initialize logger.");
    application().run(|cx| {
        init_i18n();
        init(cx);
        // ここにウインドウを記述する。
    });
}

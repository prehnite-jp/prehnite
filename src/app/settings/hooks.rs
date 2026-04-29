use crate::app::settings::apply::{apply_language_from_settings, apply_theme};
use crate::app::settings::get_global_settings;
use crate::window::main_window::menu;
use dioxus::hooks::{use_effect, use_future, Effect, UseFuture};
use dioxus::prelude::*;
use dioxus_desktop::window;
use std::time::Duration;

static LOADED_VERSION_GLOBAL_SETTINGS: GlobalSignal<u64> = GlobalSignal::new(|| 0);

pub fn use_setting_updator() -> UseFuture {
    use_future(move || async move {
        loop {
            if get_global_settings().is_require_refresh(LOADED_VERSION_GLOBAL_SETTINGS()) {
                *LOADED_VERSION_GLOBAL_SETTINGS.write() = get_global_settings().version();
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    })
}

pub fn use_global_setting() -> Effect {
    let loaded = LOADED_VERSION_GLOBAL_SETTINGS.signal();
    use_effect(move || {
        let _ = loaded.read();
        let settings = get_global_settings();
        spawn(async move {
            let registry = settings.registry();

            apply_language_from_settings(registry);
            menu::main_window_menu_bar().apply_i18n();
            apply_theme(registry).await;

            if !window().is_visible() {
                tokio::time::sleep(Duration::from_millis(100)).await;
                window().set_visible(true);
            }
        });
    })
}

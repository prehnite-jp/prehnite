use crate::windows::utilities::show_modal;
use dioxus::prelude::*;
use dioxus_desktop::{Config, DesktopContext, WindowBuilder};
use dioxus_i18n::t;

pub async fn show_settings_window() -> DesktopContext {
    show_modal(
        SettingsWindow,
        Config::new().with_menu(None),
        WindowBuilder::new().with_title(t!("settings")),
    )
    .await
}

#[component]
pub fn SettingsWindow() -> Element {
    rsx! {}
}

use crate::app::settings::hooks::use_settings;
use crate::app::window::show_modal;
use crate::style::GlobalStyle;
use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;
use dioxus_desktop::{Config, DesktopContext, WindowBuilder};
use dioxus_i18n::t;

pub async fn show_license_info_window() -> DesktopContext {
    show_modal(
        LicenseInfoWindow,
        Config::new().with_menu(None),
        WindowBuilder::new()
            .with_title(t!("license_info"))
            .with_focused(true)
            .with_minimizable(false)
            .with_maximizable(false),
    )
    .await
}

#[component]
pub fn LicenseInfoWindow() -> Element {
    use_settings();
    rsx! {
        GlobalStyle {}
        div {
            {t!("wip")}
        }
    }
}

use dioxus::prelude::*;
use dioxus::core_macro::{component, rsx};
use dioxus_desktop::{Config, DesktopContext, WindowBuilder};
use dioxus_i18n::t;
use crate::windows::utilities::show_modal;

pub async fn show_license_info_window() -> DesktopContext {
    show_modal(
        LicenseInfoWindow,
        Config::new().with_menu(None),
        WindowBuilder::new().with_title(t!("license_info")),
    )
    .await
}

#[component]
pub fn LicenseInfoWindow() -> Element {
    rsx! {}
}
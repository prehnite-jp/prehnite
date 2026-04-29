use crate::style::GlobalStyle;
use crate::app::window::show_modal;
use dioxus::core_macro::{component, rsx};
use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_desktop::{window, Config, DesktopContext, WindowBuilder};
use dioxus_i18n::t;
use tracing_unwrap::ResultExt;
use crate::app::settings::hooks::{use_global_setting, use_setting_updator};

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
    use_setting_updator();
    use_global_setting();
    rsx! {
        GlobalStyle {}
        div {
            {t!("wip")}
        }
    }
}

use crate::app::settings::{use_setting_loader};
use crate::style::GlobalStyle;
use crate::windows::utilities::show_modal;
use dioxus::core_macro::{component, rsx};
use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_desktop::{window, Config, DesktopContext, WindowBuilder};
use dioxus_i18n::t;
use tracing_unwrap::ResultExt;

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
    use_setting_loader();
    let theme_sig = crate::app::settings::THEME.signal();
    use_effect(move || {
        let theme1 = theme_sig.read().cloned();
        let theme2 = theme1.clone();
        spawn(async move {
            eval(&format!(
                "document.documentElement.setAttribute(\"data-theme\", \"{}\");",
                theme1.clone()
            ))
            .await
            .ok_or_log();
        });
        window().set_theme(Some(theme2.into()));
    });
    rsx! {
        GlobalStyle {}
        div {
            {t!("wip")}
        }
    }
}

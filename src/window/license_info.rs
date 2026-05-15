use crate::app::settings::hooks::use_settings;
use crate::app::settings::{get_global_settings, save_global_settings};
use crate::app::window::show_modal;
use crate::components::button::Button;
use crate::components::dialog::{Dialog, DialogDescription, DialogTitle};
use crate::components_::icon::material_symbols::IconRounded;
use crate::style::GlobalStyle;
use dioxus::core_macro::{component, rsx};
use dioxus::html::completions::CompleteWithBraces::dialog;
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
pub fn Page() -> Element {
    use_settings();
    rsx! {
        GlobalStyle {}
        div {
            IconRounded { "list" }
        }
    }
}

#[component]
pub fn LicenseInfoWindow() -> Element {
    let mut dialog: Signal<bool> = use_signal(|| {
        !get_global_settings()
            .registry()
            .get_license_info_message_displayed()
    });

    rsx! {
        Dialog {
            open: dialog(),
            on_open_change: move |v| async move {
                dialog.set(v);
                save_global_settings({
                    let mut reg = get_global_settings().registry().clone();
                    reg.set_license_info_message_displayed(Some(true));
                    reg
                }).await;
            },
            DialogTitle {
                { t!("info") }
            }
            DialogDescription {
                { t!("license_info_message") }
            }
        }
        Page {}
    }
}

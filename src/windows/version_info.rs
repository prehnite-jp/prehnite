use crate::app::settings::{use_setting_loader};
use crate::assets::PREHNITE_LOGO;
use crate::style::GlobalStyle;
use crate::windows::utilities::show_modal;
use dioxus::desktop::{Config, DesktopContext, WindowBuilder};
use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_desktop::{window, LogicalSize};
use dioxus_i18n::t;
use tracing_unwrap::ResultExt;

fn app_version_info() -> String {
    t!("version_info_detail", app_name: {env!("CARGO_PKG_NAME")}, version: {env!("CARGO_PKG_VERSION")})
}

fn app_build_target() -> &'static str {
    env!("BUILD_INFO_TARGET")
}

fn app_build_features() -> &'static str {
    env!("BUILD_INFO_FEATURE")
}

fn app_build_profile() -> &'static str {
    env!("BUILD_PROFILE")
}

fn feature_decoration(features: &'static str) -> String {
    if features.is_empty() {
        "".into()
    } else {
        format!(".[{}]", features)
    }
}

pub async fn show_version_info_window() -> DesktopContext {
    show_modal(
        VersionInfoWindow,
        Config::new().with_menu(None),
        WindowBuilder::new()
            .with_title(t!("version_info"))
            .with_minimizable(false)
            .with_maximizable(false)
            .with_focused(true)
            .with_resizable(false)
            .with_inner_size(LogicalSize::new(400, 400)),
    )
    .await
}

#[component]
pub fn VersionInfoWindow() -> Element {
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
            justify_content: "center",
            align_content: "center",
            display: "flex",
            flex_direction: "column",
            text_align: "center",
            div{
                justify_content: "center",
                align_content: "center",
                display: "flex",
                flex_direction: "row",
                img { src: PREHNITE_LOGO, max_width: "256px", max_height: "256px" }
            }
            p {
                margin: "10px",
                font_size: "24px",
                {app_version_info()}
            }
            p {
                font_size: "15px",
                overflow_x: "scroll",
                text_wrap: "nowrap",
                { format!("{}.{}{}", app_build_profile(), app_build_target(), feature_decoration(app_build_features())) }
            }
        }
    }
}

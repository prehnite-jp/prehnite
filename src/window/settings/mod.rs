pub mod combo_selector;
mod edit_pane;
pub mod input;
pub mod node;
pub mod setting_label;
pub mod toggler;
mod tree_pane;

use crate::app::settings::hooks::use_settings;
use crate::app::settings::setting_schema::GlobalSettings;
use crate::app::settings::supported_languages::SupportedLanguages;
use crate::app::settings::theme::Theme;
use crate::app::settings::{get_global_settings, save_global_settings};
use crate::app::window::show_modal;
use crate::components::button::*;
use crate::style::{GlobalStyle, Height100};
use crate::util::alert::message_dialog_builder;
use crate::window::settings::combo_selector::ComboSelector;
use crate::window::settings::edit_pane::SettingEditPane;
use crate::window::settings::toggler::Toggler;
use crate::window::settings::tree_pane::SettingTreePane;
use dioxus::core::{spawn, Element};
use dioxus::core_macro::component;
use dioxus::hooks::{use_effect, use_signal};
use dioxus::prelude::*;
use dioxus_desktop::{
    use_wry_event_handler, window, Config, DesktopContext, WindowBuilder,
    WindowCloseBehaviour, WindowEvent,
};
use dioxus_i18n::t;
use easy_settings::{Registry, RegistryNode, SettingValue};
use std::iter::Filter;

const HIDDEN_SETTING_KEYS: &[&str] = &["last_opened_file"];
static CURRENT_CATEGORY: GlobalSignal<Option<&'static str>> =
    Signal::global(|| GlobalSettings::categories().first().cloned());
static CURRENT_SETTING_ENTRY: GlobalSignal<Option<&'static str>> = Signal::global(|| None);
static CHANGEABLE_REGISTRY: GlobalSignal<GlobalSettings> = Signal::global(GlobalSettings::default);

fn visible_children(
    parent: Option<&'static str>,
) -> Filter<std::slice::Iter<'static, RegistryNode>, fn(&&'_ RegistryNode) -> bool> {
    GlobalSettings::child_nodes(parent)
        .iter()
        .filter(visible_filter)
}

fn visible_filter(node: &&RegistryNode) -> bool {
    !HIDDEN_SETTING_KEYS.contains(&node.value())
}

pub async fn show_settings_window() -> DesktopContext {
    show_modal(
        SettingsWindow,
        Config::new().with_menu(None),
        WindowBuilder::new()
            .with_title(t!("settings"))
            .with_focused(true)
            .with_minimizable(false)
            .with_maximizable(false),
    )
    .await
}

#[component]
pub fn SettingsWindow() -> Element {
    use_settings();
    let mut settings_changed = use_signal(|| false);
    let registry_sig = CHANGEABLE_REGISTRY.signal();
    use_effect(move || {
        let registry = registry_sig.read();
        *settings_changed.write() = registry.ne(get_global_settings().registry())
    });
    use_effect(|| *CHANGEABLE_REGISTRY.write() = get_global_settings().registry().clone());
    use_wry_event_handler(move |e, _| {
        let x: bool = match e {
            dioxus_desktop::tao::event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => true,
            _ => false,
        };
        let _: _ = spawn(async move {
            if x {
                let window = window();
                if settings_changed() {
                    if message_dialog_builder()
                        .set_title(t!("confirm"))
                        .set_text(t!("confirm_settings_not_applied"))
                        .confirm()
                        .show()
                        .unwrap_or_default()
                    {
                        window.set_close_behavior(WindowCloseBehaviour::WindowHides);
                        save_global_settings(CHANGEABLE_REGISTRY()).await;
                        window.set_close_behavior(WindowCloseBehaviour::WindowCloses);
                        window.close();
                    }
                };
            }
        });
    });
    rsx! {
        GlobalStyle {}
        Height100 {}
        div {
            display: "grid",
            grid_template_columns: "30% 70%",
            grid_template_rows: "1fr 5em",
            height: "100%",
            div {
                grid_column: "1",
                border_right: "thin solid",
                border_bottom: "thin solid",
                border_color: "var(--secondary-color-5)",
                SettingTreePane {}
            }
            div {
                grid_column: "2",
                border_bottom: "thin solid",
                border_color: "var(--secondary-color-5)",
                SettingEditPane {}
            }
            div {
                grid_column: "2",
                margin_bottom: "10px",
                display: "flex",
                flex_direction: "row",
                justify_content: "right",
                align_content: "center",
                Button {
                    margin: "10px",
                    variant: ButtonVariant::Outline,
                    disabled: !settings_changed(),
                    onclick: move |_| async move {
                        if settings_changed() {
                            info!("{:?}", CHANGEABLE_REGISTRY);
                            save_global_settings(CHANGEABLE_REGISTRY()).await;
                            *settings_changed.write() = false;
                        }
                    },
                    {{t!("apply")}}
                }
            }
        }
    }
}

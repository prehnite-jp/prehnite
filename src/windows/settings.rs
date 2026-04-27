use crate::app::settings::{get_settings, save_all_settings, use_setting_loader, GlobalSettings, SupportedLanguages, Theme};
use crate::components::button::{Button, ButtonVariant};
use crate::components::select::{
    Select, SelectGroup, SelectItemIndicator, SelectList, SelectOption, SelectTrigger, SelectValue,
};
use crate::components::switch::{Switch, SwitchThumb};
use crate::style::{GlobalStyle, Height100};
use crate::util::alert::message_dialog_builder;
use crate::windows::utilities::show_modal;
use dioxus::prelude::*;
use dioxus_desktop::{
    use_wry_event_handler, window, Config, DesktopContext, WindowBuilder,
    WindowCloseBehaviour, WindowEvent,
};
use dioxus_i18n::t;
use easy_settings::{Registry, RegistryNode};
use dioxus::document::eval;
use strum::VariantArray;
use tracing_unwrap::ResultExt;

const HIDDEN_SETTING_KEYS: &[&str] = &["last_opened_file"];

static CURRENT_CATEGORY: GlobalSignal<Option<&'static str>> = Signal::global(|| None);
static CHANGEABLE_REGISTRY: GlobalSignal<GlobalSettings> = Signal::global(GlobalSettings::default);

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
fn SettingListPane() -> Element {
    rsx! {}
}

#[component]
fn SettingEditPane() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            margin: "1em 2em",
            p {
                {CURRENT_CATEGORY.read().as_ref().map(|x| t!(&format!("settings_category_{}", x))).unwrap_or(t!("settings"))}
            }
            div {
                display: "flex",
                flex_direction: "column",
                margin: "0 40px",
                for i in GlobalSettings::child_nodes(CURRENT_CATEGORY.read().cloned()).iter().filter(|x| !HIDDEN_SETTING_KEYS.contains(&x.value())) {
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "space-between",
                        SettingNode { node: i }
                    }
                }
            }
        }
    }
}

#[component]
fn ComboSelector<T>(
    setting_key: String,
    label: String,
    selected: Option<T>,
    on_value_change: Callback<Option<T>>,
) -> Element
where
    T: VariantArray + ToString + Clone + PartialEq,
{
    rsx! {
        label { for: "input_{setting_key}", "{label}" }
        Select::<T> {
            id: "input_{setting_key}",
            placeholder: "",
            value: selected,
            on_value_change,
            SelectTrigger {
                width: "12rem",
                SelectValue {}
            }
            SelectList {
                SelectGroup {
                    for (i, x) in T::VARIANTS.iter().enumerate() {
                        {
                            let lang_name = t!(&x.to_string());
                            rsx!{
                                SelectOption::<T> {
                                    id: "select_{x.to_string()}",
                                    index: i,
                                    value: x.clone(),
                                    text_value: lang_name.clone(),
                                    "{lang_name}"
                                    SelectItemIndicator {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SettingNode(node: &'static RegistryNode) -> Element {
    match node {
        RegistryNode::Category(_) => {
            rsx! {
                a {
                    onclick: move |_| *CURRENT_CATEGORY.write() = Some(node.value()),
                    { t!(&format!("settings_category_{}", node.value())) }
                }
            }
        }
        RegistryNode::SettingItem(_) => {
            let label = t!(&format!("settings_entry_{}", node.value()));
            if node.value() == "auto_open_last_opened_file" {
                rsx! {
                    label {  for: "input_node.value()", "{label}" }
                    Switch {
                        id: "input_{node.value()}",
                        checked: CHANGEABLE_REGISTRY.read().get_auto_open_last_opened_file(),
                        on_checked_change: move |x| {
                            CHANGEABLE_REGISTRY.write().set_auto_open_last_opened_file(Some(x));
                        },
                        SwitchThumb {}
                    }
                }
            } else if node.value() == "locale" {
                rsx! {
                    ComboSelector::<SupportedLanguages> {
                        setting_key: "{node.value()}",
                        label,
                        selected: CHANGEABLE_REGISTRY.read().get_locale(),
                        on_value_change: move |x| {
                            CHANGEABLE_REGISTRY.write().set_locale(x);
                        }
                    }
                }
            } else if node.value() == "theme" {
                rsx! {
                    ComboSelector::<Theme> {
                        setting_key: "{node.value()}",
                        label,
                        selected: CHANGEABLE_REGISTRY.read().get_theme(),
                        on_value_change: move |x| {
                            CHANGEABLE_REGISTRY.write().set_theme(x);
                        }
                    }
                }
            } else {
                rsx! {
                    label { "invalid node {node.value()}" }
                }
            }
        }
    }
}

#[component]
pub fn SettingsWindow() -> Element {
    let mut settings_changed = use_signal(|| false);
    let registry_sig = CHANGEABLE_REGISTRY.signal();
    use_effect(move || {
        let registry = registry_sig.read();
        *settings_changed.write() = registry.ne(&get_settings().read())
    });
    use_effect(|| *CHANGEABLE_REGISTRY.write() = get_settings().cloned());
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
                if settings_changed.read().cloned() {
                    if message_dialog_builder()
                        .set_title(t!("confirm"))
                        .set_text(t!("confirm_settings_not_applied"))
                        .confirm()
                        .show()
                        .unwrap_or_default()
                    {
                        window.set_close_behavior(WindowCloseBehaviour::WindowHides);
                        save_all_settings(CHANGEABLE_REGISTRY.read().cloned())
                            .await
                            .ok_or_log();
                        window.set_close_behavior(WindowCloseBehaviour::WindowCloses);
                        window.close();
                    }
                };
            }
        });
    });
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
        Height100 {}
        div {
            display: "grid",
            grid_template_columns: "30% auto",
            grid_template_rows: "auto 4em",
            height: "100%",
            div {
                grid_column: "1",
                border_right: "thin solid",
                border_bottom: "thin solid",
                border_color: "var(--secondary-color-5)",
                SettingListPane {}
            }
            div {
                grid_column: "2",
                border_bottom: "thin solid",
                border_color: "var(--secondary-color-5)",
                SettingEditPane {}
            }
            div {
                grid_column: "2",
                margin: "10px",
                display: "flex",
                flex_direction: "row",
                justify_content: "right",
                align_content: "center",
                Button {
                    variant: ButtonVariant::Outline,
                    disabled: !settings_changed.read().cloned(),
                    onclick: move |_| async move {
                        if *settings_changed.read() {
                            save_all_settings(CHANGEABLE_REGISTRY.read().cloned()).await.ok_or_log();
                            *settings_changed.write() = false;
                        }
                    },
                    {{t!("apply")}}
                }
            }
        }
    }
}

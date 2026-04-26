use crate::app::settings::{get_settings, save_all_settings, GlobalSettings, SupportedLanguages};
use crate::assets::DX_COMPONENTS_THEME_CSS;
use crate::components::button::{Button, ButtonVariant};
use crate::components::select::{
    Select, SelectGroup, SelectItemIndicator, SelectList, SelectOption, SelectTrigger, SelectValue,
};
use crate::components::switch::{Switch, SwitchThumb};
use crate::util::alert::message_dialog_builder;
use crate::windows::utilities::show_modal;
use dioxus::prelude::*;
use dioxus_desktop::{
    use_window, use_wry_event_handler, Config, DesktopContext, WindowBuilder, WindowCloseBehaviour,
    WindowEvent,
};
use dioxus_i18n::t;
use easy_settings::{Registry, RegistryNode};
use std::ops::Deref;
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

// #[component]
// fn SettingListPane(current_node: Signal<Option<RegistryNode>>) -> Element {}
//

#[component]
fn SettingEditPane() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
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
                Button {
                    variant: ButtonVariant::Ghost,
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
    use_effect(|| *CHANGEABLE_REGISTRY.write() = get_settings().deref().clone());
    use_wry_event_handler(move |e, _| match e {
        dioxus_desktop::tao::event::Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
            ..
        } => {
            let window = use_window();
            if *window_id == window.id() && CHANGEABLE_REGISTRY.read().ne(get_settings().deref()) {
                if message_dialog_builder()
                    .set_title(t!("confirm"))
                    .set_text(t!("confirm_settings_not_applied"))
                    .confirm()
                    .show()
                    .unwrap_or_default()
                {
                    window.set_close_behavior(WindowCloseBehaviour::WindowHides);
                    use_future(move || async move {
                        save_all_settings(CHANGEABLE_REGISTRY.read().cloned())
                            .await
                            .ok_or_log();
                        use_window().set_close_behavior(WindowCloseBehaviour::WindowCloses);
                        use_window().close();
                    });
                }
            }
        }
        _ => {}
    });
    rsx! {
        document::Link { rel: "stylesheet", href: DX_COMPONENTS_THEME_CSS }
        SettingEditPane {  }
    }
}

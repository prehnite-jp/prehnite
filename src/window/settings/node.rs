use crate::window::settings::input::SettingInput;
use crate::window::settings::*;
use dioxus::core::Element;
use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;
use dioxus_i18n::t;
use easy_settings::RegistryNode;
use std::any::TypeId;

macro_rules! combo_impl {
    ($label:ident, $node:ident, $v:ident, ($($x:ty),+)) => {
        match $v {
            $(ty if ty == TypeId::of::<$x>() => {
                rsx! {
                    ComboSelector::<$x> {
                        setting_key: {$node.value()},
                        $label,
                        selected: CHANGEABLE_REGISTRY.read().get_with_default($node.value()).unwrap(),
                        on_value_change: move |x| {
                            CHANGEABLE_REGISTRY.write().set($node.value(), SettingValue::from(x));
                        }
                    }
                }
            })+
            _ => {
                rsx! {
                    p {
                        {format!("field {} is not compatible type.", $label)}
                    }
                }
            }
        }
    }
}

#[component]
pub fn SettingNode(node: &'static RegistryNode) -> Element {
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
            let ty = GlobalSettings::get_item_type(node.value()).unwrap();
            match ty {
                ty if ty == TypeId::of::<bool>() => {
                    rsx! {
                        Toggler {
                            setting_key: "{node.value()}",
                            label,
                            checked: CHANGEABLE_REGISTRY.read().get_with_default(node.value()).unwrap(),
                            on_checked_change: move |x| {
                                CHANGEABLE_REGISTRY.write().set(node.value(), SettingValue::from(Some(x)));
                            },
                        }
                    }
                }
                ty if ty == TypeId::of::<String>() => {
                    rsx! {
                        SettingInput {
                            setting_key: "{node.value()}",
                            label,
                            value:CHANGEABLE_REGISTRY.read().get_with_default(node.value()).unwrap(),
                            on_changed: move |x| {
                                CHANGEABLE_REGISTRY.write().set(node.value(), SettingValue::from(Some(x)));
                            }
                        }
                    }
                }
                _ => combo_impl!(label, node, ty, (SupportedLanguages, Theme)),
            }
        }
    }
}

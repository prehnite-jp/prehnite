use crate::app::settings::setting_schema::GlobalSettings;
use crate::components::accordion::{AccordionContent, AccordionItem};
use crate::components::scroll_area::ScrollArea;
use crate::custom_dx_components::accordion::{CustomAccordion, CustomAccordionTrigger};
use crate::window::settings::{
    visible_children, visible_filter, CURRENT_CATEGORY, CURRENT_SETTING_ENTRY,
};
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_primitives::scroll_area::ScrollDirection;
use easy_settings::{Registry, RegistryNode};

#[component]
fn SettingEntry(node: &'static str) -> Element {
    rsx! {
        span {
            tabindex: 0,
            class: "show-focus show-hover",
            text_wrap: "nowrap",
            margin_right: "2em",
            font_size: "11px",
            onfocusin: move |_| *CURRENT_SETTING_ENTRY.write() = Some(node),
            onfocusout: move |_| *CURRENT_SETTING_ENTRY.write() = None,
            { t!(&format!("settings_entry_{}", node)) }
        }
    }
}

#[component]
fn ItemNode(node: &'static str) -> Element {
    rsx! {
        AccordionContent {
            SettingEntry {
                node
            }
        }
    }
}

#[component]
fn Nodes(children_node: &'static [RegistryNode]) -> Element {
    let children_node = children_node.iter().filter(visible_filter);
    rsx! {
        for (index, category) in children_node.clone().filter_map(|x| if x.is_category() { Some(x.value()) } else { None }).enumerate() {
            AccordionContent {
                CustomAccordion {
                    CategoryTree {
                        index,
                        category
                    }
                }
            }
        }
        for node in children_node.filter_map(|x| if x.is_setting_item() { Some(x.value()) } else { None }) {
            ItemNode {
                node
            }
        }
    }
}

#[component]
fn CategoryTree(index: usize, category: &'static str) -> Element {
    let children_node = GlobalSettings::child_nodes(Some(category));
    rsx! {
        AccordionItem {
            index,
            div {
                onfocusin: move |_| *CURRENT_CATEGORY.write() = Some(category),
                CustomAccordionTrigger {
                    span {
                        { t!(&format!("settings_category_{}", category)) }
                    }
                },
            }
            Nodes {
                children_node
            }
        }
    }
}

#[component]
pub fn SettingTreePane() -> Element {
    let children_node = visible_children(None);

    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            div {
                border_bottom: "thin solid",
                border_color: "var(--secondary-color-5)",
                p {
                    text_overflow: "ellipsis",
                    margin: "0.5em 1em",
                    white_space: "pre",
                    " "
                }
            }
            ScrollArea {
                height: "calc(100vh - 7em)",
                direction: ScrollDirection::Both,
                div {
                    for (index, category) in children_node.clone().filter_map(|x| if x.is_category() { Some(x.value()) } else { None }).enumerate() {
                        CustomAccordion {
                            CategoryTree {
                                index,
                                category
                            }
                        }
                    }
                    for node in children_node.filter_map(|x| if x.is_setting_item() { Some(x.value()) } else { None }) {
                        SettingEntry {
                            node
                        }
                    }
                }
            }
        }
    }
}

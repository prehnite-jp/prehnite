use super::*;
use crate::window::settings::child_category_nodes::ChildCategoryNodes;
use crate::window::settings::node::SettingNode;
use dioxus::core::Element;
use dioxus::core_macro::{component, rsx};

#[component]
pub fn SettingEditPane() -> Element {
    let children = GlobalSettings::child_nodes(CURRENT_CATEGORY())
        .iter()
        .filter(|x| !HIDDEN_SETTING_KEYS.contains(&x.value()));
    let child_categories = children.clone().filter(|x| x.is_category());
    let child_items = children.filter(|x| x.is_setting_item());
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
                    {CURRENT_CATEGORY.read().as_ref().map(|x| t!(&format!("settings_category_{}", x))).unwrap_or(t!("settings"))}
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                row_gap: "0.5em",
                max_height: "50em",
                div {
                    margin: "0.5em 0 0 2em",
                    for i in child_items
                    {
                        div {
                            display: "flex",
                            flex_direction: "row",
                            justify_content: "left",
                            column_gap: "1em",
                            SettingNode { node: i }
                        }
                    }
                }
                for i in child_categories {
                    ChildCategoryNodes {
                        category: i.value()
                    }
                }
            }
        }
    }
}

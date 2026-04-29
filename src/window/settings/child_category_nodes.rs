use crate::app::settings::setting_schema::GlobalSettings;
use crate::window::settings::node::SettingNode;
use crate::window::settings::{HIDDEN_SETTING_KEYS};
use dioxus::prelude::*;
use dioxus_i18n::t;
use easy_settings::Registry;

#[component]
pub fn ChildCategoryNodes(category: Option<&'static str>) -> Element {
    let children = GlobalSettings::child_nodes(category)
        .iter()
        .filter(|x| !HIDDEN_SETTING_KEYS.contains(&x.value()));
    let child_categories = children.clone().filter(|x| x.is_category());
    let child_items = children.filter(|x| x.is_setting_item());
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            p {
                margin: "0.5em 1em",
                {category.map(|x| t!(&format!("settings_category_{}", x))).unwrap_or(t!("settings"))}
            }
            div {
                display: "flex",
                flex_direction: "column",
                row_gap: "0.5em",
                margin: "0 0 0 2em",
                for i in child_items.chain(child_categories) {
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "left",
                        column_gap: "1em",
                        SettingNode { node: i }
                    }
                }
            }
        }
    }
}

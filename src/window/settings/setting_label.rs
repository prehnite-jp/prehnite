use dioxus::prelude::*;

#[component]
pub fn SettingLabel(setting_key: String, label: String) -> Element {
    rsx! {
        label {  for: "input_{setting_key}", text_wrap: "nowrap", display: "flex", align_items: "center", "{label}" }
    }
}

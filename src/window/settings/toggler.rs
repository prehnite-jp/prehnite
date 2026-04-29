use crate::components::checkbox::*;
use crate::window::settings::setting_label::SettingLabel;
use dioxus::prelude::*;
use dioxus_primitives::checkbox::CheckboxState;
use easy_settings::SettingValue;

#[component]
pub fn Toggler(
    setting_key: String,
    label: String,
    checked: SettingValue,
    on_checked_change: Callback<bool>,
) -> Element {
    rsx! {
        div {
            display: "flex",
            align_items: "center",
            height: "auto",
            Checkbox {
                id: "input_{setting_key}",
                checked: Option::<bool>::from(checked).map(|x| if x {CheckboxState::Checked} else {CheckboxState::Unchecked}),
                on_checked_change: move |x| {
                    on_checked_change.call(x == CheckboxState::Checked);
                },
            }
        }
        SettingLabel { setting_key, label }
    }
}

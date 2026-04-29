use crate::components::input::Input;
use crate::window::settings::setting_label::SettingLabel;
use dioxus::prelude::*;
use easy_settings::SettingValue;

#[component]
pub fn SettingInput(
    setting_key: String,
    label: String,
    value: SettingValue,
    on_changed: Callback<String>,
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>
) -> Element {
    rsx! {
        SettingLabel { setting_key: setting_key.clone(), label }
        Input {
            id: "input_{setting_key}",
            value: Option::<String>::from(value).unwrap_or_default(),
            onchange: move |x:Event<FormData>| {
                on_changed.call(x.value())
            },
            attributes
        }
    }
}

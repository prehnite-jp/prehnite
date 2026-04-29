use crate::components::select::*;
use crate::window::settings::setting_label::SettingLabel;
use dioxus::core::{Callback, Element};
use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;
use dioxus_i18n::t;
use easy_settings::SettingValue;
use serde::de::DeserializeOwned;
use strum::VariantArray;

#[component]
pub fn ComboSelector<T>(
    setting_key: String,
    label: String,
    selected: SettingValue,
    on_value_change: Callback<Option<T>>,
) -> Element
where
    T: VariantArray + ToString + Clone + PartialEq + DeserializeOwned,
{
    rsx! {
        SettingLabel { setting_key: setting_key.clone(), label }
        Select::<T> {
            id: "input_{setting_key}",
            placeholder: "",
            value: Option::<T>::from(selected),
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
                                    id: "option_{x.to_string()}",
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

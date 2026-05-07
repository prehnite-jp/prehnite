use dioxus::prelude::*;

#[component]
pub fn IconOutlined(
    children: Element,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        span {
            class: "material-symbols-outlined",
            ..attributes,
            { children }
        }
    }
}

#[component]
pub fn IconRounded(
    children: Element,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        span {
            class: "material-symbols-rounded",
            ..attributes,
            { children }
        }
    }
}

#[component]
pub fn IconSharp(
    children: Element,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        span {
            class: "material-symbols-sharp",
            ..attributes,
            { children }
        }
    }
}

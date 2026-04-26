use dioxus::prelude::*;
use crate::assets::{DX_COMPONENTS_THEME_CSS, GLOBAL_CSS};

#[component]
pub fn GlobalStyle() -> Element {
    rsx!{
        document::Link { rel: "stylesheet", href: DX_COMPONENTS_THEME_CSS },
        document::Link { rel: "stylesheet", href: GLOBAL_CSS }
    }
}
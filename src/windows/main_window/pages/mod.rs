use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home,
}

#[component]
fn Home() -> Element {
    rsx! { { t!("wip") } }
}

use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home,
}

#[component]
fn Home() -> Element {
    rsx! {}
}

use crate::windows::about::show_about_window;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home,
}

#[component]
fn Home() -> Element {
    rsx! { button{ onclick: move |_| async move { show_about_window().await; }, "Click"} }
}

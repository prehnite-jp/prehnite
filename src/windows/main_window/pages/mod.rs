use crate::windows::version_info::show_version_info_window;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home,
}

#[component]
fn Home() -> Element {
    rsx! { button{ onclick: move |_| async move { show_version_info_window().await; }, "Click"} }
}

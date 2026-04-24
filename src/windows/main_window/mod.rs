use crate::app::initializer;
use crate::windows::main_window::pages::Route;
use dioxus::prelude::*;

pub mod pages;

#[component]
pub fn PrehniteApp() -> Element {
    initializer::initialize_i18n();
    rsx! { Router::<Route> {} }
}

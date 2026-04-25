use crate::app::initializer;
use crate::windows::main_window::pages::Route;
use dioxus::prelude::*;

pub mod menu;
pub mod pages;

#[component]
pub fn PrehniteApp() -> Element {
    initializer::initialize_i18n();
    (*menu::main_window_menu_bar())
        .as_ref()
        .map(|x| x.apply_i18n());
    rsx! { Router::<Route> {} }
}

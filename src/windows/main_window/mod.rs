use crate::windows::version_info::show_version_info_window;
use crate::windows::main_window::menu::menu_handler;
use crate::windows::main_window::pages::Route;
use crate::windows::utilities::page_initializer;
use dioxus::prelude::*;
use dioxus_desktop::use_muda_event_handler;

pub mod menu;
pub mod pages;

#[component]
pub fn PrehniteApp() -> Element {
    page_initializer();
    menu_handler();
    (*menu::main_window_menu_bar())
        .as_ref()
        .map(|x| x.apply_i18n());
    rsx! { Router::<Route> {} }
}

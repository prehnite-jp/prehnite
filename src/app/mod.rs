pub mod book;
pub mod db;
pub(crate) mod initializer;
pub mod settings;

use crate::pages::Route;
use dioxus::prelude::*;

#[component]
pub fn PrehniteApp() -> Element {
    initializer::initialize_i18n();
    rsx! { Router::<Route> {} }
}

pub mod book;
pub mod db;
pub(crate) mod initializer;
pub mod settings;

use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn PrehniteApp() -> Element {
    initializer::initialize_i18n();
    rsx! {
        div{ { t!("wip") } }
    }
}

#[cfg(feature = "desktop")]
pub(crate) mod initializer;
pub mod settings;

use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn PrehniteApp() -> Element {
    let mut initialized = use_signal(|| false);
    #[cfg(feature = "desktop")]
    use_resource(move || async move {
        initializer::load_settings().await.unwrap();
        initializer::initialize_i18n();
        initialized.set(true);
    });

    if initialized.read().cloned() {
        rsx! {
            div { { t!("wip") } }
        }
    } else {
        rsx! {}
    }
}

pub(crate) mod initializer;
pub mod settings;
pub mod db;

use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn PrehniteApp() -> Element {
    let mut initialized = use_signal(|| false);
    let mut filename: Signal<PathBuf> = use_signal(PathBuf::new);
    use_future(move || async move {
        initializer::load_settings().await.unwrap();
        initializer::initialize_i18n();
        initialized.set(true);
    });

    if initialized.read().cloned() {
        rsx! {
            // div { { t!("wip") } }
            input {
                // tell the input to pick a file
                type: "file",
                // list the accepted extensions
                accept: ".txt,.rs",
                multiple: false,
                onchange: move |e| {
                    if let Some(x) = e.files().first() {
                        *filename.write() = x.path();
                    }
                },
            }
        }
    } else {
        rsx! {}
    }
}

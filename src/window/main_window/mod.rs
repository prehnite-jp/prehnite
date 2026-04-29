use crate::app::book::open_new_book;
use crate::app::db::{close_book_db_pool, is_book_opened, open_book_db_pool};
use crate::app::settings::supported_languages::SupportedLanguages;
use crate::app::settings::{get_global_settings, load_global_settings, save_global_settings};
use crate::style::GlobalStyle;
use crate::window::main_window::menu::{update_menu_status, use_menu_handler};
use crate::window::main_window::pages::Route;
use dioxus::prelude::*;
use dioxus_desktop::{use_wry_event_handler, window, DesktopContext, WindowEvent};
use dioxus_i18n::prelude::*;
use std::path::PathBuf;
use std::sync::OnceLock;
use tracing_unwrap::ResultExt;
use crate::app::settings::hooks::{use_global_setting, use_setting_updator};

pub mod menu;
pub mod pages;

async fn new_book(path: PathBuf) {
    open_new_book(&path).await.ok_or_log();
    update_menu_status();
    let mut settings = get_global_settings().registry().clone();
    settings.set_last_opened_file(path.to_str().map(|x| x.into()));
    save_global_settings(settings).await;
}

async fn open_book(path: PathBuf) {
    open_book_db_pool(&path).await.ok_or_log();
    update_menu_status();
    let mut settings = get_global_settings().registry().clone();
    settings.set_last_opened_file(path.to_str().map(|x| x.into()));
    save_global_settings(settings).await;
}

async fn close_book() {
    close_book_db_pool().await;
    update_menu_status();
    let mut settings = get_global_settings().registry().clone();
    settings.set_last_opened_file(None);
    save_global_settings(settings).await;
}

thread_local! {
static MAIN_WINDOW_CONTEXT: OnceLock<DesktopContext> = OnceLock::new();
}

pub fn get_main_window_context() -> Option<DesktopContext> {
    MAIN_WINDOW_CONTEXT.with(|x| x.get().cloned())
}

#[component]
pub fn PrehniteApp() -> Element {
    use_future(move || async {
        load_global_settings().await;
    });
    use_setting_updator();
    use_global_setting();
    use_init_i18n(|| {
        I18nConfig::new(get_global_settings().registry().get_locale().into())
            .with_locale((
                SupportedLanguages::EnUS.into(),
                include_str!("../../../assets/locales/en-US.ftl"),
            ))
            .with_locale((
                SupportedLanguages::JaJP.into(),
                include_str!("../../../assets/locales/ja-JP.ftl"),
            ))
    });
    use_menu_handler();
    // ちらつきを抑止
    use_wry_event_handler(move |x, _| match x {
        dioxus_desktop::tao::event::Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            window().set_visible(false);
        }
        _ => {}
    });
    use_future(|| async {
        let s = get_global_settings();
        let reg = s.registry();
        if let Some(path) = reg
            .get_last_opened_file()
            .filter(move |_| !reg.get_auto_open_last_opened_file())
        {
            open_book(path.into()).await;
            if !is_book_opened() {
                close_book().await;
            }
        }
    });
    use_future(move || async {
        MAIN_WINDOW_CONTEXT.with(|x| {
            if x.get().is_none() {
                x.set(window()).ok();
            }
        });
    });
    rsx! {
        GlobalStyle {}
        div {
            Router::<Route> {}
        }
    }
}

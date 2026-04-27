use crate::app::book::open_new_book;
use crate::app::db::{close_book_db_pool, is_book_opened, open_book_db_pool};
use crate::app::settings;
use crate::app::settings::{
    get_settings, save_all_settings, use_setting_loader, SupportedLanguages,
};
use crate::style::GlobalStyle;
use crate::windows::main_window::menu::{update_menu_status, use_menu_handler};
use crate::windows::main_window::pages::Route;
use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_desktop::{use_wry_event_handler, window, DesktopContext, WindowEvent};
use dioxus_i18n::prelude::*;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::OnceLock;
use tracing_unwrap::ResultExt;

pub mod menu;
pub mod pages;

async fn new_book(path: PathBuf) {
    open_new_book(&path).await.ok_or_log();
    update_menu_status();
    let mut settings = get_settings().deref().clone();
    settings.set_last_opened_file(path.to_str().map(|x| x.into()));
    save_all_settings(settings).await.ok_or_log();
}

async fn open_book(path: PathBuf) {
    open_book_db_pool(&path).await.ok_or_log();
    update_menu_status();
    let mut settings = get_settings().deref().clone();
    settings.set_last_opened_file(path.to_str().map(|x| x.into()));
    save_all_settings(settings).await.ok_or_log();
}

async fn close_book() {
    close_book_db_pool().await;
    update_menu_status();
    let mut settings = get_settings().deref().clone();
    settings.set_last_opened_file(None);
    save_all_settings(settings).await.ok_or_log();
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
        settings::load().await.ok_or_log();
    });
    use_init_i18n(|| {
        I18nConfig::new(get_settings().get_locale().into())
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
        let s = get_settings();
        if let Some(path) = s
            .get_last_opened_file()
            .filter(move |_| !s.get_auto_open_last_opened_file())
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
    use_setting_loader();
    let theme_sig = settings::THEME.signal();
    use_effect(move || {
        let theme = theme_sig.read();
        let theme1 = theme.cloned();
        let theme2 = theme1.clone();
        spawn(async move {
            eval(&format!(
                "document.documentElement.setAttribute(\"data-theme\", \"{}\");",
                theme1.clone()
            ))
            .await
            .ok_or_log();
        });
        window().set_theme(Some(theme2.into()));
    });
    rsx! {
        GlobalStyle {}
        div {
            Router::<Route> {}
        }
    }
}

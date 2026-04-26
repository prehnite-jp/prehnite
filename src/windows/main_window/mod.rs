use crate::app::book::open_new_book;
use crate::app::db::{close_book_db_pool, is_book_opened, open_book_db_pool};
use crate::app::settings::{get_settings, save_all_settings};
use crate::style::GlobalStyle;
use crate::windows::main_window::menu::{menu_handler, update_menu_status};
use crate::windows::main_window::pages::Route;
use crate::windows::utilities::page_initializer;
use dioxus::prelude::*;
use std::ops::Deref;
use std::path::PathBuf;
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

fn auto_opener() {
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
}

#[component]
pub fn PrehniteApp() -> Element {
    page_initializer();
    menu_handler();
    auto_opener();
    (*menu::main_window_menu_bar())
        .as_ref()
        .unwrap()
        .apply_i18n();
    rsx! {
        GlobalStyle {}
        Router::<Route> {}
    }
}

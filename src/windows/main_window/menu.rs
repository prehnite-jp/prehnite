use crate::app::db::is_book_opened;
use crate::util::alert::{file_dialog_builder, AlertResult};
use crate::windows::license_info::show_license_info_window;
use crate::windows::main_window::{close_book, new_book, open_book};
use crate::windows::settings::show_settings_window;
use crate::windows::version_info::show_version_info_window;
use dioxus::desktop::muda::{Menu, MenuItem, Submenu};
use dioxus::dioxus_core;
use dioxus_desktop::{use_muda_event_handler, window};
use dioxus_i18n::t;
use std::ops::Deref;
use std::rc::Rc;
use tracing_unwrap::ResultExt;

struct I18nSubmenu {
    key: &'static str,
    itm: Submenu,
}

impl I18nSubmenu {
    fn apply_i18n(&self) {
        self.itm.set_text(t!(self.key));
    }
}

impl From<(&'static str, bool, Submenu)> for I18nSubmenu {
    fn from((key, _, itm): (&'static str, bool, Submenu)) -> Self {
        Self { key, itm }
    }
}

struct I18nMenuItem {
    key: &'static str,
    itm: MenuItem,
}

impl I18nMenuItem {
    fn apply_i18n(&self) {
        self.itm.set_text(t!(self.key));
    }

    fn apply_i18n_vec(items: &Vec<Self>) {
        items.iter().for_each(|x| x.apply_i18n());
    }
}

impl From<(&'static str, bool, MenuItem)> for I18nMenuItem {
    fn from((key, _, itm): (&'static str, bool, MenuItem)) -> Self {
        Self { key, itm }
    }
}

struct MenuBarBuilder {
    file: (&'static str, bool),
    file_menu_items: Vec<(&'static str, bool)>,
    help: (&'static str, bool),
    help_menu_items: Vec<(&'static str, bool)>,
}

impl MenuBarBuilder {
    fn sub_menu_builder((text, enabled): (&'static str, bool)) -> I18nSubmenu {
        (text, enabled, Submenu::with_id(text, text, enabled)).into()
    }

    fn menu_item_builder((text, enabled): (&'static str, bool)) -> I18nMenuItem {
        (text, enabled, MenuItem::with_id(text, text, enabled, None)).into()
    }

    fn menu_item_builder_vec(list: &Vec<(&'static str, bool)>) -> Vec<I18nMenuItem> {
        list.into_iter()
            .map(|x| *x)
            .map(Self::menu_item_builder)
            .collect()
    }

    fn append_menu_item(submenu: &Submenu, menu_item: &Vec<I18nMenuItem>) -> anyhow::Result<()> {
        for m in menu_item {
            submenu.append(&m.itm)?;
        }
        Ok(())
    }

    pub fn build(self) -> anyhow::Result<MenuBar> {
        let file = Self::sub_menu_builder(self.file);
        let file_menu_items: Vec<_> = Self::menu_item_builder_vec(&self.file_menu_items);
        Self::append_menu_item(&file.itm, &file_menu_items)?;
        let help = Self::sub_menu_builder(self.help);
        let help_menu_items = Self::menu_item_builder_vec(&self.help_menu_items);
        Self::append_menu_item(&help.itm, &help_menu_items)?;
        let menu = Menu::new();
        menu.append(&file.itm)?;
        menu.append(&help.itm)?;
        Ok(MenuBar {
            menu,
            file,
            file_menu_items,
            help,
            help_menu_items,
        })
    }
}

pub struct MenuBar {
    menu: Menu,
    file: I18nSubmenu,
    file_menu_items: Vec<I18nMenuItem>,
    help: I18nSubmenu,
    help_menu_items: Vec<I18nMenuItem>,
}

impl MenuBar {
    pub fn get_menu(&self) -> &Menu {
        &self.menu
    }

    pub fn apply_i18n(&self) {
        self.file.apply_i18n();
        I18nMenuItem::apply_i18n_vec(&self.file_menu_items);
        self.help.apply_i18n();
        I18nMenuItem::apply_i18n_vec(&self.help_menu_items);
    }
}

fn default_menubar() -> MenuBar {
    MenuBarBuilder {
        file: ("file", true),
        file_menu_items: vec![
            ("new_file", true),
            ("open_file", true),
            ("close_file", false),
            ("settings", true),
            ("exit", true),
        ],
        help: ("help", true),
        help_menu_items: vec![("version_info", true), ("license_info", true)],
    }
    .build()
    .unwrap_or_alert()
}

thread_local! {
    static MENU_BAR: Rc<MenuBar> = Rc::new(default_menubar());
}

pub fn main_window_menu_bar() -> Rc<MenuBar> {
    MENU_BAR.with(move |x| x.clone())
}

pub fn update_menu_status() {
    main_window_menu_bar()
        .deref()
        .file_menu_items
        .iter()
        .for_each(|x| match x.key {
            "close_file" => x.itm.set_enabled(is_book_opened()),
            _ => {}
        });
}

pub(super) fn use_menu_handler() {
    use_muda_event_handler(|e| match e.id().0.as_str() {
        "new_file" => {
            let _: _ = dioxus_core::spawn(async {
                let file = file_dialog_builder()
                    .set_title(t!("new_file"))
                    .add_filter("prehnite book", ["prehnite"])
                    .save_single_file()
                    .spawn()
                    .await;
                if let Some(file) = file.ok_or_log().and_then(|x| x) {
                    new_book(file).await;
                }
            });
        }
        "open_file" => {
            let _: _ = dioxus_core::spawn(async {
                let file = file_dialog_builder()
                    .set_title(t!("open_file"))
                    .add_filter("prehnite book", ["prehnite"])
                    .open_single_file()
                    .spawn()
                    .await;
                if let Some(file) = file.ok_or_log().and_then(|x| x) {
                    open_book(file).await;
                }
            });
        }
        "close_file" => {
            let _: _ = dioxus_core::spawn(close_book());
        }
        "settings" => {
            let _: _ = dioxus_core::spawn(async {
                show_settings_window().await;
            });
        }
        "exit" => {
            window().close();
        }
        "version_info" => {
            let _: _ = dioxus_core::spawn(async {
                show_version_info_window().await;
            });
        }
        "license_info" => {
            let _: _ = dioxus_core::spawn(async {
                show_license_info_window().await;
            });
        }
        _ => {
            println!("x");
        }
    });
}

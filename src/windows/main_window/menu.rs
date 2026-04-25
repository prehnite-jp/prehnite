use dioxus::desktop::muda::{Menu, MenuItem, Submenu};
use dioxus_i18n::t;
use std::rc::Rc;

struct I18nSubmenu {
    key: &'static str,
    itm: Submenu,
}

impl I18nSubmenu {
    fn apply_i18n(&self) {
        self.itm.set_text(t!(self.key));
    }

    fn apply_i18n_vec(items: &Vec<Self>) {
        items.iter().for_each(|x| x.apply_i18n());
    }
}

impl From<(&'static str, Submenu)> for I18nSubmenu {
    fn from((key, itm): (&'static str, Submenu)) -> Self {
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

impl From<(&'static str, MenuItem)> for I18nMenuItem {
    fn from((key, itm): (&'static str, MenuItem)) -> Self {
        Self { key, itm }
    }
}

struct MenuBarBuilder {
    file: &'static str,
    file_menu_items: Vec<&'static str>,
    help: &'static str,
    help_menu_items: Vec<&'static str>,
}

impl MenuBarBuilder {
    fn sub_menu_builder(text: &'static str) -> I18nSubmenu {
        (text, Submenu::new(text, true)).into()
    }

    fn menu_item_builder(text: &'static str) -> I18nMenuItem {
        (text, MenuItem::new(text, true, None)).into()
    }

    fn menu_item_builder_vec(list: &Vec<&'static str>) -> Vec<I18nMenuItem> {
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

    pub fn new() -> Self {
        MenuBarBuilder {
            file: "file",
            file_menu_items: vec!["open_file", "close_file", "settings", "exit"],
            help: "help",
            help_menu_items: vec!["version_info", "license_info"],
        }
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

thread_local! {
    static MENU_BAR: Rc<Option<MenuBar>> = Rc::new(MenuBarBuilder::new().build().ok());
}

pub fn main_window_menu_bar() -> Rc<Option<MenuBar>> {
    MENU_BAR.with(move |x| x.clone())
}

use crate::app::button;
use crate::app::i18n_w;
use crate::app::window::main_window::page::MainWindowPageId;
use crate::app::window::main_window::{BookOpenerMessage, MainWindow, MainWindowMessage};
use crate::app::Border;
use crate::app::Radius;
use iced::{widget, Element, Length, Task};
use iced_aw::menu::Item;
use iced_aw::menu_items;
use iced_aw::{menu, menu_bar};
use prehnite_core::db::close_book_err_handled;

#[derive(Clone, Debug)]
pub enum MenuType {
    File,
    Show,
    Help,
}

#[derive(Clone, Debug)]
pub enum MenuBarMessage {
    MenuBtnPressed(MenuType),
    NewFile,
    OpenFile,
    CloseFile,
    OpenSettings,
    OpenBackgroundInfoEditor,
    OpenBibliographyEditor,
    OpenVersionInfoWindow,
    Exit,
    OpenLicenseInfoWindow,
}

macro_rules! menu_button_maybe {
    ($i18n_id:expr, $message:expr) => {
        button(i18n_w($i18n_id).size(12))
            .style(button::text)
            .width(150.0f32)
            .on_press_maybe($message)
    };
}

macro_rules! menu_button {
    ($i18n_id:expr, $message:expr) => {
        menu_button_maybe!($i18n_id, Some($message))
    };
}
macro_rules! top_level_menu_button {
    ($i18n_id:expr, $message:expr) => {
        button(i18n_w($i18n_id).size(12))
            .style(button::text)
            .on_press($message)
    };
}

pub fn menubar<'a>(is_book_opened: bool) -> Element<'a, MenuBarMessage> {
    let file_menu: Item<MenuBarMessage, _, _> = Item::with_menu(
        top_level_menu_button!("file", MenuBarMessage::MenuBtnPressed(MenuType::File)),
        menu!(
            (menu_button!("new-file", MenuBarMessage::NewFile)),
            (menu_button!("open-file", MenuBarMessage::OpenFile)),
            (menu_button_maybe!(
                "close-file",
                is_book_opened.then_some(MenuBarMessage::CloseFile)
            )),
            (widget::rule::horizontal(1)),
            (menu_button!("settings", MenuBarMessage::OpenSettings)),
            (widget::rule::horizontal(1)),
            (menu_button!("exit", MenuBarMessage::Exit)),
        )
        .max_width(180.0f32),
    );
    let show_menu: Item<MenuBarMessage, _, _> = Item::with_menu(
        top_level_menu_button!("show", MenuBarMessage::MenuBtnPressed(MenuType::Show)),
        menu!(
            (menu_button!(
                "background-info-editor",
                MenuBarMessage::OpenBackgroundInfoEditor
            )),
            (menu_button!(
                "bibliography-editor",
                MenuBarMessage::OpenBibliographyEditor
            )),
        )
        .max_width(180.0f32),
    );
    let help_menu: Item<MenuBarMessage, _, _> = Item::with_menu(
        top_level_menu_button!("help", MenuBarMessage::MenuBtnPressed(MenuType::Help)),
        menu!(
            (menu_button!("version-info", MenuBarMessage::OpenVersionInfoWindow)),
            (menu_button!("license-info", MenuBarMessage::OpenLicenseInfoWindow))
        )
        .max_width(180.0f32),
    );
    let menu_bar = menu_bar![file_menu, show_menu, help_menu]
        .style(|t, s| {
            let style = menu::primary(t, s);
            menu::Style {
                menu_border: Border {
                    radius: Radius::new(0),
                    ..style.menu_border
                },
                ..style
            }
        })
        .close_on_item_click_global(true);
    widget::column![menu_bar, widget::rule::horizontal(1)]
        .width(Length::Fill)
        .into()
}

pub fn menubar_handler(
    main_window: &mut MainWindow,
    msg: MenuBarMessage,
) -> Task<MainWindowMessage> {
    match msg {
        MenuBarMessage::MenuBtnPressed(_) => {}
        MenuBarMessage::NewFile => {
            return Task::done(MainWindowMessage::BookOpener(BookOpenerMessage::New));
        }
        MenuBarMessage::OpenFile => {
            return Task::done(MainWindowMessage::BookOpener(BookOpenerMessage::Open));
        }
        MenuBarMessage::CloseFile => {
            main_window.is_book_opened = false;
            return Task::future(async {
                close_book_err_handled().await;
                MainWindowMessage::ChangePage(MainWindowPageId::BookNotOpened)
            });
        }
        MenuBarMessage::OpenSettings => return Task::done(MainWindowMessage::OpenSettingWindow),
        MenuBarMessage::OpenBackgroundInfoEditor => {
            return Task::done(MainWindowMessage::OpenBackgroundInfoEditorWindow);
        }
        MenuBarMessage::OpenBibliographyEditor => {
            return Task::done(MainWindowMessage::OpenBibliographyEditorWindow);
        }
        MenuBarMessage::OpenVersionInfoWindow => {
            return Task::done(MainWindowMessage::OpenVersionInfoWindow);
        }
        MenuBarMessage::Exit => return iced::exit(),
        MenuBarMessage::OpenLicenseInfoWindow => {
            return Task::done(MainWindowMessage::OpenLicenseInfoWindow);
        }
    }
    Task::none()
}

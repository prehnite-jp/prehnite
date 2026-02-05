mod book_opener;
mod menubar;
pub mod page;

use crate::app::window::main_window::book_opener::book_opener_handler;
use crate::app::window::main_window::menubar::{menubar, menubar_handler, MenuBarMessage};
use crate::app::window::main_window::page::book_not_opened::BookNotOpened;
use crate::app::window::main_window::page::item_list::{ItemListActions, ItemListMessage};
use crate::app::window::main_window::page::{MainWindowPage, MainWindowPageId};
use crate::app::window::{Window, WindowMessage};
use crate::util::app_version_info;
use iced::futures::FutureExt;
use iced::{window, Element, Task};
use prehnite_core::db::{acquire_err_handled, open_book_err_handled, query, DBType};
use prehnite_core::i18n::i18n_w;
use prehnite_core::settings::SettingKey;
use prehnite_core::util::alert::UnwrapOrErrorAlert;
use prehnite_core::util::file_dialog::FileOpe;
use tracing::error;

impl Into<FileOpe> for BookOpenerMessage {
    fn into(self) -> FileOpe {
        match self {
            BookOpenerMessage::New => FileOpe::New,
            _ => FileOpe::Open,
        }
    }
}

#[derive(Clone, Debug)]
pub enum BookOpenerMessage {
    Open,
    New,
    Opened,
    NotOpened,
}

#[derive(Clone, Debug)]
pub enum MainWindowMessage {
    BookOpener(BookOpenerMessage),
    ChangePage(MainWindowPageId),
    ItemList(ItemListMessage),
    MenuBar(MenuBarMessage),
    BookOpened,
    OpenVersionInfoWindow,
}

#[derive(Debug)]
pub struct MainWindow {
    page: MainWindowPage,
    is_book_opened: bool,
    window_id: Option<window::Id>,
}

impl MainWindow {
    #[tracing::instrument]
    async fn open_last_opened_book() -> MainWindowMessage {
        let mut conn = acquire_err_handled(DBType::AppGlobal)
            .await
            .unwrap_or_alert();
        let last_opened = query::fetch_setting(&mut conn, SettingKey::GLastOpened)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to fetch last opened settings. Error: {:#?}", e);
                None
            });
        fn return_err() -> MainWindowMessage {
            MainWindowMessage::ChangePage(MainWindowPageId::BookNotOpened)
        }
        match last_opened
            .and_then(|v| v.setting_value)
            .and_then(|v| v.parse().ok())
        {
            None => return_err(),
            Some(v) => {
                if open_book_err_handled(v).await {
                    MainWindowMessage::BookOpened
                } else {
                    return_err()
                }
            }
        }
    }

    fn update_impl(&mut self, message: MainWindowMessage) -> Task<MainWindowMessage> {
        match message {
            MainWindowMessage::BookOpener(msg) => return book_opener_handler(self, msg),
            MainWindowMessage::ItemList(msg) => {
                if let MainWindowPage::ItemList(page) = &mut self.page {
                    return match page.update(msg) {
                        ItemListActions::Run(v) => v.map(MainWindowMessage::ItemList),
                    };
                }
                error!("invalid message received.");
            }
            MainWindowMessage::ChangePage(page) => {
                self.page = page.clone().into();
                match page {
                    MainWindowPageId::ItemList => {
                        return Task::done(MainWindowMessage::ItemList(ItemListMessage::LoadItems));
                    }
                    _ => {}
                }
            }
            MainWindowMessage::MenuBar(v) => return menubar_handler(self, v),
            MainWindowMessage::BookOpened => {
                self.is_book_opened = true;
                return Task::done(MainWindowMessage::ChangePage(MainWindowPageId::ItemList));
            }
            MainWindowMessage::OpenVersionInfoWindow => { /* handled by daemon*/ }
        }
        Task::none()
    }

    fn view_impl(&self) -> Element<'_, MainWindowMessage> {
        Element::from(iced::widget::column![
            menubar(self.is_book_opened).map(MainWindowMessage::MenuBar),
            match &self.page {
                MainWindowPage::NowLoading => i18n_w("now-loading").into(),
                MainWindowPage::BookNotOpened =>
                    BookNotOpened::view().map(MainWindowMessage::BookOpener),
                MainWindowPage::ItemList(v) => v.view().map(MainWindowMessage::ItemList),
            }
        ])
    }
}

impl Window for MainWindow {
    fn new() -> Self {
        Self {
            page: Default::default(),
            is_book_opened: false,
            window_id: None,
        }
    }

    fn init_task() -> Task<WindowMessage> {
        Task::future(Self::open_last_opened_book().map(WindowMessage::MainWindowMessage))
    }

    #[tracing::instrument]
    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage> {
        if let WindowMessage::MainWindowMessage(message) = message {
            self.update_impl(message)
                .map(WindowMessage::MainWindowMessage)
        } else {
            error!("Invalid message received.");
            Task::none()
        }
    }

    fn view(&'_ self) -> Element<'_, WindowMessage> {
        self.view_impl().map(WindowMessage::MainWindowMessage)
    }

    fn title(&'_ self) -> String {
        app_version_info()
    }

    fn set_window_id(&mut self, window_id: window::Id) {
        self.window_id = Some(window_id);
    }
}

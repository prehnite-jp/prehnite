use crate::app::page::background_info_editor::{
    BackgroundInfoEditorActions, BackgroundInfoEditorMessage,
};
use crate::app::page::book_not_opened::BookNotOpened;
use crate::app::page::draft_editor::{DraftEditorActions, DraftEditorMessage};
use crate::app::page::headline_editor::{HeadlineEditorActions, HeadlineEditorMessage};
use crate::app::page::item_list::{ItemListActions, ItemListMessage};
use crate::app::page::paragraph_editor::{ParagraphEditorActions, ParagraphEditorMessage};
use crate::app::page::{PrehnitePage, PrehnitePageId};
use crate::app::window::menubar::{menubar, MenuBarMessage, MenuType};
use crate::app::window::{Window, WindowMessage};
use crate::util::app_version_info;
use iced::futures::FutureExt;
use iced::{window, Element, Task};
use prehnite_core::db::{
    acquire_err_handled, close_book_err_handled, open_book_err_handled, query, DBType,
};
use prehnite_core::i18n::i18n_w;
use prehnite_core::settings::SettingKey;
use prehnite_core::util::alert::UnwrapOrErrorAlert;
use prehnite_core::util::file_dialog::{select_and_open_prehnite_book_file, FileOpe};
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
    ChangePage(PrehnitePageId),
    BgInfoEditor(BackgroundInfoEditorMessage),
    DraftEditor(DraftEditorMessage),
    HeadlineEditor(HeadlineEditorMessage),
    ItemList(ItemListMessage),
    ParagraphEditor(ParagraphEditorMessage),
    MenuBar(MenuBarMessage),
    BookOpened,
    OpenVersionInfoWindow,
}

#[derive(Debug)]
pub struct MainWindow {
    page: PrehnitePage,
    is_book_opened: bool,
    window_id: Option<window::Id>,
}

impl MainWindow {
    #[tracing::instrument]
    async fn open_last_opened_book() -> MainWindowMessage {
        let mut conn = acquire_err_handled(DBType::AppGlobal)
            .await
            .unwrap_or_alert();
        let last_opened = query::fetch_setting(&mut conn, SettingKey::LastOpened)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to fetch last opened settings. Error: {:#?}", e);
                None
            });
        fn return_err() -> MainWindowMessage {
            MainWindowMessage::ChangePage(PrehnitePageId::BookNotOpened)
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

    pub fn book_opener(&self, msg: BookOpenerMessage) -> Task<BookOpenerMessage> {
        select_and_open_prehnite_book_file(self.window_id.unwrap(), msg.into()).map(|v| {
            if v.is_success() {
                BookOpenerMessage::Opened
            } else {
                BookOpenerMessage::NotOpened
            }
        })
    }

    fn update_impl(&mut self, message: MainWindowMessage) -> Task<MainWindowMessage> {
        match message {
            MainWindowMessage::BookOpener(msg) => {
                match msg {
                    BookOpenerMessage::Open | BookOpenerMessage::New => {
                        return self.book_opener(msg).map(MainWindowMessage::BookOpener);
                    }
                    BookOpenerMessage::Opened => return Task::done(MainWindowMessage::BookOpened),
                    BookOpenerMessage::NotOpened => {}
                };
            }
            MainWindowMessage::BgInfoEditor(msg) => {
                let page = crate::unwrap_page!(self, PrehnitePage::BgInfoEditor);
                match page.update(msg) {
                    BackgroundInfoEditorActions::None => {}
                }
            }
            MainWindowMessage::DraftEditor(msg) => {
                let page = crate::unwrap_page!(self, PrehnitePage::DraftEditor);
                match page.update(msg) {
                    DraftEditorActions::None => {}
                }
            }
            MainWindowMessage::HeadlineEditor(msg) => {
                let page = crate::unwrap_page!(self, PrehnitePage::HeadlineEditor);
                match page.update(msg) {
                    HeadlineEditorActions::None => {}
                }
            }
            MainWindowMessage::ItemList(msg) => {
                let page = crate::unwrap_page!(self, PrehnitePage::ItemList);
                return match page.update(msg) {
                    ItemListActions::Run(v) => v.map(MainWindowMessage::ItemList),
                };
            }
            MainWindowMessage::ParagraphEditor(msg) => {
                let page = crate::unwrap_page!(self, PrehnitePage::ParagraphEditor);
                match page.update(msg) {
                    ParagraphEditorActions::None => {}
                }
            }
            MainWindowMessage::ChangePage(page) => {
                self.page = page.clone().into();
                match page {
                    PrehnitePageId::NowLoading => {}
                    PrehnitePageId::BookNotOpened => {}
                    PrehnitePageId::BgInfoEditor => {}
                    PrehnitePageId::DraftEditor => {}
                    PrehnitePageId::HeadlineEditor => {}
                    PrehnitePageId::ItemList => {
                        return Task::done(MainWindowMessage::ItemList(ItemListMessage::LoadItems));
                    }
                    PrehnitePageId::ParagraphEditor => {}
                }
            }
            MainWindowMessage::MenuBar(v) => match v {
                MenuBarMessage::MenuBtnPressed(menu_type) => match menu_type {
                    MenuType::File => {}
                    MenuType::Show => {}
                    MenuType::Help => {}
                },
                MenuBarMessage::NewFile => {
                    return Task::done(MainWindowMessage::BookOpener(BookOpenerMessage::New));
                }
                MenuBarMessage::OpenFile => {
                    return Task::done(MainWindowMessage::BookOpener(BookOpenerMessage::Open));
                }
                MenuBarMessage::CloseFile => {
                    self.is_book_opened = false;
                    return Task::future(async {
                        close_book_err_handled().await;
                        MainWindowMessage::ChangePage(PrehnitePageId::BookNotOpened)
                    });
                }
                MenuBarMessage::OpenSettings => {}
                MenuBarMessage::OpenBackgroundInfoEditor => {}
                MenuBarMessage::OpenBibliographyEditor => {}
                MenuBarMessage::OpenVersionInfoWindow => {
                    return Task::done(MainWindowMessage::OpenVersionInfoWindow);
                }
                MenuBarMessage::Exit => {
                    return iced::exit();
                }
            },
            MainWindowMessage::BookOpened => {
                self.is_book_opened = true;
                return Task::done(MainWindowMessage::ChangePage(PrehnitePageId::ItemList));
            }
            MainWindowMessage::OpenVersionInfoWindow => {}
        }
        Task::none()
    }

    fn view_impl(&self) -> Element<'_, MainWindowMessage> {
        Element::from(iced::widget::column![
            menubar(self.is_book_opened).map(MainWindowMessage::MenuBar),
            match &self.page {
                PrehnitePage::NowLoading => i18n_w("now-loading").into(),
                PrehnitePage::BookNotOpened =>
                    BookNotOpened::view().map(MainWindowMessage::BookOpener),
                PrehnitePage::BgInfoEditor(v) => v.view().map(MainWindowMessage::BgInfoEditor),
                PrehnitePage::DraftEditor(v) => v.view().map(MainWindowMessage::DraftEditor),
                PrehnitePage::HeadlineEditor(v) => v.view().map(MainWindowMessage::HeadlineEditor),
                PrehnitePage::ItemList(v) => v.view().map(MainWindowMessage::ItemList),
                PrehnitePage::ParagraphEditor(v) =>
                    v.view().map(MainWindowMessage::ParagraphEditor),
            }
        ])
    }
}

impl Window for MainWindow {
    fn new() -> (Box<dyn Window>, Task<WindowMessage>) {
        (
            Box::new(Self {
                page: Default::default(),
                is_book_opened: false,
                window_id: None,
            }),
            Task::future(Self::open_last_opened_book().map(WindowMessage::MainWindowMessage)),
        )
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

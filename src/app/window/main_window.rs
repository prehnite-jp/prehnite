use crate::app::menubar::{menubar, MenuBarMessage, MenuType};
use crate::app::page::background_info_editor::BackgroundInfoEditorActions;
use crate::app::page::book_not_opened::{BookNotOpenedActions, BookNotOpenedMessage};
use crate::app::page::draft_editor::DraftEditorActions;
use crate::app::page::headline_editor::HeadlineEditorActions;
use crate::app::page::item_list::{ItemListActions, ItemListMessage};
use crate::app::page::paragraph_editor::ParagraphEditorActions;
use crate::app::page::{PrehnitePage, PrehnitePageId};
use crate::app::window::{Window, WindowMessage};
use crate::util::app_version_info;
use iced::{Element, Task};
use prehnite_core::db::{
    acquire_err_handled, close_book_err_handled, open_book_err_handled, query, DBType,
};
use prehnite_core::i18n::{i18n, i18n_w};
use prehnite_core::settings::SettingKey;
use prehnite_core::util::alert::{alert_info_spawn, UnwrapOrErrorAlert};
use tracing::error;

#[derive(Debug)]
pub struct MainWindow {
    page: PrehnitePage,
    is_book_opened: bool,
}

impl MainWindow {
    #[tracing::instrument]
    async fn open_last_opened_book() -> WindowMessage {
        let mut conn = acquire_err_handled(DBType::AppGlobal)
            .await
            .unwrap_or_alert();
        let last_opened = query::fetch_setting(&mut conn, SettingKey::LastOpened)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to fetch last opened settings. Error: {:#?}", e);
                None
            });
        fn return_err() -> WindowMessage {
            WindowMessage::ChangePage(PrehnitePageId::BookNotOpened)
        }
        match last_opened
            .and_then(|v| v.setting_value)
            .and_then(|v| v.parse().ok())
        {
            None => return_err(),
            Some(v) => {
                if open_book_err_handled(v).await {
                    WindowMessage::BookOpened
                } else {
                    return_err()
                }
            }
        }
    }
}

impl Window for MainWindow {
    fn new() -> (Self, Task<WindowMessage>) {
        (
            Self {
                page: Default::default(),
                is_book_opened: false,
            },
            Task::future(Self::open_last_opened_book()),
        )
    }

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage> {
        match message {
            WindowMessage::None => {}
            WindowMessage::BookNotOpened(msg) => {
                let page = crate::unwrap_page!(self, PrehnitePage::BookNotOpened);
                match page.update(msg) {
                    BookNotOpenedActions::Run(v) => return v.map(WindowMessage::BookNotOpened),
                    BookNotOpenedActions::Opened => return Task::done(WindowMessage::BookOpened),
                    BookNotOpenedActions::NotOpened => {}
                };
            }
            WindowMessage::BgInfoEditor(msg) => {
                let page = crate::unwrap_page!(self, PrehnitePage::BgInfoEditor);
                match page.update(msg) {
                    BackgroundInfoEditorActions::None => {}
                }
            }
            WindowMessage::DraftEditor(msg) => {
                let page = crate::unwrap_page!(self, PrehnitePage::DraftEditor);
                match page.update(msg) {
                    DraftEditorActions::None => {}
                }
            }
            WindowMessage::HeadlineEditor(msg) => {
                let page = crate::unwrap_page!(self, PrehnitePage::HeadlineEditor);
                match page.update(msg) {
                    HeadlineEditorActions::None => {}
                }
            }
            WindowMessage::ItemList(msg) => {
                let page = crate::unwrap_page!(self, PrehnitePage::ItemList);
                return match page.update(msg) {
                    ItemListActions::Run(v) => v.map(WindowMessage::ItemList),
                };
            }
            WindowMessage::ParagraphEditor(msg) => {
                let page = crate::unwrap_page!(self, PrehnitePage::ParagraphEditor);
                match page.update(msg) {
                    ParagraphEditorActions::None => {}
                }
            }
            WindowMessage::ChangePage(page) => {
                self.page = page.clone().into();
                match page {
                    PrehnitePageId::NowLoading => {}
                    PrehnitePageId::BookNotOpened => {}
                    PrehnitePageId::BgInfoEditor => {}
                    PrehnitePageId::DraftEditor => {}
                    PrehnitePageId::HeadlineEditor => {}
                    PrehnitePageId::ItemList => {
                        return Task::done(WindowMessage::ItemList(ItemListMessage::LoadItems));
                    }
                    PrehnitePageId::ParagraphEditor => {}
                }
            }
            WindowMessage::MenuBar(v) => match v {
                MenuBarMessage::MenuBtnPressed(menu_type) => match menu_type {
                    MenuType::File => {}
                    MenuType::Show => {}
                    MenuType::Help => {}
                },
                MenuBarMessage::NewFile => {
                    return Task::done(WindowMessage::BookNotOpened(BookNotOpenedMessage::New));
                }
                MenuBarMessage::OpenFile => {
                    return Task::done(WindowMessage::BookNotOpened(BookNotOpenedMessage::Open));
                }
                MenuBarMessage::CloseFile => {
                    self.is_book_opened = false;
                    return Task::future(async {
                        close_book_err_handled().await;
                        WindowMessage::ChangePage(PrehnitePageId::BookNotOpened)
                    });
                }
                MenuBarMessage::OpenSettings => {}
                MenuBarMessage::OpenBackgroundInfoEditor => {}
                MenuBarMessage::OpenBibliographyEditor => {}
                MenuBarMessage::OpenVersionInfoDialog => {
                    return Task::future(async {
                        alert_info_spawn((
                            i18n("version-info").as_str(),
                            app_version_info().as_str(),
                        ))
                        .await;
                        WindowMessage::None
                    });
                }
                MenuBarMessage::Exit => {
                    return iced::exit();
                }
            },
            WindowMessage::BookOpened => {
                self.is_book_opened = true;
                return Task::done(WindowMessage::ChangePage(PrehnitePageId::ItemList));
            },
        }
        Task::none()
    }

    fn view(&'_ self) -> Element<'_, WindowMessage> {
        iced::widget::column![
            menubar(self.is_book_opened).map(WindowMessage::MenuBar),
            match &self.page {
                PrehnitePage::NowLoading => i18n_w("now-loading").into(),
                PrehnitePage::BookNotOpened(v) => v.view().map(WindowMessage::BookNotOpened),
                PrehnitePage::BgInfoEditor(v) => v.view().map(WindowMessage::BgInfoEditor),
                PrehnitePage::DraftEditor(v) => v.view().map(WindowMessage::DraftEditor),
                PrehnitePage::HeadlineEditor(v) => v.view().map(WindowMessage::HeadlineEditor),
                PrehnitePage::ItemList(v) => v.view().map(WindowMessage::ItemList),
                PrehnitePage::ParagraphEditor(v) => v.view().map(WindowMessage::ParagraphEditor),
            }
        ]
        .into()
    }

    fn title(&'_ self) -> String {
        app_version_info()
    }
}

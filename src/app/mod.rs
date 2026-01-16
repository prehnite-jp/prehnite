mod page;

use crate::app::page::background_info_editor::{
    BackgroundInfoEditorActions, BackgroundInfoEditorMessage,
};
use crate::app::page::book_not_opened::{BookNotOpenedActions, BookNotOpenedMessage};
use crate::app::page::draft_editor::{DraftEditorActions, DraftEditorMessage};
use crate::app::page::headline_editor::{HeadlineEditorActions, HeadlineEditorMessage};
use crate::app::page::item_list::{ItemList, ItemListActions, ItemListMessage};
use crate::app::page::paragraph_editor::{ParagraphEditorActions, ParagraphEditorMessage};
use crate::app::page::{PrehnitePage, PrehnitePageId};
use iced::widget::text;
use iced::{Element, Task};
use prehnite_core::db::{acquire_err_handled, open_book_err_handled, query, DBType};
use prehnite_core::i18n::i18n;
use prehnite_core::settings::SettingKey;
use prehnite_core::util::alert::UnwrapOrErrorAlert;
use tracing::error;

#[derive(Debug)]
pub struct PrehniteApp {
    page: PrehnitePage,
}

#[derive(Clone, Debug)]
pub enum RootMessage {
    ChangePage(PrehnitePageId),
    BookNotOpened(BookNotOpenedMessage),
    BgInfoEditor(BackgroundInfoEditorMessage),
    DraftEditor(DraftEditorMessage),
    HeadlineEditor(HeadlineEditorMessage),
    ItemList(ItemListMessage),
    ParagraphEditor(ParagraphEditorMessage),
}

macro_rules! unwrap_page {
    ($self: ident, $x:path) => {{
        match &mut $self.page {
            $x(page) => page,
            _ => {
                error!("invalid message received.");
                return Task::none();
            }
        }
    }};
}

impl PrehniteApp {
    pub fn run() -> Result<(), iced::Error> {
        iced::application(Self::new, Self::update, Self::view).run()
    }

    #[tracing::instrument]
    async fn open_last_opened_book() -> RootMessage {
        let mut conn = acquire_err_handled(DBType::AppGlobal)
            .await
            .unwrap_or_alert();
        let last_opened = query::fetch_setting(&mut conn, SettingKey::LastOpened)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to fetch last opened settings. Error: {:#?}", e);
                None
            });
        fn return_err() -> RootMessage {
            RootMessage::ChangePage(PrehnitePageId::BookNotOpened)
        }
        match last_opened
            .and_then(|v| v.setting_value)
            .and_then(|v| v.parse().ok())
        {
            None => return_err(),
            Some(v) => {
                if open_book_err_handled(v).await {
                    RootMessage::ChangePage(PrehnitePageId::ItemList)
                } else {
                    return_err()
                }
            }
        }
    }

    fn new() -> (Self, Task<RootMessage>) {
        (
            Self {
                page: Default::default(),
            },
            Task::future(Self::open_last_opened_book()),
        )
    }

    #[tracing::instrument]
    fn update(&mut self, message: RootMessage) -> Task<RootMessage> {
        match message {
            RootMessage::BookNotOpened(msg) => {
                let page = unwrap_page!(self, PrehnitePage::BookNotOpened);
                match page.update(msg) {
                    BookNotOpenedActions::Run(v) => return v.map(RootMessage::BookNotOpened),
                    BookNotOpenedActions::Opened => {
                        return Task::done(RootMessage::ChangePage(PrehnitePageId::ItemList));
                    }
                    BookNotOpenedActions::NotOpened => {}
                };
            }
            RootMessage::BgInfoEditor(msg) => {
                let page = unwrap_page!(self, PrehnitePage::BgInfoEditor);
                match page.update(msg) {
                    BackgroundInfoEditorActions::None => {}
                }
            }
            RootMessage::DraftEditor(msg) => {
                let page = unwrap_page!(self, PrehnitePage::DraftEditor);
                match page.update(msg) {
                    DraftEditorActions::None => {}
                }
            }
            RootMessage::HeadlineEditor(msg) => {
                let page = unwrap_page!(self, PrehnitePage::HeadlineEditor);
                match page.update(msg) {
                    HeadlineEditorActions::None => {}
                }
            }
            RootMessage::ItemList(msg) => {
                let page = unwrap_page!(self, PrehnitePage::ItemList);
                return match page.update(msg) {
                    ItemListActions::Run(v) => v.map(RootMessage::ItemList),
                };
            }
            RootMessage::ParagraphEditor(msg) => {
                let page = unwrap_page!(self, PrehnitePage::ParagraphEditor);
                match page.update(msg) {
                    ParagraphEditorActions::None => {}
                }
            }
            RootMessage::ChangePage(page) => {
                self.page = page.clone().into();
                match page {
                    PrehnitePageId::NowLoading => {}
                    PrehnitePageId::BookNotOpened => {}
                    PrehnitePageId::BgInfoEditor => {}
                    PrehnitePageId::DraftEditor => {}
                    PrehnitePageId::HeadlineEditor => {}
                    PrehnitePageId::ItemList => {
                        return Task::done(RootMessage::ItemList(ItemListMessage::LoadItems));
                    }
                    PrehnitePageId::ParagraphEditor => {}
                }
            }
        }
        Task::none()
    }

    #[tracing::instrument]
    fn view(&'_ self) -> Element<'_, RootMessage> {
        match &self.page {
            PrehnitePage::NowLoading => text(i18n("now-loading")).into(),
            PrehnitePage::BookNotOpened(v) => v.view().map(RootMessage::BookNotOpened),
            PrehnitePage::BgInfoEditor(v) => v.view().map(RootMessage::BgInfoEditor),
            PrehnitePage::DraftEditor(v) => v.view().map(RootMessage::DraftEditor),
            PrehnitePage::HeadlineEditor(v) => v.view().map(RootMessage::HeadlineEditor),
            PrehnitePage::ItemList(v) => v.view().map(RootMessage::ItemList),
            PrehnitePage::ParagraphEditor(v) => v.view().map(RootMessage::ParagraphEditor),
        }
    }
}

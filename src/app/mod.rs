mod page;

use crate::app::page::background_info_editor::{
    BackgroundInfoEditorActions, BackgroundInfoEditorMessage,
};
use crate::app::page::book_not_opened::{BookNotOpenedActions, BookNotOpenedMessage};
use crate::app::page::draft_editor::{DraftEditorActions, DraftEditorMessage};
use crate::app::page::headline_editor::{HeadlineEditorActions, HeadlineEditorMessage};
use crate::app::page::item_list::{ItemListActions, ItemListMessage};
use crate::app::page::paragraph_editor::{ParagraphEditorActions, ParagraphEditorMessage};
use crate::app::page::PrehnitePage;
use crate::util::book_opener::{BookOpe, BookOpenerMessage};
use iced::{Element, Task};
use prehnite_core::db::schema::Setting;
use prehnite_core::db::{acquire_err_handled, DBType};
use prehnite_core::settings::SettingKey;
use prehnite_core::util::alert::UnwrapOrErrorAlert;
use std::path::PathBuf;
use tracing::error;

#[derive(Debug)]
pub struct PrehniteApp {
    page: PrehnitePage,
}

#[derive(Clone, Debug)]
pub enum RootMessage {
    BookNotOpened(BookNotOpenedMessage),
    BackgroundInfoEditor(BackgroundInfoEditorMessage),
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
        let last_opened = Setting::fetch_setting(&mut conn, SettingKey::LastOpened)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to fetch last opened settings. Error: {:#?}", e);
                None
            });
        let path: Option<PathBuf> = match last_opened {
            None => None,
            Some(v) => v.parse().ok(),
        };
        RootMessage::BookNotOpened(BookOpenerMessage::BookSelected((BookOpe::Open, path)).into())
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
                    BookNotOpenedActions::BookOpener(opener_msg) => {
                        return opener_msg.map(RootMessage::BookNotOpened);
                    }
                    BookNotOpenedActions::BookOpened => {
                        self.page = PrehnitePage::ItemList(Default::default());
                    }
                }
            }
            RootMessage::BackgroundInfoEditor(msg) => {
                let page = unwrap_page!(self, PrehnitePage::BackgroundInfoEditor);
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
                match page.update(msg) {
                    ItemListActions::None => {}
                }
            }
            RootMessage::ParagraphEditor(msg) => {
                let page = unwrap_page!(self, PrehnitePage::ParagraphEditor);
                match page.update(msg) {
                    ParagraphEditorActions::None => {}
                }
            }
        }
        Task::none()
    }

    #[tracing::instrument]
    fn view(&'_ self) -> Element<'_, RootMessage> {
        match &self.page {
            PrehnitePage::BookNotOpened(v) => v.view().map(RootMessage::BookNotOpened),
            PrehnitePage::BackgroundInfoEditor(v) => {
                v.view().map(RootMessage::BackgroundInfoEditor)
            }
            PrehnitePage::DraftEditor(v) => v.view().map(RootMessage::DraftEditor),
            PrehnitePage::HeadlineEditor(v) => v.view().map(RootMessage::HeadlineEditor),
            PrehnitePage::ItemList(v) => v.view().map(RootMessage::ItemList),
            PrehnitePage::ParagraphEditor(v) => v.view().map(RootMessage::ParagraphEditor),
        }
    }
}

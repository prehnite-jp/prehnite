mod page;

use crate::app::page::background_info_editor::{
    BackgroundInfoEditorActions, BackgroundInfoEditorMessage,
};
use crate::app::page::book_not_opened::{
    BookNotOpened, BookNotOpenedActions, BookNotOpenedMessage,
};
use crate::app::page::draft_editor::{DraftEditorActions, DraftEditorMessage};
use crate::app::page::headline_editor::{HeadlineEditorActions, HeadlineEditorMessage};
use crate::app::page::item_list::{ItemListActions, ItemListMessage};
use crate::app::page::paragraph_editor::{ParagraphEditorActions, ParagraphEditorMessage};
use crate::app::page::PrehnitePage;
use iced::{Element, Task};
use tracing::error;

#[derive(Debug)]
pub struct PrehniteApp {
    page: PrehnitePage,
}

#[derive(Clone, Debug)]
pub enum RootMessage {
    None,
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

    fn new() -> (Self, Task<RootMessage>) {
        (
            Self {
                page: Default::default(),
            },
            Task::none(),
        )
    }

    #[tracing::instrument]
    fn update(&mut self, message: RootMessage) -> Task<RootMessage> {
        match message {
            RootMessage::None => {}
            RootMessage::BookNotOpened(msg) => {
                let page = unwrap_page!(self, PrehnitePage::BookNotOpened);
                match page.update(msg) {
                    BookNotOpenedActions::BookOpener(opener_msg) => {
                        return opener_msg.map(RootMessage::BookNotOpened);
                    }
                    BookNotOpenedActions::None => {}
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

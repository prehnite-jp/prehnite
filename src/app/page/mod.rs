use crate::app::page::background_info_editor::BackgroundInfoEditor;
use crate::app::page::book_not_opened::BookNotOpened;
use crate::app::page::draft_editor::DraftEditor;
use crate::app::page::headline_editor::HeadlineEditor;
use crate::app::page::item_list::ItemList;
use crate::app::page::paragraph_editor::ParagraphEditor;

pub mod background_info_editor;
pub mod book_not_opened;
pub mod draft_editor;
pub mod headline_editor;
pub mod item_list;
pub mod paragraph_editor;

#[derive(Clone, Debug)]
pub enum PrehnitePageId {
    NowLoading,
    BookNotOpened,
    BgInfoEditor,
    DraftEditor,
    HeadlineEditor,
    ItemList,
    ParagraphEditor,
}

#[derive(Debug, Clone, Default)]
pub enum PrehnitePage {
    #[default]
    NowLoading,
    BookNotOpened(BookNotOpened),
    BgInfoEditor(BackgroundInfoEditor),
    DraftEditor(DraftEditor),
    HeadlineEditor(HeadlineEditor),
    ItemList(ItemList),
    ParagraphEditor(ParagraphEditor),
}

impl From<PrehnitePageId> for PrehnitePage {
    fn from(value: PrehnitePageId) -> Self {
        match value {
            PrehnitePageId::NowLoading => PrehnitePage::NowLoading,
            PrehnitePageId::BookNotOpened => PrehnitePage::BookNotOpened(Default::default()),
            PrehnitePageId::BgInfoEditor => PrehnitePage::BgInfoEditor(Default::default()),
            PrehnitePageId::DraftEditor => PrehnitePage::DraftEditor(Default::default()),
            PrehnitePageId::HeadlineEditor => PrehnitePage::HeadlineEditor(Default::default()),
            PrehnitePageId::ItemList => PrehnitePage::ItemList(Default::default()),
            PrehnitePageId::ParagraphEditor => PrehnitePage::ParagraphEditor(Default::default()),
        }
    }
}

impl From<PrehnitePage> for PrehnitePageId {
    fn from(value: PrehnitePage) -> Self {
        match value {
            PrehnitePage::NowLoading => PrehnitePageId::NowLoading,
            PrehnitePage::BookNotOpened(_) => PrehnitePageId::BookNotOpened,
            PrehnitePage::BgInfoEditor(_) => PrehnitePageId::BgInfoEditor,
            PrehnitePage::DraftEditor(_) => PrehnitePageId::DraftEditor,
            PrehnitePage::HeadlineEditor(_) => PrehnitePageId::HeadlineEditor,
            PrehnitePage::ItemList(_) => PrehnitePageId::ItemList,
            PrehnitePage::ParagraphEditor(_) => PrehnitePageId::ParagraphEditor,
        }
    }
}

#[macro_export]
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

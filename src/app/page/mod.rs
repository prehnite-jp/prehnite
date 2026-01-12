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

#[derive(Debug, Clone)]
pub enum PrehnitePage {
    NowLoading,
    BookNotOpened(BookNotOpened),
    BgInfoEditor(BackgroundInfoEditor),
    DraftEditor(DraftEditor),
    HeadlineEditor(HeadlineEditor),
    ItemList(ItemList),
    ParagraphEditor(ParagraphEditor),
}

impl Default for PrehnitePage {
    fn default() -> Self {
        PrehnitePage::NowLoading
    }
}

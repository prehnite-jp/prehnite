pub mod main_window;

use crate::app::menubar::MenuBarMessage;
use crate::app::page::background_info_editor::BackgroundInfoEditorMessage;
use crate::app::page::book_not_opened::BookNotOpenedMessage;
use crate::app::page::draft_editor::DraftEditorMessage;
use crate::app::page::headline_editor::HeadlineEditorMessage;
use crate::app::page::item_list::ItemListMessage;
use crate::app::page::paragraph_editor::ParagraphEditorMessage;
use crate::app::page::PrehnitePageId;
use iced::{Element, Task};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum WindowMessage {
    None,
    ChangePage(PrehnitePageId),
    BookNotOpened(BookNotOpenedMessage),
    BgInfoEditor(BackgroundInfoEditorMessage),
    DraftEditor(DraftEditorMessage),
    HeadlineEditor(HeadlineEditorMessage),
    ItemList(ItemListMessage),
    ParagraphEditor(ParagraphEditorMessage),
    MenuBar(MenuBarMessage),
    BookOpened,
}

pub trait Window: Debug {
    fn new() -> (Self, Task<WindowMessage>)
    where
        Self: Sized;

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage>;

    fn view(&'_ self) -> Element<'_, WindowMessage>;

    fn title(&'_ self) -> String;
}

use crate::app::window::main_window::page::item_list::ItemList;

pub mod book_not_opened;
pub mod item_list;

#[derive(Clone, Debug)]
pub enum MainWindowPageId {
    NowLoading,
    BookNotOpened,
    ItemList,
}

#[derive(Debug, Clone, Default)]
pub enum MainWindowPage {
    #[default]
    NowLoading,
    BookNotOpened,
    ItemList(ItemList),
}

impl From<MainWindowPageId> for MainWindowPage {
    fn from(value: MainWindowPageId) -> Self {
        match value {
            MainWindowPageId::NowLoading => MainWindowPage::NowLoading,
            MainWindowPageId::BookNotOpened => MainWindowPage::BookNotOpened,
            MainWindowPageId::ItemList => MainWindowPage::ItemList(Default::default()),
        }
    }
}

impl From<MainWindowPage> for MainWindowPageId {
    fn from(value: MainWindowPage) -> Self {
        match value {
            MainWindowPage::NowLoading => MainWindowPageId::NowLoading,
            MainWindowPage::BookNotOpened => MainWindowPageId::BookNotOpened,
            MainWindowPage::ItemList(_) => MainWindowPageId::ItemList,
        }
    }
}

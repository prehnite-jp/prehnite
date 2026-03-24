use crate::app::window::main_window::page::item_list::ItemList;

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
    ItemList(ItemList),
}

impl From<MainWindowPageId> for MainWindowPage {
    fn from(value: MainWindowPageId) -> Self {
        match value {
            MainWindowPageId::NowLoading => MainWindowPage::NowLoading,
            MainWindowPageId::BookNotOpened => MainWindowPage::ItemList(ItemList::not_opened()),
            MainWindowPageId::ItemList => MainWindowPage::ItemList(Default::default()),
        }
    }
}

impl From<MainWindowPage> for MainWindowPageId {
    fn from(value: MainWindowPage) -> Self {
        match value {
            MainWindowPage::NowLoading => MainWindowPageId::NowLoading,
            MainWindowPage::ItemList(_) => MainWindowPageId::ItemList,
        }
    }
}

use iced::widget::text;
use iced::Element;
use prehnite_core::i18n::i18n;

#[derive(Clone, Debug)]
pub enum ItemListMessage {}

#[derive(Clone)]
pub enum ItemListActions {
    None,
}

#[derive(Debug)]
pub struct ItemList {}

impl ItemList {
    pub fn update(&mut self, msg: ItemListMessage) -> ItemListActions {
        ItemListActions::None
    }

    pub fn view(&self) -> Element<ItemListMessage> {
        text(i18n("wip")).into()
    }
}

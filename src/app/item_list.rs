use iced::{Element, Task};
use iced::widget::text;
use prehnite_core::i18n::i18n;

#[derive(Clone)]
pub enum ItemListMessage {}

#[derive(Clone)]
pub enum ItemListActions {}

#[derive(Debug)]
pub struct ItemList {}

impl ItemList {
    fn new() -> Self {
        Self{}
    }

    fn update(&mut self, msg: ItemListMessage) -> Task<ItemListActions> {
        Task::none()
    }

    fn view(&self) -> Element<ItemListMessage> {
        text(i18n("wip")).into()
    }
}

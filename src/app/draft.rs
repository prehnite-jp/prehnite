use iced::{Element, Task};
use iced::widget::text;
use prehnite_core::i18n::i18n;

#[derive(Clone)]
pub enum DraftMessage {}

#[derive(Clone)]
pub enum DraftActions {}

#[derive(Debug)]
pub struct Draft {}

impl Draft {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, msg: DraftMessage) -> Task<DraftActions> {
        Task::none()
    }

    fn view(&self) -> Element<DraftMessage> {
        text(i18n("wip")).into()
    }
}

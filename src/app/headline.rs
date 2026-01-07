use iced::widget::text;
use iced::{Element, Task};
use prehnite_core::i18n::i18n;

#[derive(Clone)]
pub enum HeadlineMessage {}

#[derive(Clone)]
pub enum HeadlineActions {}

#[derive(Debug)]
pub struct Headline {}

impl Headline {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, msg: HeadlineMessage) -> iced::Task<HeadlineActions> {
        Task::none()
    }

    fn view(&self) -> Element<HeadlineMessage> {
        text(i18n("wip")).into()
    }
}

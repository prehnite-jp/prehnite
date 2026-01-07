use iced::widget::text;
use iced::{Element, Task};
use prehnite_core::i18n::i18n;

#[derive(Clone)]
pub enum ParagraphMessage {}

#[derive(Clone)]
pub enum ParagraphActions {}

#[derive(Debug)]
pub struct Paragraph {}

impl Paragraph {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, msg: ParagraphMessage) -> Task<ParagraphActions> {
        Task::none()
    }

    fn view(&self) -> Element<ParagraphMessage> {
        text(i18n("wip")).into()
    }
}

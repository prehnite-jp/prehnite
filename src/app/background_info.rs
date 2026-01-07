use iced::widget::text;
use iced::{Element, Task};
use prehnite_core::i18n::i18n;

#[derive(Clone)]
pub enum BackgroundInfoMessage {}

#[derive(Clone)]
pub enum BackgroundInfoActions {}

#[derive(Debug)]
pub struct BackgroundInfo {}

impl BackgroundInfo {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, msg: BackgroundInfoMessage) -> Task<BackgroundInfoActions> {
        Task::none()
    }

    fn view(&self) -> Element<BackgroundInfoMessage> {
        text(i18n("wip")).into()
    }
}

use iced::widget::text;
use iced::Element;
use prehnite_core::i18n::i18n;

#[derive(Clone, Debug)]
pub enum BackgroundInfoEditorMessage {}

#[derive(Clone)]
pub enum BackgroundInfoEditorActions {
    None,
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct BackgroundInfoEditor {}

impl BackgroundInfoEditor {
    pub fn update(&mut self, msg: BackgroundInfoEditorMessage) -> BackgroundInfoEditorActions {
        BackgroundInfoEditorActions::None
    }

    pub fn view(&'_ self) -> Element<'_, BackgroundInfoEditorMessage> {
        text(i18n("wip")).into()
    }
}

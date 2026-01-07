use iced::widget::text;
use iced::Element;
use prehnite_core::i18n::i18n;

#[derive(Clone, Debug)]
pub enum ParagraphEditorMessage {}

#[derive(Clone)]
pub enum ParagraphEditorActions {
    None,
}

#[derive(Debug)]
pub struct ParagraphEditor {}

impl ParagraphEditor {
    pub fn update(&mut self, msg: ParagraphEditorMessage) -> ParagraphEditorActions {
        ParagraphEditorActions::None
    }

    pub fn view(&self) -> Element<ParagraphEditorMessage> {
        text(i18n("wip")).into()
    }
}

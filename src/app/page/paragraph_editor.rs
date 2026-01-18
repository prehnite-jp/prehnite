use iced::Element;
use prehnite_core::i18n::i18n_w;

#[derive(Clone, Debug)]
pub enum ParagraphEditorMessage {}

#[derive(Clone)]
pub enum ParagraphEditorActions {
    None,
}

#[derive(Debug, Clone, Default)]
pub struct ParagraphEditor {}

impl ParagraphEditor {
    pub fn update(&mut self, _msg: ParagraphEditorMessage) -> ParagraphEditorActions {
        ParagraphEditorActions::None
    }

    pub fn view(&'_ self) -> Element<'_, ParagraphEditorMessage> {
        i18n_w("wip").into()
    }
}

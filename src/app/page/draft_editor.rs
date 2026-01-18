use iced::Element;
use prehnite_core::i18n::i18n_w;

#[derive(Clone, Debug)]
pub enum DraftEditorMessage {}

#[derive(Clone)]
pub enum DraftEditorActions {
    None,
}

#[derive(Debug, Clone, Default)]
pub struct DraftEditor {}

impl DraftEditor {
    pub fn update(&mut self, _msg: DraftEditorMessage) -> DraftEditorActions {
        DraftEditorActions::None
    }

    pub fn view(&'_ self) -> Element<'_, DraftEditorMessage> {
        i18n_w("wip").into()
    }
}

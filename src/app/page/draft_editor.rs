use iced::widget::text;
use iced::Element;
use prehnite_core::i18n::i18n;

#[derive(Clone, Debug)]
pub enum DraftEditorMessage {}

#[derive(Clone)]
pub enum DraftEditorActions {
    None,
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct DraftEditor {}

impl DraftEditor {
    pub fn update(&mut self, msg: DraftEditorMessage) -> DraftEditorActions {
        DraftEditorActions::None
    }

    pub fn view(&'_ self) -> Element<'_, DraftEditorMessage> {
        text(i18n("wip")).into()
    }
}

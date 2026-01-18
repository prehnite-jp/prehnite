use iced::Element;
use prehnite_core::i18n::i18n_w;

#[derive(Clone, Debug)]
pub enum BackgroundInfoEditorMessage {}

#[derive(Clone)]
pub enum BackgroundInfoEditorActions {
    None,
}

#[derive(Debug, Clone, Default)]
pub struct BackgroundInfoEditor {}

impl BackgroundInfoEditor {
    pub fn update(&mut self, _msg: BackgroundInfoEditorMessage) -> BackgroundInfoEditorActions {
        BackgroundInfoEditorActions::None
    }

    pub fn view(&'_ self) -> Element<'_, BackgroundInfoEditorMessage> {
        i18n_w("wip").into()
    }
}

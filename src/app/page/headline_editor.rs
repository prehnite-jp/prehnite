use iced::Element;
use prehnite_core::i18n::i18n_w;

#[derive(Clone, Debug)]
pub enum HeadlineEditorMessage {}

#[derive(Clone)]
pub enum HeadlineEditorActions {
    None,
}

#[derive(Debug, Clone, Default)]
pub struct HeadlineEditor {}

impl HeadlineEditor {
    pub fn update(&mut self, _msg: HeadlineEditorMessage) -> HeadlineEditorActions {
        HeadlineEditorActions::None
    }

    pub fn view(&'_ self) -> Element<'_, HeadlineEditorMessage> {
        i18n_w("wip").into()
    }
}

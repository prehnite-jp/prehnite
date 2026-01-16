use iced::widget::text;
use iced::Element;
use prehnite_core::i18n::i18n;

#[derive(Clone, Debug)]
pub enum HeadlineEditorMessage {}

#[derive(Clone)]
pub enum HeadlineEditorActions {
    None,
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct HeadlineEditor {}

impl HeadlineEditor {
    pub fn update(&mut self, msg: HeadlineEditorMessage) -> HeadlineEditorActions {
        HeadlineEditorActions::None
    }

    pub fn view(&'_ self) -> Element<'_, HeadlineEditorMessage> {
        text(i18n("wip")).into()
    }
}

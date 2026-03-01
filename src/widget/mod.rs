use iced::widget::{button, space, Button};
use iced::{padding, Element};
use prehnite_font_manager::material_symbol::CONTENT_COPY;
use prehnite_font_manager::widget::material_symbol;

pub(crate) mod styles;

pub fn hideable<'a, T>(element: impl Into<Element<'a, T>>, is_visible: bool) -> Element<'a, T>
where
    T: 'static,
{
    if is_visible {
        element.into()
    } else {
        Element::from(space())
    }
}

pub fn copy_button<'a, T>() -> Button<'a, T> {
    button(material_symbol(CONTENT_COPY))
        .padding(padding::left(1))
        .style(button::text)
}

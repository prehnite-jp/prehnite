use iced::{widget, Font};

pub enum IconFamily {
    MaterialSymbolsOutlined,
}

impl From<IconFamily> for Font {
    fn from(value: IconFamily) -> Self {
        Font::with_name(match value {
            IconFamily::MaterialSymbolsOutlined => {
                crate::font::fonts::material_symbols_outlined::NAME
            }
        })
    }
}

pub fn material_symbol<'a>(code_point: impl Into<String>) -> widget::Text<'a> {
    icon(code_point, IconFamily::MaterialSymbolsOutlined)
}

pub fn icon<'a>(code_point: impl Into<String>, icon_family: IconFamily) -> widget::Text<'a> {
    iced::widget::text(code_point.into()).font(icon_family)
}

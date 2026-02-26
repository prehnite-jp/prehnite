use crate::widget::font::get_font_opt;
use iced::alignment::Vertical;
use iced::widget::text::Wrapping;
use iced::widget::{text, Text};
use iced::{widget, Alignment, Font, Length, Pixels};

macro_rules! text_builder_template {
    ($self:ident, $res:ident, $func:path, $v:ident) => {
        let mut $res = $func($v);
        if let Some(f) = $self.font {
            $res = $res.font(f);
        }
        if let Some(w) = $self.wrapping {
            $res = $res.wrapping(w);
        }
        if let Some(a) = $self.alignment {
            $res = $res.align_x(a);
        }
        if let Some(v) = $self.vertical {
            $res = $res.align_y(v);
        }
        if let Some(w) = $self.width {
            $res = $res.width(w);
        }
        if let Some(h) = $self.height {
            $res = $res.height(h);
        }
        if let Some(s) = $self.size {
            $res = $res.size(s);
        }
    };
}

#[derive(Default)]
pub struct TextBuilder {
    pub font: Option<Font>,
    pub wrapping: Option<Wrapping>,
    pub alignment: Option<Alignment>,
    pub vertical: Option<Vertical>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub size: Option<Pixels>,
}

impl TextBuilder {
    pub fn text<'a>(&self, txt: impl text::IntoFragment<'a>) -> Text<'a> {
        text_builder_template!(self, t, text, txt);
        t
    }

    pub fn rich<'a, Link, Message>(
        &self,
        spans: impl AsRef<[text::Span<'a, Link, Font>]> + 'a,
    ) -> text::Rich<'a, Link, Message>
    where
        Link: Clone + 'static,
    {
        text_builder_template!(self, r, widget::rich_text, spans);
        r
    }

    pub fn with_font() -> Self {
        Self {
            font: get_font_opt(),
            ..Default::default()
        }
    }

    pub fn font(mut self, v: impl Into<Font>) -> Self {
        self.font = Some(v.into());
        self
    }

    pub fn wrapping(mut self, wrapping: impl Into<Wrapping>) -> Self {
        self.wrapping = Some(wrapping.into());
        self
    }

    pub fn alignment(mut self, alignment: impl Into<Alignment>) -> Self {
        self.alignment = Some(alignment.into());
        self
    }

    pub fn vertical(mut self, vertical: impl Into<Vertical>) -> Self {
        self.vertical = Some(vertical.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }
}

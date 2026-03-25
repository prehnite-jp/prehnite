#![doc = "汎用的な[`Text`]のヘルパ"]
use crate::widget::font::get_font_opt;
use iced::alignment::Vertical;
use iced::widget::text::{Rich, Wrapping};
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
/// テキストの一括書式設定用のヘルパ
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
    /// 設定を適用した[`Text`]を取得します。
    pub fn text<'a>(&self, txt: impl text::IntoFragment<'a>) -> Text<'a> {
        text_builder_template!(self, t, text, txt);
        t
    }

    /// 設定を適用した[`Rich`]を取得します。
    pub fn rich<'a, Link, Message>(
        &self,
        spans: impl AsRef<[text::Span<'a, Link, Font>]> + 'a,
    ) -> Rich<'a, Link, Message>
    where
        Link: Clone + 'static,
    {
        text_builder_template!(self, r, widget::rich_text, spans);
        r
    }

    /// 設定中のフォントを適用した新しい[`TextBuilder`]を初期化します。
    pub fn with_font() -> Self {
        Self {
            font: get_font_opt(),
            ..Default::default()
        }
    }

    /// フォントを設定します。
    pub fn font(mut self, v: impl Into<Font>) -> Self {
        self.font = Some(v.into());
        self
    }

    /// 折り返しを設定します。
    pub fn wrapping(mut self, wrapping: impl Into<Wrapping>) -> Self {
        self.wrapping = Some(wrapping.into());
        self
    }

    /// 水平方向の配置を設定します。
    pub fn alignment(mut self, alignment: impl Into<Alignment>) -> Self {
        self.alignment = Some(alignment.into());
        self
    }

    /// 垂直方向の配置を設定します。
    pub fn vertical(mut self, vertical: impl Into<Vertical>) -> Self {
        self.vertical = Some(vertical.into());
        self
    }

    /// 全体の幅を設定します。
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// 全体の高さを設定します。
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// テキストサイズを設定します。
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }
}

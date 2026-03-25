#![doc = "フォント設定を使用するテキストウィジェット"]
use crate::widget::text::TextBuilder;
use iced::widget::text::Rich;
use iced::widget::Text;
use iced::{widget, Font};
use std::sync::{LazyLock, OnceLock, RwLock};
use tracing::error;
use tracing_unwrap::{OptionExt, ResultExt};

static DEFAULT_FONT: OnceLock<Font> = OnceLock::new();

#[tracing::instrument]
/// デフォルトフォントを初期化します。
pub fn init_default_font(font: Font) {
    DEFAULT_FONT
        .set(font)
        .inspect_err(|_| error!("set_default_font was called twice."))
        .ok_or_log();
}

/// デフォルトフォントを取得します。
///
/// # Panics
/// デフォルトフォントが初期化されていない場合
pub fn get_default_font() -> Font {
    DEFAULT_FONT
        .get()
        .expect_or_log("DEFAULT_FONT not initialized.")
        .clone()
}

static CURRENT_FONT_FAMILY: LazyLock<RwLock<Option<Font>>> = LazyLock::new(|| RwLock::new(None));

#[tracing::instrument]
/// 使用するフォントを設定します。
///
/// # Panics
/// 現在のフォントファミリーの読み込み時にLockPoisoningが発生した場合
pub fn set_font(font_family: Option<&'static String>) {
    *CURRENT_FONT_FAMILY.write().unwrap_or_log() = font_family.map(|v| Font::with_name(v.as_str()));
}

/// 使用するフォントを取得します。
///
/// # Panics
/// 現在のフォントファミリーの読み込み時にLockPoisoningが発生した場合
pub fn get_font_opt() -> Option<Font> {
    CURRENT_FONT_FAMILY.read().unwrap_or_log().clone()
}

/// 使用するフォントを取得します。設定されていない場合は、デフォルトフォントを使用します。
///
/// # Panics
/// 現在のフォントファミリーの読み込み時にLockPoisoningが発生した場合
pub fn get_font() -> Font {
    CURRENT_FONT_FAMILY
        .read()
        .unwrap_or_log()
        .clone()
        .unwrap_or(get_default_font())
}

/// フォント設定が適用された[`Text`]
#[inline]
pub fn ftext<'a>(text: impl widget::text::IntoFragment<'a>) -> Text<'a> {
    TextBuilder::with_font().text(text)
}

/// フォント設定が適用された[`Rich`]
#[inline]
pub fn frich_text<'a, Link, Message>(
    spans: impl AsRef<[widget::text::Span<'a, Link, Font>]> + 'a,
) -> Rich<'a, Link, Message>
where
    Link: Clone + 'static,
{
    TextBuilder::with_font().rich(spans)
}

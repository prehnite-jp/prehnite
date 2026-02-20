use iced::{widget, Font};
use std::sync::{LazyLock, OnceLock, RwLock};
use tracing::error;

static DEFAULT_FONT: OnceLock<Font> = OnceLock::new();

#[tracing::instrument]
pub fn set_default_font(font: Font) {
    match DEFAULT_FONT.set(font) {
        Ok(_) => {}
        Err(_) => {
            error!("set_default_font was called twice.")
        }
    };
}

pub fn get_default_font() -> Font {
    DEFAULT_FONT
        .get()
        .expect("DEFAULT_FONT not initialized.")
        .clone()
}

static CURRENT_FONT_FAMILY: LazyLock<RwLock<Option<Font>>> = LazyLock::new(|| RwLock::new(None));

#[tracing::instrument]
pub fn set_font(font_family: Option<&'static String>) {
    match CURRENT_FONT_FAMILY.write() {
        Ok(mut v) => {
            *v = font_family.map(|v| Font::with_name(v.as_str()));
        }
        Err(e) => {
            error!("Failed to set font. E: {e:?}")
        }
    }
}

pub fn get_font_opt() -> Option<Font> {
    CURRENT_FONT_FAMILY.read().ok().and_then(|v| *v).clone()
}

pub fn get_font() -> Font {
    CURRENT_FONT_FAMILY
        .read()
        .ok()
        .and_then(|v| *v)
        .clone()
        .unwrap_or(get_default_font())
}

pub fn ftext<'a>(text: impl widget::text::IntoFragment<'a>) -> widget::Text<'a> {
    widget::text(text).font_maybe(get_font_opt())
}

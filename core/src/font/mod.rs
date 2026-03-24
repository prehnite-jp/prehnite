use iced::{Application, Daemon, Program};
use iced_graphics::text::font_system;
use crate::i18n::get_locale_language;
use std::sync::{OnceLock};
use tracing::error;

pub mod fonts;
pub mod widget;
pub mod material_symbol;

#[tracing::instrument]
fn font_list() -> Vec<String> {
    let mut font_list: Vec<String> = match font_system().write() {
        Ok(mut v) => v
            .raw()
            .db()
            .faces()
            .filter_map(|v| v.families.first().map(|v| v.0.clone()))
            .collect(),
        Err(e) => {
            error!("Failed to lock font_system. {e:#?}");
            Default::default()
        }
    };
    font_list.sort();
    font_list.dedup();
    font_list
}

pub fn get_global_font_list() -> &'static Vec<String> {
    static FONT_LIST: OnceLock<Vec<String>> = OnceLock::new();
    FONT_LIST.get_or_init(|| font_list())
}

pub fn get_default_font_family() -> &'static str {
    static EN_DEFAULT_FONT: &&str = &fonts::noto_sans::NAME;
    static JA_DEFAULT_FONT: &&str = &fonts::sawarabi_gothic::NAME;
    match get_locale_language().as_str() {
        "ja" => *JA_DEFAULT_FONT,
        _ => *EN_DEFAULT_FONT,
    }
}

pub trait FontLoader {
    fn load_all_prehnite_bundled_font(self) -> Self;
}

impl<P: Program> FontLoader for Daemon<P> {
    fn load_all_prehnite_bundled_font(self) -> Self {
        self.font(fonts::noto_sans::FONT)
            .font(fonts::sawarabi_gothic::FONT)
            .font(fonts::material_symbols_outlined::FONT)
    }
}

impl<P: Program> FontLoader for Application<P> {
    fn load_all_prehnite_bundled_font(self) -> Self {
        self.font(fonts::noto_sans::FONT)
            .font(fonts::sawarabi_gothic::FONT)
            .font(fonts::material_symbols_outlined::FONT)
    }
}

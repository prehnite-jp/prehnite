use easy_settings::re_export::serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr, VariantArray};
use dark_light::Mode;
use dioxus_desktop::wry::RGBA;
use tracing_unwrap::ResultExt;

#[derive(
    Clone, Deserialize, Serialize, Debug, PartialEq, VariantArray, IntoStaticStr, Display, Default,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl From<Mode> for Theme {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Dark => Theme::Dark,
            _ => Theme::Light,
        }
    }
}

impl From<Theme> for dioxus_desktop::tao::window::Theme {
    fn from(value: Theme) -> Self {
        match value {
            Theme::Light => dioxus_desktop::tao::window::Theme::Light,
            Theme::Dark => dioxus_desktop::tao::window::Theme::Dark,
        }
    }
}

impl Theme {
    pub fn get_system_default() -> Self {
        dark_light::detect()
            .ok_or_log()
            .map(|x| x.into())
            .unwrap_or_default()
    }

    pub fn bg_color(&self) -> RGBA {
        match self {
            Theme::Light => (0xFF, 0xFF, 0xFF, 0xFF),
            Theme::Dark => (0x3E, 0x3E, 0x3E, 0xFF),
        }
    }
}
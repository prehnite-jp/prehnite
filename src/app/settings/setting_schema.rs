use crate::app::settings::fetch::Fetch;
use crate::app::settings::supported_languages::SupportedLanguages;
use crate::app::settings::theme::Theme;
use easy_settings::Registry;

#[derive(Clone, Registry, Debug, PartialEq)]
#[easy_settings(categories("general"))]
pub struct GlobalSettings {
    #[easy_settings(default = SupportedLanguages::get_locale_default())]
    #[easy_settings(categories("general"))]
    locale: Option<SupportedLanguages>,
    #[easy_settings(categories("general"))]
    last_opened_file: Option<String>,
    #[easy_settings(default = true)]
    #[easy_settings(categories("general"))]
    auto_open_last_opened_file: Option<bool>,
    #[easy_settings(default = Theme::get_system_default())]
    #[easy_settings(categories("general"))]
    theme: Option<Theme>,
    #[easy_settings(default = false)]
    #[easy_settings(categories("general"))]
    license_info_message_displayed: Option<bool>,
}

impl Fetch for GlobalSettings {}

use dioxus::prelude::*;
use std::path::PathBuf;
use std::sync::LazyLock;

pub const PREHNITE_LOGO: Asset = asset!("/assets/icon/icon.png");
pub const DX_COMPONENTS_THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");
pub const GLOBAL_CSS: Asset = asset!("/assets/global.css");
pub const HEIGHT100: Asset = asset!("/assets/css/height100.css");
#[used]
static FONT: Asset = asset!(
    "/assets/font/",
    AssetOptions::folder().with_hash_suffix(false)
);

#[used]
static FONT_P: LazyLock<PathBuf> = LazyLock::new(|| FONT.resolve());
#![allow(unused)]

macro_rules! font_mod {
    ($mod_name:ident, $font_name:expr, $font_file_path:expr, $license_file_path:expr) => {
        pub mod $mod_name {
            pub const NAME: &str = $font_name;
            pub const FONT: &[u8] = include_bytes!($font_file_path);
            pub const LICENSE: &str = include_str!($license_file_path);
        }
    };
}

font_mod!(
    noto_sans,
    "",
    "../../assets/fonts/Noto_Sans/NotoSans-VariableFont_wdth,wght.ttf",
    "../../assets/fonts/Noto_Sans/OFL.txt"
);

font_mod!(
    noto_sans_italic,
    "",
    "../../assets/fonts/Noto_Sans/NotoSans-Italic-VariableFont_wdth,wght.ttf",
    "../../assets/fonts/Noto_Sans/OFL.txt"
);

font_mod!(
    noto_sans_jp,
    "",
    "../../assets/fonts/Noto_Sans_JP/NotoSansJP-VariableFont_wght.ttf",
    "../../assets/fonts/Noto_Sans_JP/OFL.txt"
);

font_mod!(
    material_symbols_outlined,
    "",
    "../../assets/fonts/Material_Symbols_Outlined/MaterialSymbolsOutlined-VariableFont_FILL,GRAD,opsz,wght.ttf",
    "../../assets/fonts/Material_Symbols_Outlined/LICENSE.txt"
);

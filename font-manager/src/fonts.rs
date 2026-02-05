macro_rules! font_mod {
    ($mod_name:ident, $font_name:expr, $font_file_path:expr, $license_file_path:expr) => {
        pub mod $mod_name {
            pub const NAME: &str = $font_name;
            pub(crate) const FONT: &[u8] = include_bytes!($font_file_path);
            pub const LICENSE: &str = include_str!($license_file_path);
        }
    };
}

font_mod!(
    noto_sans,
    "Noto Sans",
    "../assets/fonts/Noto_Sans/NotoSans-VariableFont_wdth,wght.ttf",
    "../assets/fonts/Noto_Sans/OFL.txt"
);

font_mod!(
    sawarabi_gothic,
    "Sawarabi Gothic",
    "../assets/fonts/Sawarabi_Gothic/SawarabiGothic-Regular.ttf",
    "../assets/fonts/Sawarabi_Gothic/OFL.txt"
);

font_mod!(
    material_symbols_outlined,
    "Material Symbols Rounded",
    "../assets/fonts/Material_Symbols_Outlined/MaterialSymbolsOutlined-VariableFont_FILL,GRAD,opsz,wght.ttf",
    "../assets/fonts/Material_Symbols_Outlined/LICENSE.txt"
);

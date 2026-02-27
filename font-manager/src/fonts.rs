macro_rules! font_mod {
    ($mod_name:ident, $font_name:expr, $font_file_path:expr) => {
        pub mod $mod_name {
            pub const NAME: &str = $font_name;
            pub(crate) const FONT: &[u8] = include_bytes!($font_file_path);
        }
    };
}

font_mod!(
    noto_sans,
    "Noto Sans",
    "../assets/fonts/Noto_Sans/NotoSans-VariableFont_wdth,wght.ttf"
);

font_mod!(
    sawarabi_gothic,
    "Sawarabi Gothic",
    "../assets/fonts/Sawarabi_Gothic/SawarabiGothic-Regular.ttf"
);

font_mod!(
    material_symbols_outlined,
    "Material Symbols Outlined",
    "../assets/fonts/Material_Symbols_Outlined/MaterialSymbolsOutlined-VariableFont_FILL,GRAD,opsz,wght.ttf"
);

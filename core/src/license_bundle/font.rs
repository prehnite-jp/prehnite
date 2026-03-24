use crate::license_bundle::{LicenseBundle, Package};

pub fn font() -> LicenseBundle {
    let mut license = LicenseBundle::new();
    license.push(
        Package::new("Material Symbols Outlined", "Apache-2.0")
            .homepage("https://fonts.google.com/icons")
            .repository("https://github.com/google/material-design-icons")
            .license_text(
                include_str!(
                    "../../../assets/fonts/Material_Symbols_Outlined/LICENSE.txt"
                )
                .to_string(),
            ),
    );
    license.push(
        Package::new("Noto Sans", "OFL-1.1")
            .homepage("https://fonts.google.com/noto/specimen/Noto+Sans")
            .repository("https://github.com/notofonts/latin-greek-cyrillic")
            .license_text(
                include_str!("../../../assets/fonts/Noto_Sans/OFL.txt").to_string(),
            ),
    );
    license.push(
        Package::new("Sawarabi Gothic", "OFL-1.1")
            .homepage("https://fonts.google.com/specimen/Sawarabi+Gothic")
            .license_text(
                include_str!("../../../assets/fonts/Sawarabi_Gothic/OFL.txt")
                    .to_string(),
            ),
    );
    license
}

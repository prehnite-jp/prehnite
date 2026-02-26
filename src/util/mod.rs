use fluent_bundle::FluentArgs;
use prehnite_core::i18n::i18n_fmt;

pub fn app_version_info() -> String {
    let mut args = FluentArgs::new();
    args.set("app-name", env!("CARGO_PKG_NAME"));
    args.set("version", env!("CARGO_PKG_VERSION"));
    i18n_fmt("version-info-detail", Some(&args))
}

pub fn app_build_target() -> &'static str {
    env!("BUILD_INFO_TARGET")
}

pub fn app_build_features() -> &'static str {
    env!("BUILD_INFO_FEATURE")
}

pub fn app_build_profile() -> &'static str {
    env!("BUILD_PROFILE")
}

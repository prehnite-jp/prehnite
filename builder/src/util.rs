pub(crate) fn features() -> Vec<String> {
    std::env::var("CARGO_CFG_FEATURE")
        .unwrap()
        .split(",")
        .map(|v| v.to_string())
        .collect()
}

pub(crate) fn set_env(key: &str, value: &str) {
    println!("cargo::rustc-env=PREHNITE_BUILDER_{key}={value}")
}

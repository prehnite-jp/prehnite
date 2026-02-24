#![allow(unused)]
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub(crate) fn get_features() -> Vec<String> {
    let features = std::env::var("CARGO_CFG_FEATURE").unwrap();
    features.split(",").map(|v| v.to_string()).collect()
}

pub(crate) fn target() -> String {
    std::env::var("TARGET").unwrap()
}

pub(crate) fn read_string_from_file(path: PathBuf) -> String {
    let mut result: String = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut result)
        .unwrap();
    result
}

pub(crate) fn license_zip_path() -> PathBuf {
    Path::new(&std::env::var("OUT_DIR").unwrap()).join("license-bundle.zip")
}

pub(crate) fn set_env(key: &str, value: &str) {
    println!("cargo::rustc-env={key}={value}");
}

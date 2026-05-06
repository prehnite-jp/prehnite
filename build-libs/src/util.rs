#![allow(unused)]
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub(crate) fn get_features() -> Vec<String> {
    let features = std::env::var("CARGO_CFG_FEATURE").unwrap();
    features.split(",").map(|v| v.to_string()).collect()
}

pub(crate) fn target() -> String {
    std::env::var("TARGET").unwrap()
}

pub fn set_env(key: &str, value: &str) {
    println!("cargo::rustc-env={key}={value}");
}

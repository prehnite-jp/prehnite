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

pub(crate) fn read_string_from_file(path: PathBuf) -> anyhow::Result<String> {
    let mut result: String = String::new();
    File::open(path)?.read_to_string(&mut result)?;
    Ok(result)
}

pub fn set_env(key: &str, value: &str) {
    println!("cargo::rustc-env={key}={value}");
}

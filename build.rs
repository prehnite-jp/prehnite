#![allow(unused)]

use prehnite_build_libs::util::set_env;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=build-libs");
    println!("cargo::rerun-if-changed=license-bundle");
    println!("cargo::rerun-if-changed=font-manager");
    println!("cargo::rerun-if-changed=Cargo.lock");
    println!("cargo::rerun-if-changed=Cargo.toml");
    set_env(
        "BUILD_INFO_TARGET",
        std::env::var("TARGET").unwrap().as_str(),
    );
    set_env(
        "BUILD_INFO_FEATURE",
        std::env::var("CARGO_CFG_FEATURE").unwrap().as_str(),
    );
    set_env("BUILD_PROFILE", std::env::var("PROFILE").unwrap().as_str());
    prehnite_build_libs::execute_all_build_process()
}

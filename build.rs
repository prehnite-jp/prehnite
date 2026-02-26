#![allow(unused)]

use prehnite_build_libs::util::set_env;

fn main() {
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

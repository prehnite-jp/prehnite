#![allow(unused)]
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use prehnite_builder::gen_license_info_list;

fn common() {
    gen_license_info_list();
}

#[cfg(not(debug_assertions))]
fn main() {
    common();
    set_icon()
}

#[cfg(debug_assertions)]
fn main() {
    common();
}

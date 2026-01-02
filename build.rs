#![allow(unused)]
#[cfg(target_os = "windows")]
fn set_icon() {
    extern crate embed_resource;
    embed_resource::compile(
        "assets/platform/win/prehnite.exe.icon.rc",
        embed_resource::NONE,
    )
        .manifest_optional()
        .unwrap();
}

#[cfg(not(any(target_os = "windows")))]
fn set_icon() {
    println!("Configure software icon skipped.")
}

#[cfg(not(debug_assertions))]
fn main() {
    set_icon()
}

#[cfg(debug_assertions)]
fn main() {}

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

#[cfg(not(target_os = "windows"))]
fn set_icon() {
    println!("Configure software icon skipped.")
}

fn main() {
    set_icon()
}

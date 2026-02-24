use crate::build_process;
use crate::BuildProcess;

#[cfg(not(all(target_os = "windows", feature = "icon")))]
use crate::build_process_empty_impl;

build_process!(SetIcon);

#[cfg(all(target_os = "windows", feature = "icon"))]
impl BuildProcess for SetIcon {
    fn execute(&self) {
        extern crate embed_resource;
        embed_resource::compile(
            "assets/platform/win/prehnite.exe.icon.rc",
            embed_resource::NONE,
        )
        .manifest_optional()
        .unwrap();
    }
}

#[cfg(not(all(target_os = "windows", feature = "icon")))]
build_process_empty_impl!(SetIcon);

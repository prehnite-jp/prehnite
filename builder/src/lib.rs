use crate::license_bundler::{
    Dependency, Package, path_missing_dependency_names, path_pkg_dependencies_json,
    path_pkg_list_json,
};
use krates::KrateDetails;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::{File, create_dir, create_dir_all};
use std::io::{Read, Write};
use std::sync::Arc;

pub(crate) mod license_bundler;
pub(crate) mod util;

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
fn set_icon() {}

pub fn gen_license_info_list() {
    let mut cmd = krates::Cmd::new();
    cmd.manifest_path("./Cargo.toml").features(util::features());
    let mut builder = krates::Builder::new();
    builder.include_targets(std::iter::once((std::env::var("TARGET").unwrap(), vec![])));
    let mut ignored_pkg = vec![];
    let krate: krates::Krates<cargo_about::Krate, _> = builder
        .build::<cargo_about::Krate, krates::Edge, _>(cmd, |pkg: krates::Package| {
            ignored_pkg.push(pkg);
        })
        .unwrap();
    let mut license_config = cargo_about::licenses::config::Config::default();
    license_config.accepted = crate::license_bundler::allowed_licenses();
    let license = cargo_about::licenses::Gatherer::with_store(Arc::new(
        cargo_about::licenses::store_from_cache().unwrap(),
    ))
    .gather(
        &krate,
        &license_config,
        Some(reqwest::blocking::ClientBuilder::new().build().unwrap()),
    );
    let mut packages: BTreeMap<String, Package> = krate
        .krates()
        .map(|v| {
            (
                v.name.clone(),
                Package {
                    name: v.name.clone(),
                    repository_url: v.repository.clone(),
                    homepage_url: v.homepage.clone(),
                    authors: v.authors.clone(),
                    license: v.license.clone(),
                    license_text: vec![],
                },
            )
        })
        .collect();
    let pkg_names: Vec<String> = packages.keys().map(|v| v.clone()).collect();
    let mut dependencies: Vec<Dependency> = Vec::new();
    for x in license {
        let pkg_name = x.krate.name();
        match packages.get_mut(pkg_name) {
            None => {}
            Some(pkg) => {
                for x in x.license_files.iter() {
                    let mut license_text: String = String::new();
                    File::open(x.path.clone())
                        .unwrap()
                        .read_to_string(&mut license_text)
                        .unwrap();
                    pkg.license_text.push(license_text);
                }
            }
        };
    }
    File::create(path_pkg_list_json())
        .unwrap()
        .write_all(serde_json::to_string(&packages).unwrap().as_bytes())
        .unwrap();
    #[derive(Serialize)]
    struct MissingDeps {
        depend: String,
        dependency: String,
    }
    let mut missing = vec![];
    for pkg in krate.krates() {
        dependencies.push(Dependency {
            pkg_index: pkg_names.iter().position(|v| *v == pkg.name).unwrap(),
            dependency_index: pkg
                .dependencies
                .iter()
                .filter_map(|dep| match pkg_names.iter().position(|v| *v == dep.name) {
                    None => {
                        missing.push(MissingDeps {
                            depend: pkg.name.clone(),
                            dependency: dep.name.clone(),
                        });
                        None
                    }
                    Some(v) => Some(v),
                })
                .collect(),
        })
    }
    File::create(path_missing_dependency_names())
        .unwrap()
        .write_all(serde_json::to_string(&missing).unwrap().as_bytes())
        .unwrap();
    File::create(path_pkg_dependencies_json())
        .unwrap()
        .write_all(serde_json::to_string(&dependencies).unwrap().as_bytes())
        .unwrap();
}

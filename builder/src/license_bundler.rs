use serde::{Deserialize, Serialize};
use spdx::License;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub(crate) fn allowed_licenses() -> Vec<spdx::Licensee> {
    #[derive(Deserialize)]
    struct Licenses {
        allow: Vec<String>,
    }

    #[derive(Deserialize)]
    struct DenyToml {
        licenses: Licenses,
    }
    let toml = toml::from_str::<DenyToml>(include_str!("../../deny.toml")).unwrap();
    toml.licenses
        .allow
        .iter()
        .map(|v| spdx::Licensee::from_str(v).unwrap())
        .collect()
}

pub(crate) fn path_pkg_list_json() -> PathBuf {
    Path::new(&std::env::var("OUT_DIR").unwrap()).join("oss-license-bundler.pkg_list.json")
}

pub(crate) fn path_pkg_dependencies_json() -> PathBuf {
    Path::new(&std::env::var("OUT_DIR").unwrap()).join("oss-license-bundler.pkg_deps.json")
}

pub(crate) fn path_missing_dependency_names() -> PathBuf {
    Path::new(&std::env::var("OUT_DIR").unwrap()).join("oss-license-bundler.missing_dep_names.json")
}

#[derive(Serialize)]
pub(crate) struct Package {
    pub name: String,
    pub repository_url: Option<String>,
    pub homepage_url: Option<String>,
    pub authors: Vec<String>,
    pub license_text: Vec<String>,
    pub license: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct Dependency {
    pub(crate) pkg_index: usize,
    pub(crate) dependency_index: Vec<usize>,
}

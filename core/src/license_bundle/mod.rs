mod font;

use crate::license_bundle::font::font;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub type LicenseBundle = Vec<Package>;

pub fn get_default_license_bundle() -> LicenseBundle {
    font()
}

pub fn get_names_from_default_license_bundle() -> Vec<String> {
    get_default_license_bundle()
        .into_iter()
        .map(|v| v.name)
        .collect()
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub authors: Vec<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license_info: String,
    pub licenses: Vec<License>,
    pub dependencies: BTreeSet<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub full_text: String,
}

impl Package {
    pub fn new(name: impl Into<String>, license_info: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            license_info: license_info.into(),
            ..Default::default()
        }
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.authors.push(author.into());
        self
    }

    pub fn homepage(mut self, url: impl Into<String>) -> Self {
        self.homepage = Some(url.into());
        self
    }

    pub fn repository(mut self, url: impl Into<String>) -> Self {
        self.repository = Some(url.into());
        self
    }

    pub fn license_text(mut self, license: String) -> Self {
        self.licenses.push(License { full_text: license });
        self
    }

    pub fn override_license_text(mut self, license: String) -> Self {
        self.licenses.clear();
        self.licenses.push(License { full_text: license });
        self
    }

    pub fn dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies.extend(dependencies);
        self
    }

    pub fn prehnite_member_license(self) -> Self {
        self.override_license_text(include_str!("../../../LICENSE").to_string())
    }

    pub fn prehnite() -> Self {
        Package::new("prehnite", "Zlib")
            .author("saku shirakura<saku@sakushira.com>")
            .homepage("https://prehnite.jp")
            .dependencies(get_names_from_default_license_bundle())
            .prehnite_member_license()
    }
}

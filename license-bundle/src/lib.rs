mod font;

use crate::font::font;
use serde::{Deserialize, Serialize};

pub type LicenseBundle = Vec<Package>;

pub fn get_default_license_bundle() -> LicenseBundle {
    font()
}

#[derive(Default, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub authors: Vec<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license_info: String,
    pub licenses: Vec<License>,
}

#[derive(Default, Serialize, Deserialize)]
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
}

#![doc = "ライセンスバンドル"]
mod font;

use crate::license_bundle::font::font;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::io::Cursor;
use zip::ZipArchive;

pub type LicenseBundle = Vec<Package>;

/// zipファイルから[`LicenseBundle`]を読み取ります。
pub fn read_license_bundle_from_zip(data: &[u8]) -> LicenseBundle {
    let mut zip = ZipArchive::new(Cursor::new(data)).unwrap();
    serde_json::from_reader(zip.by_name("content").unwrap()).unwrap()
}

/// デフォルトのライセンスバンドルを取得します。
///
/// `bundle_license`フラグが有効でない場合はこのライセンスバンドルの内容が使用されます。
pub fn get_default_license_bundle() -> LicenseBundle {
    font()
}

/// デフォルトのライセンスバンドルが依存する第三者ソフトウェアの名前一覧を取得する。
pub fn get_names_from_default_license_bundle() -> Vec<String> {
    get_default_license_bundle()
        .into_iter()
        .map(|v| v.name)
        .collect()
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
/// 第三者ソフトウェアの詳細を表す構造体
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
/// ライセンスを表す構造体
pub struct License {
    pub full_text: String,
}

impl Package {
    /// 名称とライセンス情報で初期化します。
    pub fn new(name: impl Into<String>, license_info: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            license_info: license_info.into(),
            ..Default::default()
        }
    }

    /// 著者を追加します。
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.authors.push(author.into());
        self
    }

    /// ホームページを設定します。
    pub fn homepage(mut self, url: impl Into<String>) -> Self {
        self.homepage = Some(url.into());
        self
    }

    /// リポジトリを設定します。
    pub fn repository(mut self, url: impl Into<String>) -> Self {
        self.repository = Some(url.into());
        self
    }

    /// ライセンスの全文を追加します。
    pub fn license_text(mut self, license: String) -> Self {
        self.licenses.push(License { full_text: license });
        self
    }

    /// ライセンスの全文をすべて削除し、新しいライセンスの全文で上書きします。
    pub fn override_license_text(mut self, license: String) -> Self {
        self.licenses.clear();
        self.licenses.push(License { full_text: license });
        self
    }

    /// 依存関係を追加します。
    pub fn dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies.extend(dependencies);
        self
    }

    /// Prehniteのメンバのライセンスを設定します。
    pub fn prehnite_member_license(self) -> Self {
        self.override_license_text(include_str!("../../../LICENSE").to_string())
    }

    /// Prehniteのライセンスを取得します。
    pub fn prehnite() -> Self {
        Package::new("prehnite", "Zlib")
            .author("saku shirakura<saku@sakushira.com>")
            .homepage("https://prehnite.jp")
            .dependencies(get_names_from_default_license_bundle())
            .prehnite_member_license()
    }
}

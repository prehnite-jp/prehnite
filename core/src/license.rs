#![doc = "LicenseBundleの読み取り"]
use crate::license_bundle::LicenseBundle;
use std::io::Cursor;
use zip::ZipArchive;

/// zipファイルから[`LicenseBundle`]を読み取ります。
pub fn read_license_bundle_from_zip(data: &[u8]) -> LicenseBundle {
    let mut zip = ZipArchive::new(Cursor::new(data)).unwrap();
    serde_json::from_reader(zip.by_name("content").unwrap()).unwrap()
}

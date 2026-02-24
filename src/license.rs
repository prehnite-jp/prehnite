use prehnite_core::license::read_license_bundle_from_zip;
use prehnite_core::license_bundle::LicenseBundle;

const LICENSE_BUNDLE_ZIP_BYTES: &[u8] = include_bytes!(env!("LICENSE_BUNDLE_ZIP_PATH"));

pub fn license_bundle() -> LicenseBundle {
    read_license_bundle_from_zip(LICENSE_BUNDLE_ZIP_BYTES)
}

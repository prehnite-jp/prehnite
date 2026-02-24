use crate::util::set_env;
use crate::{build_process, BuildProcess};
use prehnite_license_bundle::{
    get_default_license_bundle, get_names_from_default_license_bundle, Package,
};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

const FILE_NAME: &str = "content";

fn license_zip_path() -> PathBuf {
    Path::new(&std::env::var("OUT_DIR").unwrap()).join("license-bundle.zip")
}

#[cfg(feature = "bundle_license")]
fn license_collector() -> prehnite_license_bundle::LicenseBundle {
    let license_config = cargo_about::licenses::config::Config::default();
    let crates = cargo_about::get_all_crates(
        "Cargo.toml".as_ref(),
        true,
        false,
        crate::util::get_features(),
        false,
        krates::LockOptions {
            frozen: false,
            locked: false,
            offline: false,
        },
        &license_config,
        &[crate::util::target()],
    )
    .unwrap();

    let license = cargo_about::licenses::Gatherer::with_store(
        cargo_about::licenses::store_from_cache().unwrap().into(),
    )
    .gather(
        &crates,
        &license_config,
        Some(reqwest::blocking::ClientBuilder::new().build().unwrap()),
    );

    license
        .iter()
        .map(|v| prehnite_license_bundle::Package {
            name: v.krate.name.clone(),
            authors: v.krate.authors.clone(),
            homepage: v.krate.homepage.clone(),
            repository: v.krate.repository.clone(),
            license_info: v.lic_info.to_string(),
            licenses: v
                .license_files
                .iter()
                .map(|v| prehnite_license_bundle::License {
                    full_text: crate::util::read_string_from_file(v.path.canonicalize().unwrap()),
                })
                .collect(),
            dependencies: v
                .krate
                .dependencies
                .iter()
                .map(|v| v.name.clone())
                .collect(),
        })
        .collect()
}

build_process!(BundleLicense);

impl BuildProcess for BundleLicense {
    fn execute(&self) {
        #[allow(unused_mut)]
        let mut license = get_default_license_bundle();
        #[cfg(feature = "bundle_license")]
        license.extend(license_collector());
        match license.iter_mut().find(|v| v.name == "prehnite") {
            None => license.push(Package::prehnite()),
            Some(v) => v
                .dependencies
                .extend(get_names_from_default_license_bundle()),
        }

        let license_list_json = serde_json::to_string(&license).unwrap();

        let output_path = license_zip_path();
        let mut zip = ZipWriter::new(File::create(&output_path).unwrap());
        zip.start_file(
            FILE_NAME,
            SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Zstd)
                .compression_level(Some(3)),
        )
        .unwrap();
        zip.write_all(license_list_json.as_bytes()).unwrap();
        zip.flush().unwrap();
        zip.finish().unwrap();
        set_env("LICENSE_BUNDLE_ZIP_PATH", output_path.to_str().unwrap());
    }
}

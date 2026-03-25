use crate::util::set_env;
use crate::{build_process, BuildProcess};
#[cfg(feature = "bundle_license")]
use cargo_about::licenses::KrateLicense;
use prehnite_core::license_bundle::{
    get_default_license_bundle, get_names_from_default_license_bundle, Package,
};
#[cfg(feature = "bundle_license")]
use prehnite_core::license_bundle::LicenseBundle;
#[cfg(feature = "bundle_license")]
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

const FILE_NAME: &str = "content";

fn license_zip_path() -> anyhow::Result<PathBuf> {
    Ok(Path::new(&std::env::var("OUT_DIR")?).join("license-bundle.zip"))
}

#[cfg(feature = "bundle_license")]
fn license_collector() -> anyhow::Result<LicenseBundle> {

    const MY_APP_HOMEPAGE: &str = "https://prehnite.jp/";

    let license_config = cargo_about::licenses::config::Config {
        ignore_build_dependencies: true,
        ignore_dev_dependencies: !cfg!(debug_assertions),
        ..cargo_about::licenses::config::Config::default()
    };
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
    )?;

    let license = cargo_about::licenses::Gatherer::with_store(
        cargo_about::licenses::store_from_cache()?.into(),
    )
    .gather(
        &crates,
        &license_config,
        Some(reqwest::blocking::ClientBuilder::new().build()?),
    );

    let crate_names: BTreeSet<String> = license.iter().map(|v| v.krate.name.clone()).collect();

    fn load_full_text(v: &KrateLicense) -> anyhow::Result<Vec<prehnite_core::license_bundle::License>> {
        v.license_files
            .iter()
            .map(|v| {
                Ok(prehnite_core::license_bundle::License {
                    full_text: crate::util::read_string_from_file(v.path.canonicalize()?)?,
                })
            })
            .collect::<anyhow::Result<Vec<prehnite_core::license_bundle::License>>>()
    }

    fn dependencies(v: &KrateLicense, crate_names: &BTreeSet<String>) -> BTreeSet<String> {
        v.krate
            .dependencies
            .iter()
            .map(|v| v.name.clone())
            .filter(|v| crate_names.contains(v))
            .collect()
    }

    license
        .iter()
        .map(move |v| {
            Ok(Package {
                name: v.krate.name.clone(),
                authors: v.krate.authors.clone(),
                homepage: v.krate.homepage.clone(),
                repository: v.krate.repository.clone(),
                license_info: v.lic_info.to_string(),
                licenses: load_full_text(v)?,
                dependencies: dependencies(v, &crate_names),
            })
        })
        .collect::<anyhow::Result<Vec<Package>>>()?
        .into_iter()
        .map(|v| {
            if v.homepage
                .clone()
                .map(|v| v.eq(MY_APP_HOMEPAGE))
                .unwrap_or_default()
            {
                Ok(v.prehnite_member_license())
            } else {
                Ok(v)
            }
        })
        .collect::<anyhow::Result<LicenseBundle>>()
}

build_process!(BundleLicense);

impl BuildProcess for BundleLicense {
    fn execute(&self) -> anyhow::Result<()> {
        #[allow(unused_mut)]
        let mut license = get_default_license_bundle();
        #[cfg(feature = "bundle_license")]
        license.extend(license_collector()?);
        match license.iter_mut().find(|v| v.name == "prehnite") {
            None => license.push(Package::prehnite()),
            Some(v) => v
                .dependencies
                .extend(get_names_from_default_license_bundle()),
        }

        let license_list_json = serde_json::to_string(&license)?;

        let output_path = license_zip_path()?;
        let mut zip = ZipWriter::new(File::create(&output_path)?);
        zip.start_file(
            FILE_NAME,
            SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Zstd)
                .compression_level(Some(3)),
        )?;
        zip.write_all(license_list_json.as_bytes())?;
        zip.flush()?;
        zip.finish()?;
        set_env(
            "LICENSE_BUNDLE_ZIP_PATH",
            output_path
                .to_str()
                .expect("Failed to convert output path."),
        );
        Ok(())
    }
}

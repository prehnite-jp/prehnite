#![doc = "アプリのグローバルな定数等"]

#[cfg(feature = "backend")]
const DEFAULT_APP_DIR_NAME: &str = ".jp.prehnite.prehnite";

#[cfg(feature = "backend")]
#[tracing::instrument]
/// アプリディレクトリのパスを取得します。
pub fn global_dir() -> Option<std::path::PathBuf> {
    std::env::var_os(crate::env::ENV_KEY_GLOBAL_DIR_PATH)
        .map(|v| Some(std::path::PathBuf::from(v)))
        .unwrap_or_else(|| {
            if cfg!(debug_assertions) {
                Some(std::path::PathBuf::from("."))
            } else {
                let path = std::env::home_dir();
                if path.is_none() {
                    tracing::error!("Failed to get home_dir. The home directory may not be set.");
                    return None;
                }
                let mut path = path.unwrap();
                path.push(DEFAULT_APP_DIR_NAME);
                Some(path)
            }
        })
}

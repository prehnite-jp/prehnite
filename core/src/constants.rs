#![doc = "アプリのグローバルな定数等"]

#[cfg(feature = "backend")]
const DEFAULT_APP_DIR_NAME: &str = ".jp.prehnite.prehnite";

#[cfg(feature = "backend")]
#[tracing::instrument]
/// アプリディレクトリのパスを取得します。
pub fn global_dir() -> PathBuf {
    std::env::var_os(ENV_KEY_GLOBAL_DIR_PATH)
        .map(|v| v.into())
        .unwrap_or_else(|| {
            if cfg!(debug_assertions) {
                ".".into()
            } else {
                let mut path = std::env::home_dir().unwrap_or_else(|| {
                    error!("Failed to get home_dir. The home directory may not be set.");
                    // TODO: エラーダイアログを表示する。
                    panic!();
                });
                path.push(DEFAULT_APP_DIR_NAME);
                path
            }
        })
}

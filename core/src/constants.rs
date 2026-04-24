#![doc = "アプリのグローバルな定数等"]

#[tracing::instrument]
/// アプリディレクトリのパスを取得します。
pub fn global_dir() -> Option<std::path::PathBuf> {
    use tracing_unwrap::ResultExt;
    std::env::var_os(crate::env::ENV_KEY_GLOBAL_DIR_PATH)
        .map(|v| Some(std::path::PathBuf::from(v)))
        .unwrap_or_else(|| {
            if cfg!(debug_assertions) {
                std::env::current_dir().ok_or_log()
            } else {
                std::env::current_exe()
                    .ok_or_log()
                    .as_ref()
                    .and_then(|x| x.parent())
                    .map(|x| x.to_path_buf())
            }
        })
}

/// グローバル設定データベースのパスを取得します。
pub fn global_db_file_path() -> Option<std::path::PathBuf> {
    global_dir().map(|x| x.join("global.db"))
}

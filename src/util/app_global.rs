use crate::env::ENV_KEY_GLOBAL_DIR_PATH;
use crate::util::fatal_init_db_error;
use std::path::PathBuf;

const DEFAULT_APP_DIR_NAME: &str = ".jp.prehnite.prehnite";

pub fn global_dir() -> PathBuf {
    std::env::var_os(ENV_KEY_GLOBAL_DIR_PATH)
        .map(|v| v.into())
        .unwrap_or_else(|| {
            if cfg!(debug_assertions) {
                ".".into()
            } else {
                let mut path = std::env::home_dir().unwrap_or_else(|| {
                    fatal_init_db_error();
                    panic!();
                });
                path.push(DEFAULT_APP_DIR_NAME);
                path
            }
        })
}

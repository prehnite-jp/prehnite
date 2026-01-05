use crate::util::fatal_init_db_error;
use std::path::PathBuf;

pub fn global_dir() -> PathBuf {
    std::env::var_os("PREHNITE_GLOBAL_DIR_PATH")
        .map(|v| v.into())
        .unwrap_or_else(|| {
            let mut path = std::env::home_dir().unwrap_or_else(|| {
                fatal_init_db_error();
                panic!();
            });
            path.push(".jp.prehnite.prehnite");
            path
        })
}

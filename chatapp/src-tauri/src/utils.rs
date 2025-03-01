use std::path::PathBuf;

const APP_DIR: &str = ".hn";
const LOG_DIR: &str = "log";
#[allow(unused)]
const CACHE_DIR: &str = "cache";
#[allow(unused)]
const DB_DIR: &str = "db";
#[allow(unused)]
const CONFIG_DIR: &str = "config";

#[inline]
pub(crate) fn app_dir() -> PathBuf {
    dirs::home_dir()
        .expect("failed to get home directory")
        .join(APP_DIR)
}

#[inline]
pub(crate) fn log_dir() -> PathBuf {
    app_dir().join(LOG_DIR)
}

#[allow(unused)]
#[inline]
pub(crate) fn cache_dir() -> PathBuf {
    app_dir().join(CACHE_DIR)
}

#[allow(unused)]
#[inline]
pub(crate) fn db_dir() -> PathBuf {
    app_dir().join(DB_DIR)
}

#[allow(unused)]
#[inline]
pub(crate) fn config_dir() -> PathBuf {
    app_dir().join(CONFIG_DIR)
}

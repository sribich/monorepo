//! This module

use std::fs::create_dir_all;
use std::path::PathBuf;

/// Returns the path to the base directory used for local storage.
///
/// TODO: unwrap
pub fn get_app_dir() -> PathBuf {
    let base_dir = dirs::data_local_dir().unwrap();

    #[cfg(debug_assertions)]
    return base_dir.join("prelearning/application-debug");
    #[cfg(not(debug_assertions))]
    return base_dir.join("prelearning/application");
}

pub fn get_cache_dir() -> PathBuf {
    let cache_dir = get_app_dir().join("cache");

    if !cache_dir.exists() {
        create_dir_all(&cache_dir).unwrap();
    }

    cache_dir
}

pub fn get_data_dir() -> PathBuf {
    let data_dir = get_app_dir().join("data");

    if !data_dir.exists() {
        create_dir_all(&data_dir).unwrap();
    }

    data_dir
}

// config
// db
// storage

// application
//   cache
//   data
//   config.toml
//   sqlite.db

//! This module is for platform-specific stuff.

use {
    crate::{consts, error::MainError},
    std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    },
};

pub use self::inner::force_cargo_color;

// Last-modified time of a file, in milliseconds since the UNIX epoch.
pub fn file_last_modified(file: &fs::File) -> u128 {
    file.metadata()
        .and_then(|md| {
            md.modified()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_millis())
        })
        .unwrap_or(0)
}

// Current system time, in milliseconds since the UNIX epoch.
pub fn current_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[cfg(not(test))]
pub fn cache_dir() -> Result<PathBuf, MainError> {
    dirs_next::cache_dir()
        .map(|dir| dir.join(consts::PROGRAM_NAME))
        .ok_or_else(|| ("Cannot get cache directory").into())
}

#[cfg(test)]
pub fn cache_dir() -> Result<PathBuf, MainError> {
    static TEMP_DIR: once_cell::sync::Lazy<tempfile::TempDir> = once_cell::sync::Lazy::new(|| {
        tempfile::TempDir::new().expect("Cannot create temporary directory")
    });
    Ok(TEMP_DIR.path().to_path_buf())
}

pub fn generated_projects_cache_path() -> Result<PathBuf, MainError> {
    cache_dir().map(|dir| dir.join("projects"))
}

pub fn binary_cache_path() -> Result<PathBuf, MainError> {
    cache_dir().map(|dir| dir.join("binaries"))
}

pub fn templates_dir() -> Result<PathBuf, MainError> {
    if cfg!(debug_assertions) {
        if let Ok(path) = std::env::var("RUST_SCRIPT_DEBUG_TEMPLATE_PATH") {
            return Ok(path.into());
        }
    }

    dirs_next::data_local_dir()
        .map(|dir| dir.join(consts::PROGRAM_NAME).join("templates"))
        .ok_or_else(|| ("Cannot get cache directory").into())
}

#[cfg(unix)]
mod inner {
    pub use super::*;

    /// Returns `true` if `rust-script` should force Cargo to use coloured
    /// output.
    ///
    /// This depends on whether `rust-script`'s STDERR is connected to a TTY or
    /// not.
    pub fn force_cargo_color() -> bool {
        atty::is(atty::Stream::Stderr)
    }
}

#[cfg(windows)]
pub mod inner {
    pub use super::*;

    /// Returns `true` if `rust-script` should force Cargo to use coloured
    /// output.
    ///
    /// Always returns `false` on Windows because colour is communicated over a
    /// side-channel.
    pub fn force_cargo_color() -> bool {
        false
    }
}

use serde::Deserialize;
use std::{env, fs, io, path::Path};
use thiserror::Error;

#[derive(Deserialize)]
pub struct Serve {
    pub addr: Box<str>,
}

#[derive(Deserialize)]
pub struct Logging {
    pub dirs: Box<str>,
}

#[derive(Deserialize)]
pub struct Config {
    pub serve: Serve,
    pub logging: Logging,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

/// # Panics
/// Panics if `CONFIG_PATH` env variable not specified
#[must_use]
pub fn get_path() -> Box<str> {
    let Ok(path) = env::var("CONFIG_PATH") else {
        panic!("`CONFIG_PATH` env variable not specified!");
    };
    path.into_boxed_str()
}

#[allow(clippy::missing_errors_doc)]
pub fn parse_from_fs(path: impl AsRef<Path>) -> Result<Config, ParseError> {
    let raw = fs::read_to_string(path)?;
    toml::from_str(&raw).map_err(Into::into)
}

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

#[derive(Deserialize, Clone)]
pub struct Database {
    pub host: Box<str>,
    pub port: i16,
    pub user: Box<str>,
    pub password: Box<str>,
    #[allow(clippy::struct_field_names)]
    pub database: Box<str>,
}

impl Database {
    pub fn get_postgres_url(&self) -> String {
        format!(
            "postgres://{user}:{password}@{host}:{port}/{database}",
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            database = self.database,
        )
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub serve: Serve,
    pub logging: Logging,
    pub database: Database,
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

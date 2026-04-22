use serde::Deserialize;
use std::{
    env::{self, VarError},
    fs, io,
    path::Path,
};
use thiserror::Error;

#[derive(Deserialize, Clone)]
pub struct Bot {
    pub token: Box<str>,
}

#[derive(Deserialize, Clone)]
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

#[derive(Deserialize, Clone)]
pub struct Config {
    pub bot: Bot,
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
///
/// Panics if the `CONFIG_PATH` environment variable is not valid UTF-8.
#[must_use]
pub fn get_path() -> Box<str> {
    let path = match env::var("CONFIG_PATH") {
        Ok(val) => val,
        Err(VarError::NotPresent) => String::from("configs/dev.toml"),
        Err(VarError::NotUnicode(_)) => {
            panic!("`CONFIG_PATH` env variable is not a valid UTF-8 string!");
        }
    };

    path.into_boxed_str()
}

#[allow(clippy::missing_errors_doc)]
pub fn parse_from_fs(path: impl AsRef<Path>) -> Result<Config, ParseError> {
    let raw = fs::read_to_string(path)?;
    toml::from_str(&raw).map_err(Into::into)
}

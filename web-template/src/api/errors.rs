//! API-layer error types that arise around routing, before any interactor.

use thiserror::Error;

#[derive(Debug, Error)]
#[error("no route matches the requested path")]
pub struct PathNotFound;

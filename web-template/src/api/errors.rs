//! API-layer error types that arise around routing, before any interactor.

use axum::extract::rejection;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("no route matches the requested path")]
pub struct PathNotFound;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct JsonRejection(#[from] pub rejection::JsonRejection);

#[derive(Debug, Error)]
#[error(transparent)]
pub struct PathRejection(#[from] pub rejection::PathRejection);

#[derive(Debug, Error)]
#[error(transparent)]
pub struct QueryRejection(#[from] pub rejection::QueryRejection);

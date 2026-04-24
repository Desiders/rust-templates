//! Extractor wrappers that preserve axum extractor behavior while replacing
//! axum's plain-text rejection responses with the API's JSON error structure.
//!
//! Use these in controllers instead of `axum::Json`, `axum::extract::Path`,
//! and `axum::extract::Query` when rejection errors should be returned as
//! `ErrResponse` through `Resp::Err`.

use axum::{
    extract::{
        FromRequest, FromRequestParts,
        rejection::{JsonRejection, PathRejection},
    },
    http::request::Parts,
    response::{IntoResponse as _, Response},
};
use serde::de::DeserializeOwned;

use crate::api::controllers::responses::base::Resp;

pub struct Json<T>(pub T);
pub struct Path<T>(pub T);
pub struct Query<T>(pub T);

impl<S, T> FromRequest<S> for Json<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(axum::Json(value)) => Ok(Self(value)),
            Err(err) => Err((err.status(), Resp::err(err)).into_response()),
        }
    }
}

impl<S, T> FromRequestParts<S> for Path<T>
where
    axum::extract::Path<T>: FromRequestParts<S, Rejection = PathRejection>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(axum::extract::Path(value)) => Ok(Self(value)),
            Err(err) => Err((err.status(), Resp::err(err)).into_response()),
        }
    }
}

impl<S, T> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request_parts(parts, state).await {
            Ok(axum::extract::Query(value)) => Ok(Self(value)),
            Err(err) => Err((err.status(), Resp::err(err)).into_response()),
        }
    }
}

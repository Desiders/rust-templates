//! Shared API response structures used by controllers.
//!
//! Controllers return `Resp` so successful values and domain/application errors
//! are serialized with one consistent JSON shape.

mod serializer;

use axum::response::Response;
use serde::Serialize;
use serializer::{JsonSerializer, Serializer};
use std::{borrow::Cow, marker::PhantomData};
use utoipa::ToSchema;

use crate::domain::code::{entities::Code, interfaces::IntoCode};

/// Controller return type that can represent either a successful payload or an
/// error that can be converted into an API code.
pub enum Resp {}

impl Resp {
    pub fn with_serializer<S: Serializer>() -> RespWithSerializer<S> {
        RespWithSerializer(PhantomData)
    }

    pub fn ok(data: impl Serialize) -> Response {
        Self::with_serializer::<JsonSerializer>().ok(data)
    }

    pub fn err(err: impl IntoCode) -> Response {
        Self::with_serializer::<JsonSerializer>().err(err)
    }
}

pub struct RespWithSerializer<S>(PhantomData<S>);

#[allow(clippy::unused_self)]
impl<S: Serializer> RespWithSerializer<S> {
    pub fn ok(self, data: impl Serialize) -> Response {
        S::ok(OkResponse::from(data))
    }

    pub fn err(self, err: impl IntoCode) -> Response {
        S::err(ErrResponse::from(err))
    }
}

/// Successful JSON response structure.
///
/// Serialized as `{ "is_success": true, "result": ... }`.
#[derive(Serialize, ToSchema)]
pub struct OkResponse<R: ?Sized> {
    is_success: bool,
    result: R,
}

impl<R: Serialize> From<R> for OkResponse<R> {
    fn from(resp: R) -> Self {
        Self {
            result: resp,
            is_success: true,
        }
    }
}

/// Error JSON response structure.
///
/// Serialized as `{ "is_success": false, "code": ..., "name": ..., "message": ... }`.
#[derive(Serialize, ToSchema)]
pub struct ErrResponse {
    is_success: bool,
    code: u16,
    name: &'static str,
    message: Cow<'static, str>,
}

impl<E: IntoCode> From<E> for ErrResponse {
    fn from(err: E) -> Self {
        let Code {
            code,
            name,
            message,
        } = err.into_code();
        Self {
            code,
            name,
            message,
            is_success: false,
        }
    }
}

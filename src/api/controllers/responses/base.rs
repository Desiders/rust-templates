use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::code::{Code, IntoCode};

#[allow(unused)]
pub enum Resp<R: Serialize, E: IntoCode> {
    Ok(R),
    Err(E),
}

impl<R: Serialize, E: IntoCode> Resp<R, E> {
    #[allow(private_bounds)]
    pub fn with_serializer<S: Serializer>(self) -> Response {
        match self {
            Resp::Ok(data) => S::ok(OkResponse::from(data)),
            Resp::Err(err) => S::err(ErrResponse::from(err)),
        }
    }
}

impl<R: Serialize, E: IntoCode> IntoResponse for Resp<R, E> {
    fn into_response(self) -> Response {
        self.with_serializer::<JsonSerializer>()
    }
}

trait Serializer {
    fn ok<R: Serialize>(resp: OkResponse<R>) -> Response;
    fn err(err: ErrResponse) -> Response;
}

pub struct JsonSerializer;

impl Serializer for JsonSerializer {
    fn ok<R: Serialize>(resp: OkResponse<R>) -> Response {
        Json(resp).into_response()
    }

    fn err(err: ErrResponse) -> Response {
        Json(err).into_response()
    }
}

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

#[derive(Serialize, ToSchema)]
pub struct ErrResponse {
    is_success: bool,
    code: u16,
    name: &'static str,
    message: String,
}

impl<E: IntoCode> From<E> for ErrResponse {
    fn from(err: E) -> Self {
        let Code { code, name } = err.into_code();
        Self {
            code,
            name,
            message: err.to_string(),
            is_success: false,
        }
    }
}

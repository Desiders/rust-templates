use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use super::{ErrResponse, OkResponse};

pub trait Serializer {
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

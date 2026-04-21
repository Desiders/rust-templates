use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use super::{ErrResponse, OkResponse};

/// Converts typed API response structures into axum responses.
///
/// This keeps `Resp` independent from a concrete wire format. Controllers can
/// keep returning `Resp`, while serializers decide whether the response is
/// emitted as JSON or another representation.
pub trait Serializer {
    fn ok<R: Serialize>(resp: OkResponse<R>) -> Response;
    fn err(err: ErrResponse) -> Response;
}

/// JSON response serializer for the API response structure.
pub struct JsonSerializer;

impl Serializer for JsonSerializer {
    fn ok<R: Serialize>(resp: OkResponse<R>) -> Response {
        Json(resp).into_response()
    }

    fn err(err: ErrResponse) -> Response {
        Json(err).into_response()
    }
}

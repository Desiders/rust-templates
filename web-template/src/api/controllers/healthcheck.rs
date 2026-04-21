use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;
use std::convert::Infallible;
use tracing::instrument;
use utoipa::OpenApi;

use super::responses::base::{OkResponse, Resp};

#[derive(Serialize)]
struct Status {
    status: Box<str>,
}

#[utoipa::path(get, path = "status",
    responses(
        (status = StatusCode::OK, body = OkResponse<&str>, description = "Status response indicating the server is healthy")
    )
)]
#[instrument(skip_all)]
async fn status() -> impl IntoResponse {
    (
        StatusCode::OK,
        Resp::<_, Infallible>::Ok(Status {
            status: "OK".into(),
        }),
    )
}

#[utoipa::path(get, path = "ping",
    responses(
        (status = StatusCode::OK, body = OkResponse<&str>, description = "Pong response indicating the server is alive")
    )
)]
#[instrument(skip_all)]
async fn ping() -> impl IntoResponse {
    (StatusCode::OK, Resp::<_, Infallible>::Ok("pong"))
}

#[derive(OpenApi)]
#[openapi(paths(ping, status))]
pub(super) struct Doc;

pub(super) fn router() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/status", get(status))
}

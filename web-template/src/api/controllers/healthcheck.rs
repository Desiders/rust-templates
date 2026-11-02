use axum::{Router, response::IntoResponse, routing::get};
use serde::Serialize;
use tracing::instrument;
use utoipa::{OpenApi, ToSchema};

use crate::api::responses::{OkResponse, Resp};

#[derive(Serialize, ToSchema)]
struct Status {
    status: &'static str,
}

#[utoipa::path(
    get,
    path = "status",
    responses((status = StatusCode::OK, body = OkResponse<Status>, description = "Process is healthy")),
)]
#[instrument(skip_all)]
async fn status() -> impl IntoResponse {
    Resp::ok(Status { status: "OK" })
}

#[utoipa::path(
    get,
    path = "ping",
    responses((status = StatusCode::OK, body = OkResponse<&str>, description = "Process is alive")),
)]
#[instrument(skip_all)]
async fn ping() -> impl IntoResponse {
    Resp::ok("pong")
}

#[derive(OpenApi)]
#[openapi(paths(ping, status))]
pub(super) struct Doc;

pub(super) fn router() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/status", get(status))
}

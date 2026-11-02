use axum::{http::StatusCode, response::IntoResponse};
use tracing::instrument;

use crate::api::errors::PathNotFound;
use crate::api::responses::Resp;

/// 404 handler for unmatched paths. Returns the standard error envelope.
#[instrument(skip_all)]
pub(super) async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Resp::err(PathNotFound))
}

use axum::{
    Router,
    response::{IntoResponse, Redirect},
    routing::get,
};
use tracing::instrument;

#[instrument(skip_all)]
async fn docs() -> impl IntoResponse {
    Redirect::temporary("/docs")
}

pub(super) fn router() -> Router {
    Router::new().route("/", get(docs))
}

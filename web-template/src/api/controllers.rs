use axum::Router;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

pub(super) mod requests;
pub(super) mod responses;

mod healthcheck;

#[derive(OpenApi)]
#[openapi(
    info(title = "API"),
    nest(
        (path = "/", api = healthcheck::Doc),
    ),
)]
struct Doc;

pub fn router() -> Router {
    Router::new()
        .merge(healthcheck::router())
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", Doc::openapi()).path("/docs"))
}

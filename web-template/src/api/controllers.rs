#![allow(clippy::needless_for_each)]

use axum::Router;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

pub(super) mod responses;

mod default;
mod healthcheck;
mod users;

#[derive(OpenApi)]
#[openapi(
    info(title = "API"),
    nest(
        (path = "/", api = healthcheck::Doc),
        (path = "/users", api = users::Doc),
    ),
)]
struct Doc;

pub fn router() -> Router {
    Router::new()
        .merge(default::router())
        .merge(healthcheck::router())
        .nest("/users", users::router())
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", Doc::openapi()).path("/docs"))
}

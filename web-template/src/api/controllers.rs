use axum::Router;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::application::db::tx_manager::TxManager;

pub(super) mod responses;

mod healthcheck;
mod users;

#[derive(OpenApi)]
#[openapi(
    info(title = "API"),
    nest(
        (path = "/", api = healthcheck::Doc),
    ),
)]
struct Doc;

pub fn router<TxM: TxManager>() -> Router {
    Router::new()
        .merge(healthcheck::router())
        .nest("/users", users::router::<TxM>())
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", Doc::openapi()).path("/docs"))
}

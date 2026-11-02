use axum::Router;

pub(super) mod users;

pub(super) fn router() -> Router {
    Router::new().merge(users::router())
}

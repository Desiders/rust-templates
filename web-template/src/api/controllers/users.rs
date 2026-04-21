use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};
use froodi::{Inject, InjectTransient};
use tracing::{error, instrument};
use utoipa::OpenApi;

use super::responses::base::{ErrResponse, OkResponse, Resp};
use crate::{
    application::{
        common::interactor::Interactor,
        db::tx_manager::TxManager,
        user::{
            dtos::CreateUser,
            interactors::{SaveUser, SaveUserInput},
        },
    },
    domain::{common::errors::ErrKind, user::entities::User},
};

#[utoipa::path(post, path = "",
    request_body = CreateUser,
    responses(
        (status = StatusCode::CREATED, body = OkResponse<User>, description = "User created successfully"),
        (status = StatusCode::CONFLICT, body = ErrResponse, description = "User already exists"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, body = ErrResponse, description = "Unexpected error occurred"),
    ),
)]
#[instrument(skip_all)]
async fn create(
    Inject(interactor): Inject<SaveUser>,
    InjectTransient(mut tx_manager): InjectTransient<Box<dyn TxManager>>,
    Json(CreateUser { id, username }): Json<CreateUser>,
) -> impl IntoResponse {
    match interactor
        .execute(SaveUserInput {
            user: User::new(id, username),
            tx_manager: tx_manager.as_mut(),
        })
        .await
    {
        Ok(user) => (StatusCode::OK, Resp::Ok(user)),
        Err(err) => {
            error!(%err , "Add user error");
            match err {
                ErrKind::Expected(_) => (StatusCode::CONFLICT, Resp::Err(err)),
                ErrKind::Unexpected(_) => (StatusCode::INTERNAL_SERVER_ERROR, Resp::Err(err)),
            }
        }
    }
}

#[derive(OpenApi)]
#[openapi(paths(create))]
pub(super) struct Doc;

pub(super) fn router() -> Router {
    Router::new().route("/", post(create))
}

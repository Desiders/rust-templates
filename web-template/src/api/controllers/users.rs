use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};
use froodi::{Inject, InjectTransient};
use tracing::error;
use utoipa::OpenApi;

use super::responses::base::{OkResponse, Resp};
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

#[utoipa::path(post, path = "", responses(
    (status = StatusCode::CREATED, body = OkResponse<User>)
))]
async fn create<TxM: TxManager>(
    Inject(interactor): Inject<SaveUser>,
    InjectTransient(mut tx_manager): InjectTransient<TxM>,
    Json(CreateUser { id, username }): Json<CreateUser>,
) -> impl IntoResponse {
    match interactor
        .execute(SaveUserInput {
            user: User::new(id, username),
            tx_manager: &mut tx_manager,
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

pub(super) fn router<TxM: TxManager>() -> Router {
    Router::new().route("/", post(create::<TxM>))
}

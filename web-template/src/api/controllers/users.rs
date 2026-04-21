use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use froodi::{Inject, InjectTransient};
use tracing::{error, instrument};
use utoipa::OpenApi;
use uuid::Uuid;

use super::responses::base::{ErrResponse, OkResponse, Resp};
use crate::{
    application::{
        common::{entities::Pagination, interactor::Interactor},
        db::tx_manager::TxManager,
        user::{
            dtos::CreateUser,
            interactors::{
                GetUserById, GetUserByIdInput, GetUserByUsername, GetUserByUsernameInput, GetUsers,
                GetUsersInput, SaveUser, SaveUserInput,
            },
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

#[utoipa::path(get, path = "",
    params(Pagination),
    responses(
        (status = StatusCode::OK, body = OkResponse<Vec<User>>, description = "Users received successfully"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, body = ErrResponse, description = "Unexpected error occurred"),
    ),
)]
#[instrument(skip_all)]
async fn get_all(
    Inject(interactor): Inject<GetUsers>,
    InjectTransient(mut tx_manager): InjectTransient<Box<dyn TxManager>>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    match interactor
        .execute(GetUsersInput {
            pagination,
            tx_manager: tx_manager.as_mut(),
        })
        .await
    {
        Ok(users) => (StatusCode::OK, Resp::Ok(users)),
        Err(err) => {
            error!(%err , "Get users error");
            match err {
                ErrKind::Expected(_) => (StatusCode::INTERNAL_SERVER_ERROR, Resp::Err(err)),
                ErrKind::Unexpected(_) => (StatusCode::INTERNAL_SERVER_ERROR, Resp::Err(err)),
            }
        }
    }
}

#[utoipa::path(get, path = "/@{username}",
    params(
        ("username" = String, Path, description = "Username"),
    ),
    responses(
        (status = StatusCode::OK, body = OkResponse<User>, description = "User received successfully"),
        (status = StatusCode::NOT_FOUND, body = ErrResponse, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, body = ErrResponse, description = "Unexpected error occurred"),
    ),
)]
#[instrument(skip_all)]
async fn get_by_username(
    Inject(interactor): Inject<GetUserByUsername>,
    InjectTransient(mut tx_manager): InjectTransient<Box<dyn TxManager>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    match interactor
        .execute(GetUserByUsernameInput {
            username,
            tx_manager: tx_manager.as_mut(),
        })
        .await
    {
        Ok(user) => (StatusCode::OK, Resp::Ok(user)),
        Err(err) => {
            error!(%err , "Get user by username error");
            match err {
                ErrKind::Expected(_) => (StatusCode::NOT_FOUND, Resp::Err(err)),
                ErrKind::Unexpected(_) => (StatusCode::INTERNAL_SERVER_ERROR, Resp::Err(err)),
            }
        }
    }
}

#[utoipa::path(get, path = "/{id}",
    params(
        ("id" = Uuid, Path, description = "User UUID v7"),
    ),
    responses(
        (status = StatusCode::OK, body = OkResponse<User>, description = "User received successfully"),
        (status = StatusCode::NOT_FOUND, body = ErrResponse, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, body = ErrResponse, description = "Unexpected error occurred"),
    ),
)]
#[instrument(skip_all)]
async fn get_by_id(
    Inject(interactor): Inject<GetUserById>,
    InjectTransient(mut tx_manager): InjectTransient<Box<dyn TxManager>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match interactor
        .execute(GetUserByIdInput {
            id,
            tx_manager: tx_manager.as_mut(),
        })
        .await
    {
        Ok(user) => (StatusCode::OK, Resp::Ok(user)),
        Err(err) => {
            error!(%err , "Get user by id error");
            match err {
                ErrKind::Expected(_) => (StatusCode::NOT_FOUND, Resp::Err(err)),
                ErrKind::Unexpected(_) => (StatusCode::INTERNAL_SERVER_ERROR, Resp::Err(err)),
            }
        }
    }
}

#[derive(OpenApi)]
#[openapi(paths(create, get_all, get_by_username, get_by_id))]
pub(super) struct Doc;

pub(super) fn router() -> Router {
    Router::new()
        .route("/", post(create).get(get_all))
        .route("/@{username}", get(get_by_username))
        .route("/{id}", get(get_by_id))
}

use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use froodi::InjectTransient;
use tracing::{error, instrument};
use utoipa::OpenApi;
use uuid::Uuid;

use super::responses::base::{ErrResponse, OkResponse, Resp};
use crate::{
    api::extractors::responses::{Json, Path, Query},
    application::{
        common::{entities::Pagination, interactor::Interactor},
        user::{
            dtos::CreateUser,
            interactors::{
                AddUser, AddUserInput, DeleteUserById, DeleteUserByIdInput, GetUserById,
                GetUserByIdInput, GetUserByUsername, GetUserByUsernameInput, GetUsers,
                GetUsersInput,
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
    InjectTransient(interactor): InjectTransient<AddUser>,
    Json(CreateUser { id, username }): Json<CreateUser>,
) -> impl IntoResponse {
    match interactor
        .execute(AddUserInput {
            user: User::new(id, username),
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
    InjectTransient(interactor): InjectTransient<GetUsers>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    match interactor.execute(GetUsersInput { pagination }).await {
        Ok(users) => (StatusCode::OK, Resp::Ok(users)),
        Err(err) => {
            error!(%err , "Get users error");
            (StatusCode::INTERNAL_SERVER_ERROR, Resp::Err(err))
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
    InjectTransient(interactor): InjectTransient<GetUserByUsername>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    match interactor
        .execute(GetUserByUsernameInput { username })
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
    InjectTransient(interactor): InjectTransient<GetUserById>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match interactor.execute(GetUserByIdInput { id }).await {
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

#[utoipa::path(delete, path = "/{id}",
    params(
        ("id" = Uuid, Path, description = "User UUID v7"),
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "User deleted successfully"),
        (status = StatusCode::NOT_FOUND, body = ErrResponse, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, body = ErrResponse, description = "Unexpected error occurred"),
    ),
)]
#[instrument(skip_all)]
async fn delete_by_id(
    InjectTransient(interactor): InjectTransient<DeleteUserById>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match interactor.execute(DeleteUserByIdInput { id }).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => {
            error!(%err , "Delete user by id error");
            match err {
                ErrKind::Expected(_) => (StatusCode::NOT_FOUND, Resp::<(), _>::Err(err)),
                ErrKind::Unexpected(_) => (StatusCode::INTERNAL_SERVER_ERROR, Resp::Err(err)),
            }
            .into_response()
        }
    }
}

#[derive(OpenApi)]
#[openapi(paths(create, get_all, get_by_username, get_by_id, delete_by_id))]
pub(super) struct Doc;

pub(super) fn router() -> Router {
    Router::new()
        .route("/", post(create).get(get_all))
        .route("/@{username}", get(get_by_username))
        .route("/{id}", get(get_by_id).delete(delete_by_id))
}

use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use froodi::InjectTransient;
use tracing::{error, instrument};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::api::responses::{ErrResponse, OkResponse, Resp};
use crate::{
    api::{
        errors::{JsonRejection, PathRejection, QueryRejection},
        extractors::responses::{Json, Path, Query},
    },
    application::{
        common::{entities::Pagination, interactor::Interactor},
        user::{
            dtos::CreateUser,
            interactors::{
                AddUser, DeleteUserById, GetUserById, GetUserByUsername, GetUsers, GetUsersInput,
            },
        },
    },
    domain::{
        code::entities::Unexpected,
        common::errors::ErrKind,
        user::{
            entities::User,
            errors::{UserAlreadyExists, UserByIdNotFound, UserByUsernameNotFound},
        },
    },
};

#[utoipa::path(post, path = "/users",
    operation_id = "create_user",
    request_body = CreateUser,
    responses(
        (status = StatusCode::CREATED, body = OkResponse<User>, description = "User created successfully"),
        (status = StatusCode::BAD_REQUEST, body = ErrResponse<JsonRejection>, description = "Malformed JSON"),
        (status = StatusCode::PAYLOAD_TOO_LARGE, body = ErrResponse<JsonRejection>, description = "Request body exceeds the limit"),
        (status = StatusCode::UNSUPPORTED_MEDIA_TYPE, body = ErrResponse<JsonRejection>, description = "Content-Type must be application/json"),
        (status = StatusCode::UNPROCESSABLE_ENTITY, body = ErrResponse<JsonRejection>, description = "JSON does not match the request type"),
        (status = StatusCode::CONFLICT, body = ErrResponse<UserAlreadyExists>, description = "User already exists"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, body = ErrResponse<Unexpected>, description = "Unexpected error occurred"),
    ),
)]
#[instrument(skip_all)]
async fn create(
    InjectTransient(interactor): InjectTransient<AddUser>,
    Json(body): Json<CreateUser>,
) -> Response {
    match interactor.execute(body).await {
        Ok(user) => (StatusCode::CREATED, Resp::ok(user)).into_response(),
        Err(err) => {
            error!(%err, "Add user error");
            let status_code = match &err {
                ErrKind::Expected(_) => StatusCode::CONFLICT,
                ErrKind::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status_code, Resp::err(err)).into_response()
        }
    }
}

#[utoipa::path(get, path = "/users",
    operation_id = "list_users",
    params(Pagination),
    responses(
        (status = StatusCode::OK, body = OkResponse<Vec<User>>, description = "Users received successfully"),
        (status = StatusCode::BAD_REQUEST, body = ErrResponse<QueryRejection>, description = "Invalid query parameters"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, body = ErrResponse<Unexpected>, description = "Unexpected error occurred"),
    ),
)]
#[instrument(skip_all)]
async fn get_all(
    InjectTransient(interactor): InjectTransient<GetUsers>,
    Query(pagination): Query<Pagination>,
) -> Response {
    match interactor.execute(GetUsersInput { pagination }).await {
        Ok(users) => Resp::ok(users),
        Err(err) => {
            error!(%err, "Get users error");
            (StatusCode::INTERNAL_SERVER_ERROR, Resp::err(err)).into_response()
        }
    }
}

#[utoipa::path(get, path = "/users/@{username}",
    operation_id = "get_user_by_username",
    params(
        ("username" = String, Path, description = "Username"),
    ),
    responses(
        (status = StatusCode::OK, body = OkResponse<User>, description = "User received successfully"),
        (status = StatusCode::BAD_REQUEST, body = ErrResponse<PathRejection>, description = "Invalid path parameters"),
        (status = StatusCode::NOT_FOUND, body = ErrResponse<UserByUsernameNotFound>, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, body = ErrResponse<Unexpected>, description = "Unexpected error occurred"),
    ),
)]
#[instrument(skip_all)]
async fn get_by_username(
    InjectTransient(interactor): InjectTransient<GetUserByUsername>,
    Path(username): Path<String>,
) -> Response {
    match interactor.execute(username).await {
        Ok(user) => Resp::ok(user),
        Err(err) => {
            error!(%err, "Get user by username error");
            let status_code = match &err {
                ErrKind::Expected(_) => StatusCode::NOT_FOUND,
                ErrKind::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status_code, Resp::err(err)).into_response()
        }
    }
}

#[utoipa::path(get, path = "/users/{id}",
    operation_id = "get_user",
    params(
        ("id" = Uuid, Path, description = "User UUID v7"),
    ),
    responses(
        (status = StatusCode::OK, body = OkResponse<User>, description = "User received successfully"),
        (status = StatusCode::BAD_REQUEST, body = ErrResponse<PathRejection>, description = "Invalid path parameters"),
        (status = StatusCode::NOT_FOUND, body = ErrResponse<UserByIdNotFound>, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, body = ErrResponse<Unexpected>, description = "Unexpected error occurred"),
    ),
)]
#[instrument(skip_all)]
async fn get_by_id(
    InjectTransient(interactor): InjectTransient<GetUserById>,
    Path(id): Path<Uuid>,
) -> Response {
    match interactor.execute(id).await {
        Ok(user) => Resp::ok(user),
        Err(err) => {
            error!(%err, "Get user by id error");
            let status_code = match &err {
                ErrKind::Expected(_) => StatusCode::NOT_FOUND,
                ErrKind::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status_code, Resp::err(err)).into_response()
        }
    }
}

#[utoipa::path(delete, path = "/users/{id}",
    operation_id = "delete_user",
    params(
        ("id" = Uuid, Path, description = "User UUID v7"),
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "User deleted successfully"),
        (status = StatusCode::BAD_REQUEST, body = ErrResponse<PathRejection>, description = "Invalid path parameters"),
        (status = StatusCode::NOT_FOUND, body = ErrResponse<UserByIdNotFound>, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, body = ErrResponse<Unexpected>, description = "Unexpected error occurred"),
    ),
)]
#[instrument(skip_all)]
async fn delete_by_id(
    InjectTransient(interactor): InjectTransient<DeleteUserById>,
    Path(id): Path<Uuid>,
) -> Response {
    match interactor.execute(id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => {
            error!(%err, "Delete user by id error");
            let status_code = match &err {
                ErrKind::Expected(_) => StatusCode::NOT_FOUND,
                ErrKind::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status_code, Resp::err(err)).into_response()
        }
    }
}

#[derive(OpenApi)]
#[openapi(paths(create, get_all, get_by_username, get_by_id, delete_by_id))]
pub(crate) struct Doc;

pub(super) fn router() -> Router {
    Router::new()
        .route("/users", post(create).get(get_all))
        .route("/users/@{username}", get(get_by_username))
        .route("/users/{id}", get(get_by_id).delete(delete_by_id))
}

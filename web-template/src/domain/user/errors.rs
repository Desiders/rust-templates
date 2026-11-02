use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
#[error("User with id {id} or username {username:?} already exists")]
pub struct UserAlreadyExists {
    pub id: Uuid,
    pub username: Option<String>,
}

#[derive(Debug, thiserror::Error)]
#[error("User with id {id} not found")]
pub struct UserByIdNotFound {
    pub id: Uuid,
}

#[derive(Debug, thiserror::Error)]
#[error("User with username {username} not found")]
pub struct UserByUsernameNotFound {
    pub username: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Username must be 5-32 characters of [a-z0-9_]")]
pub struct InvalidUsername;

#[derive(Debug, thiserror::Error)]
pub enum CreateUserError {
    #[error(transparent)]
    InvalidUsername(#[from] InvalidUsername),
    #[error(transparent)]
    UserAlreadyExists(#[from] UserAlreadyExists),
}

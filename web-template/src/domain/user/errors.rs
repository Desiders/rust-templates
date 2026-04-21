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

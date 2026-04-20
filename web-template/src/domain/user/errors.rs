#[derive(Debug, thiserror::Error)]
#[error("User with id {id} or username {username:?} already exists")]
pub struct UserAlreadyExists {
    pub id: i64,
    pub username: Option<String>,
}

#[derive(Debug, thiserror::Error)]
#[error("User with id {id} not found")]
pub struct UserNotFound {
    pub id: i64,
}

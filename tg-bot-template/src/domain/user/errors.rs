#[derive(Debug, thiserror::Error)]
#[error("User with Telegram id {tg_id} or username {username:?} already exists")]
pub struct UserAlreadyExists {
    pub tg_id: i64,
    pub username: Option<String>,
}

#[derive(Debug, thiserror::Error)]
#[error("User with Telegram id {tg_id} not found")]
pub struct UserByTgIdNotFound {
    pub tg_id: i64,
}

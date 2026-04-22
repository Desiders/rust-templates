use async_trait::async_trait;

use crate::domain::{
    common::errors::ErrKind,
    user::{
        entities::User,
        errors::{UserAlreadyExists, UserByTgIdNotFound},
    },
};

#[async_trait]
pub trait UserReader: Send + Sync {
    async fn get_by_tg_id(&self, tg_id: i64) -> Result<User, ErrKind<UserByTgIdNotFound>>;
}

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn add(&self, user: User) -> Result<User, ErrKind<UserAlreadyExists>>;
}

use async_trait::async_trait;

use crate::domain::{
    common::errors::ErrKind,
    user::{
        entities::User,
        errors::{UserAlreadyExists, UserNotFound},
    },
};

#[async_trait]
pub trait UserReader: Send + Sync {
    async fn get_by_id(&self, id: i64) -> Result<User, ErrKind<UserNotFound>>;
}

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn add(&self, user: User) -> Result<User, ErrKind<UserAlreadyExists>>;
}

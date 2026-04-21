use async_trait::async_trait;
use std::convert::Infallible;
use uuid::Uuid;

use crate::{
    application::common::entities::Pagination,
    domain::{
        common::errors::ErrKind,
        user::{
            entities::User,
            errors::{UserAlreadyExists, UserByIdNotFound},
        },
    },
};

#[async_trait]
pub trait UserReader: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<User, ErrKind<UserByIdNotFound>>;
    async fn get_all(&self, pagination: Pagination) -> Result<Vec<User>, ErrKind<Infallible>>;
}

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn add(&self, user: User) -> Result<User, ErrKind<UserAlreadyExists>>;
}

use serde::Serialize;
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

use super::errors::InvalidUsername;
use crate::infra::db::models::users;

#[derive(Debug, Serialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn validate_username(username: &str) -> Result<(), InvalidUsername> {
        let len = username.chars().count();
        let charset_ok = username
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_');
        if (5..=32).contains(&len) && charset_ok {
            Ok(())
        } else {
            Err(InvalidUsername)
        }
    }

    pub fn new(id: Uuid, username: Option<String>) -> Self {
        Self {
            id,
            username,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl From<users::Model> for User {
    fn from(
        users::Model {
            id,
            username,
            created_at,
            updated_at,
        }: users::Model,
    ) -> Self {
        Self {
            id,
            username,
            created_at,
            updated_at,
        }
    }
}

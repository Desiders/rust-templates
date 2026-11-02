use serde::Serialize;
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

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

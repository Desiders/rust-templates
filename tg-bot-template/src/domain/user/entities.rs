use serde::Serialize;
use time::OffsetDateTime;

use crate::infra::db::models::users;

#[derive(Debug, Serialize)]
pub struct User {
    pub tg_id: i64,
    pub username: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn new(tg_id: i64, username: Option<String>) -> Self {
        Self {
            tg_id,
            username,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl From<users::Model> for User {
    fn from(
        users::Model {
            tg_id,
            username,
            created_at,
            updated_at,
        }: users::Model,
    ) -> Self {
        Self {
            tg_id,
            username,
            created_at,
            updated_at,
        }
    }
}

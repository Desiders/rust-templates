use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUser {
    pub id: i64,
    pub username: Option<String>,
}

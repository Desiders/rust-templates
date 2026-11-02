use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUser {
    #[serde(deserialize_with = "crate::utils::serde::deserialize_uuid_v7")]
    pub id: Uuid,
    #[schema(min_length = 5, max_length = 32, pattern = "^[a-z0-9_]+$")]
    pub username: Option<String>,
}

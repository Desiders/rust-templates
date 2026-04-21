use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUser {
    #[serde(deserialize_with = "crate::utils::serde::deserialize_uuid_v7")]
    pub id: Uuid,
    pub username: Option<String>,
}

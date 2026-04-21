//! Serde helper functions shared by request/response DTOs.
//!
//! Keep custom serialization and deserialization rules here when they are not
//! specific to one DTO. This avoids repeating validation logic across
//! application modules.

use serde::{Deserialize, Deserializer, de::Error as _};
use uuid::{Uuid, Version};

/// Deserializes a UUID and rejects it unless it is UUID version 7.
pub fn deserialize_uuid_v7<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let id = Uuid::deserialize(deserializer)?;
    if id.get_version() == Some(Version::SortRand) {
        Ok(id)
    } else {
        Err(D::Error::custom("expected UUID v7"))
    }
}

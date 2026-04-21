use serde::{de::Error as _, Deserialize, Deserializer};
use uuid::{Uuid, Version};

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

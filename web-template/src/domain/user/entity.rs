use time::OffsetDateTime;

pub struct User {
    pub id: i64,
    pub username: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

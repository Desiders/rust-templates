#[derive(Debug)]
pub struct CreateUser {
    pub tg_id: i64,
    pub username: Option<String>,
}

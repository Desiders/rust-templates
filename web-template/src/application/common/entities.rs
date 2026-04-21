use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

pub const DEFAULT_LIMIT: u64 = 50;
pub const MAX_LIMIT: u64 = 100;

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Pagination {
    /// Exclusive cursor. Results start after this UUID in the selected order.
    pub after_id: Option<Uuid>,
    /// Maximum number of items to return. Defaults to 50 and is capped at 100.
    #[param(minimum = 0, maximum = 100, default = 50)]
    pub limit: Option<u64>,
    /// Sort order. Defaults to ascending.
    #[serde(default)]
    #[param(inline)]
    pub order: Order,
}

impl Pagination {
    pub fn limit(&self) -> u64 {
        self.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT)
    }
}

impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Order {
    Asc,
    Desc,
}

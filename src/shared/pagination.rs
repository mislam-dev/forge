use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub limit: u64,
}

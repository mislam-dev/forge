use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdParams(pub Uuid);

impl IdParams {
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}


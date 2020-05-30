use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResetResponse {
    reset_token: String,
    id: String,
}

impl ResetResponse {
    pub fn new(id: &str, reset_token: &str) -> ResetResponse {
        ResetResponse {
            id: id.to_string(),
            reset_token: reset_token.to_string(),
        }
    }
}
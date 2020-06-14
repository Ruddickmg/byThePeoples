use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResetToken {
    pub reset_token: String,
    pub id: String,
}

impl ResetToken {
    pub fn new(id: &str, reset_token: &str) -> ResetToken {
        ResetToken {
            id: id.to_string(),
            reset_token: reset_token.to_string(),
        }
    }
}
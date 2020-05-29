use std::fmt;
use zxcvbn::zxcvbn as check_password_strength;
use serde::{Serialize, Deserialize};
use crate::Result;

pub enum Strength {
    Strong,
    Moderate,
    Weak(PasswordIssues),
}

impl fmt::Display for Strength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Strength::Strong => "Strong",
                Strength::Moderate => "Moderate",
                Strength::Weak(_) => "Weak",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct PasswordIssues {
    message: String,
    warning: Option<String>,
    suggestions: Vec<String>,
}
const WEAK_PASSWORD_MESSAGE: &str = "Password is not strong enough";

pub fn strength(name: &str, email: &str, password: &str) -> Result<Strength> {
    let result = check_password_strength(&password, &[&name, &email])?;
    Ok(match result.score() {
        0..=2 => Strength::Weak(match result.feedback() {
            Some(message) => PasswordIssues {
                message: String::from(WEAK_PASSWORD_MESSAGE),
                warning: message.warning().map(|warning| warning.to_string()),
                suggestions: message
                    .suggestions()
                    .iter()
                    .map(|suggestion| suggestion.to_string())
                    .collect(),
            },
            None => PasswordIssues {
                message: String::from(WEAK_PASSWORD_MESSAGE),
                warning: None,
                suggestions: vec![],
            },
        }),
        3 => Strength::Moderate,
        _ => Strength::Strong,
    })
}

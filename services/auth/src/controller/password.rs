extern crate argonautica;

use crate::{configuration::hash, Error, InternalServerError};
use argonautica::{Hasher, Verifier};
use serde::{Deserialize, Serialize};
use std::fmt;
use zxcvbn;

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
                Strength::Strong => "Password strength is strong",
                Strength::Moderate => "Password strength is moderate",
                Strength::Weak(_) => "Password strength is weak",
            }
        )
    }
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    let mut hasher = Hasher::default();
    Ok(hasher
        .with_password(password)
        .with_secret_key(hash::secret())
        .hash()?)
}

pub fn authenticate(password: &str, hash: &str) -> Result<bool, Error> {
    let mut verifier = Verifier::default();
    match verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(hash::secret())
        .verify()
    {
        Ok(result) => Ok(result),
        Err(error) => match error.kind() {
            argonautica::ErrorKind::HashDecodeError => Ok(false),
            _ => Err(Error::InternalServerError(InternalServerError::Unknown(
                format!("{:#?}", error),
            ))),
        },
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PasswordIssues {
    message: String,
    warning: Option<String>,
    suggestions: Vec<String>,
}
const WEAK_PASSWORD_MESSAGE: &str = "Password is not strong enough";

pub fn strength(name: &str, email: &str, password: &str) -> Result<Strength, Error> {
    let result = zxcvbn::zxcvbn(&password, &[&name, &email])?;
    Ok(match result.score() {
        0..=2 => Strength::Weak(match result.feedback() {
            Some(message) => PasswordIssues {
                message: String::from(WEAK_PASSWORD_MESSAGE),
                warning: match message.warning() {
                    Some(warning) => Some(warning.to_string()),
                    None => None,
                },
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

// ---

#[cfg(test)]
mod hashing_and_auth_tests {
    use super::*;

    #[test]
    fn password_matches_hash() {
        let password = String::from("Cool!");
        let hashed_password = match hash_password(&password) {
            Ok(hashed) => hashed,
            Err(error) => panic!("Error hashing password: {}", error),
        };
        match authenticate(&password, &hashed_password) {
            Ok(valid) => {
                if !valid {
                    panic!("Password was not validated correctly, identical passwords responded as mismatched");
                }
            }
            Err(error) => panic!("Authentication failed due to hash mismatch: {}", error),
        };
    }

    #[test]
    fn password_does_not_match_hash() {
        let password = String::from("Cool!");
        let invalid_password = String::from("Not cool...");
        let hashed_password = match hash_password(&password) {
            Ok(hashed) => hashed,
            Err(_) => panic!("Error saving password"),
        };
        match authenticate(&invalid_password, &hashed_password) {
            Ok(valid) => {
                if valid {
                    panic!(
                        "Password was incorrectly validated for invalid password: {}",
                        valid
                    );
                }
            }
            Err(error) => panic!("Error authenticating password: {}", error),
        };
    }
}

extern crate argonautica;

use crate::{configuration::hash, Error, InternalServerError};
use argonautica::{Hasher, Verifier};
use std::fmt;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Strength {
    Strong = 3,
    Moderate = 2,
    Weak = 1,
}

impl fmt::Display for Strength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Strength::Strong => "Strong",
                Strength::Moderate => "Moderate",
                Strength::Weak => "Weak",
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

pub fn strength(password: &str) -> Strength {
    if password == "password" {
        return Strength::Weak;
    }
    Strength::Strong
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

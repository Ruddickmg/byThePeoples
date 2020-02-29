extern crate argonautica;

use super::configuration::hash;
use argonautica::{Hasher, Verifier};

pub fn handle_argonautica_error<T>(
    result: Result<T, argonautica::Error>,
    message: &str,
) -> Result<T, String> {
    match result {
        Ok(value) => Ok(value),
        _ => Err(format!("{}", message)),
    }
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let mut hasher = Hasher::default();
    handle_argonautica_error(
        hasher
            .with_password(password)
            .with_secret_key(hash::secret())
            .hash(),
        "An error occurred while hashing password",
    )
}

pub fn authenticate(password: &str, hash: &str) -> Result<bool, String> {
    let mut verifier = Verifier::default();
    handle_argonautica_error(
        verifier
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(hash::secret())
            .verify(),
        "An error occurred while authenticating the a password",
    )
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

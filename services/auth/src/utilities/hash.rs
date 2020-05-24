extern crate argonautica;

use crate::{configuration::hash, Error};
use argonautica::{Hasher, Verifier};
use ring::{rand as ring_rand, rand::SecureRandom};
use std::str;
use rand;
use rand::{Rng, distributions::Alphanumeric};

const SALT_LENGTH: usize = 32;

fn generate_salt() -> Result<Vec<u8>, Error> {
    let rng = ring_rand::SystemRandom::new();
    let mut salt = [0u8; SALT_LENGTH];
    rng.fill(&mut salt)?;
    Ok(salt.to_vec())
}

pub fn token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SALT_LENGTH)
        .collect()
}

pub fn generate(word: &str) -> Result<String, Error> {
    Ok(Hasher::default()
        .configure_lanes(hash::lanes())
        .configure_iterations(hash::time_cost())
        .configure_memory_size(hash::memory_usage())
        .with_salt(generate_salt()?)
        .with_password(word)
        .with_secret_key(hash::secret())
        .hash()?)
}

pub fn authenticate(password: &str, hash: &str) -> Result<bool, Error> {
    match Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(hash::secret())
        .verify()
    {
        Ok(result) => Ok(result),
        Err(error) => match error.kind() {
            argonautica::ErrorKind::HashDecodeError => Ok(false),
            _ => Err(Error::InternalServerError(format!("{:#?}", error))),
        },
    }
}

#[cfg(test)]
mod hashing_and_auth_tests {
    use super::*;

    #[test]
    fn behaves_correctly_when_hashes_match() {
        let password = String::from("Cool!");
        let hashed_password = match generate(&password) {
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
    fn behaves_correctly_when_hash_does_not_match() {
        let password = String::from("Cool!");
        let invalid_password = String::from("Not cool...");
        let hashed_password = match generate(&password) {
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

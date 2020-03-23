use std::env;

const HASH_SECRET: &str = "HASH_SECRET";

pub fn secret() -> String {
    String::from(match env::var(HASH_SECRET) {
        Ok(secret) => secret,
        Err(_) => String::from(""),
    })
}

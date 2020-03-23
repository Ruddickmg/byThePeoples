use std::env;

const JWT_SECRET: &str = "JWT_SECRET";
const JWT_EXPIRATION: &str = "JWT_EXPIRATION";
const DEFAULT_EXPIRATION: &str = "500000";
const DEFAULT_USIZE_EXPIRATION: usize = 500000;

pub fn expiration() -> usize {
    match environment::env_or_default(JWT_EXPIRATION, DEFAULT_EXPIRATION).parse::<usize>() {
        Ok(size) => size,
        Err(_) => DEFAULT_USIZE_EXPIRATION,
    }
}

pub fn secret() -> String {
    match env::var(JWT_SECRET) {
        Ok(s) => s,
        Err(_) => String::from(""),
    }
}

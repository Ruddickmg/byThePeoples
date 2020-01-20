pub mod authentication;
pub mod model;
pub mod routes;

use std::env;

pub fn env_or_default(variable_name: &str, default: &str) -> String {
    match env::var(variable_name) {
        Ok(value) => value,
        Err(_) => format!("{}", default),
    }
}

pub mod environment {
    use std::env;

    const ENVIRONMENT: &str = "ENVIRONMENT";
    const PRODUCTION: &str = "production";
    const STAGING: &str = "staging";
    const DEVELOPMENT: &str = "development";

    fn test_environment(target: &str) -> bool {
        match env::var(ENVIRONMENT) {
            Ok(environment) => environment == target,
            Err(_) => false,
        }
    }
    pub fn in_production() -> bool {
        test_environment(PRODUCTION)
    }
    pub fn in_staging() -> bool {
        test_environment(STAGING)
    }
    pub fn in_development() -> bool {
        test_environment(DEVELOPMENT)
    }
}

pub mod connection {
    const PORT: &str = "PORT";
    const ADDRESS: &str = "IP";
    const DEFAULT_PORT: &str = "80";
    const ALL_ADDRESSES: &str = "0.0.0.0";

    pub fn uri() -> String {
        let ip = super::env_or_default(ADDRESS, ALL_ADDRESSES);
        let port = super::env_or_default(PORT, DEFAULT_PORT);
        format!("{}:{}", ip, port)
    }
}

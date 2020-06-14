use std::env;

const ENVIRONMENT: &str = "ENVIRONMENT";
const PRODUCTION: &str = "production";
const STAGING: &str = "staging";
const DEVELOPMENT: &str = "development";

pub fn env_or_default<T: std::fmt::Display>(variable_name: &str, default: T) -> String {
    env::var(variable_name).unwrap_or(format!("{}", default))
}

pub fn path(path: &str) -> String {
    format!(
        "{}{}",
        env::current_dir().unwrap().to_str().unwrap(),
        path,
    )
}

fn check_environment(target: &str) -> bool {
    match env::var(ENVIRONMENT) {
        Ok(environment) => environment == target,
        Err(_) => false,
    }
}

pub fn in_production() -> bool {
    check_environment(PRODUCTION)
}
pub fn in_staging() -> bool {
    check_environment(STAGING)
}
pub fn in_development() -> bool {
    check_environment(DEVELOPMENT)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

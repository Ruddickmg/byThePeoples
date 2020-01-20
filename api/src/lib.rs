use std::sync::{Arc, Mutex};

pub mod graph_ql;
pub mod routes;

pub struct AppData {
    pub schema: Arc<graph_ql::schema::Schema>,
    pub app_name: String,
    pub counter: Mutex<i32>,
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
    use std::env;

    const PORT: &str = "PORT";
    const ADDRESS: &str = "IP";
    const DEFAULT_PORT: u32 = 80;
    const ALL_ADDRESSES: &str = "0.0.0.0";

    pub fn uri() -> String {
        let ip = match env::var(ADDRESS) {
            Ok(ip) => ip,
            Err(_) => format!("{}", ALL_ADDRESSES),
        };
        let port = match env::var(PORT) {
            Ok(port) => port,
            Err(_) => format!("{}", DEFAULT_PORT),
        };
        format!("{}:{}", ip, port)
    }
}

use std::sync::{Arc, Mutex};

pub mod graph_ql;
pub mod models;
pub mod redis;
pub mod routes;

pub struct AppData {
    pub schema: Arc<graph_ql::schema::Schema>,
    pub app_name: String,
    pub counter: Mutex<i32>,
}

pub mod connection {
    const PORT: &str = "PORT";
    const ADDRESS: &str = "IP";
    const DEFAULT_PORT: &str = "3000";
    const ALL_ADDRESSES: &str = "0.0.0.0";

    pub const GRAPHQL_ENDPOINT: &str = "graphql";
    pub const REDIS_IP: &str = "127.0.0.2:6379";

    pub fn uri() -> String {
        let ip = environment::env_or_default(ADDRESS, ALL_ADDRESSES);
        let port = environment::env_or_default(PORT, DEFAULT_PORT);
        format!("{}:{}", ip, port)
    }

    pub fn graphql() -> String {
        format!("http://{}/{}", uri(), GRAPHQL_ENDPOINT)
    }
}

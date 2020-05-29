const PORT: &str = "PORT";
const ADDRESS: &str = "IP";
const DEFAULT_PORT: &str = "8080";
const ALL_ADDRESSES: &str = "0.0.0.0";

pub fn uri() -> String {
    let ip = environment::env_or_default(ADDRESS, ALL_ADDRESSES);
    let port = environment::env_or_default(PORT, DEFAULT_PORT);
    format!("{}:{}", ip, port)
}
mod mutations;
mod queries;
pub mod schema;

use schema::{create_schema, Schema};
use std::sync::Arc;

pub fn graph_schema() -> Arc<Schema> {
    std::sync::Arc::new(create_schema())
}

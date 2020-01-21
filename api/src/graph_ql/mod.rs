pub mod schema;

use std::sync::Arc;

use schema::{create_schema, Schema};

pub fn graph_schema() -> Arc<Schema> {
    std::sync::Arc::new(create_schema())
}

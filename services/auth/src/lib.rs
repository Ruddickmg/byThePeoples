mod error;

pub mod configuration;
pub mod constants;
pub mod controller;
pub mod handler;
pub mod model;
pub mod repository;
pub mod routes;
pub mod utilities;
pub mod server;

pub type Result<T> = std::result::Result<T, error::Error>;

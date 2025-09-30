//! Storage layer module

pub mod db;
pub mod department;
pub mod section;
pub mod user;
pub mod business_task;
pub mod document_type;
pub mod document_path;
pub mod counter;
pub mod query;

pub use db::init_db_pool;

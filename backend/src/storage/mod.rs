//! Storage layer module

pub mod business_task;
pub mod counter;
pub mod db;
pub mod department;
pub mod document_path;
pub mod document_type;
pub mod query;
pub mod section;
pub mod user;

pub use db::init_db_pool;

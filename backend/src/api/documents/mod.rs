//! Document API handlers module

pub mod create_auto;
pub mod create_manual;
pub mod delete;
pub mod get_by_id;
pub mod get_by_number;
pub mod search;
pub mod update_path;

pub use create_auto::create_document_auto;
pub use create_manual::create_document_manual;
pub use delete::delete_document;
pub use get_by_id::get_document_by_id;
pub use get_by_number::get_document_by_number;
pub use search::search_documents;
pub use update_path::update_document_path;

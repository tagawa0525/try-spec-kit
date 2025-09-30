//! API handlers

pub mod documents;

use axum::{
    Router,
    routing::{get, post, put, delete},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::SqlitePool;

use crate::error::Error;

/// Create the API router
pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        // Document endpoints
        .route("/api/documents", post(documents::create_document_auto))
        .route("/api/documents/manual", post(documents::create_document_manual))
        .route("/api/documents/search", get(documents::search_documents))
        .route("/api/documents/:id", get(documents::get_document_by_id))
        .route("/api/documents/:id/path", put(documents::update_document_path))
        .route("/api/documents/:id", delete(documents::delete_document))
        .route("/api/documents/number/:number", get(documents::get_document_by_number))
        .with_state(pool)
}

/// Convert Error to HTTP response
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Error::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Error::DocumentNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Error::DepartmentNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Error::SectionNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Error::UserNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Error::BusinessTaskNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Error::Validation(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::RelativePathNotAllowed => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::InvalidTypeCode(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::DuplicateDocumentNumber(_) => (StatusCode::CONFLICT, self.to_string()),
            Error::UnauthorizedDocumentType => (StatusCode::FORBIDDEN, self.to_string()),
            Error::ConcurrentModification => (StatusCode::CONFLICT, self.to_string()),
            Error::InvalidRuleComponent(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            Error::Migration(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Migration error".to_string()),
            Error::Serialization(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Serialization error".to_string()),
            Error::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "IO error".to_string()),
            Error::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        
        let body = Json(json!({
            "error": message,
        }));
        
        (status, body).into_response()
    }
}

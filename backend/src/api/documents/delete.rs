//! DELETE /api/documents/:id

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::SqlitePool;

use crate::error::Result;
use crate::models::DocumentId;
use crate::services::document_service;

/// DELETE /api/documents/:id - Logically delete document
pub async fn delete_document(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode> {
    document_service::delete_document(&pool, &DocumentId::new(&id)).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_delete_document_signature() {
        // Compile-time type check
        let _: fn(State<SqlitePool>, Path<String>) -> _ = delete_document;
    }
}

//! GET /api/documents/:id

use axum::{extract::{Path, State}, Json};
use sqlx::SqlitePool;

use crate::error::{Error, Result};
use crate::models::DocumentId;
use crate::services::document_service;
use super::create_auto::CreateDocumentResponse;

/// GET /api/documents/:id - Get document by ID
pub async fn get_document_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<CreateDocumentResponse>> {
    let doc = document_service::get_document_by_id(&pool, &DocumentId::new(&id))
        .await?
        .ok_or_else(|| Error::DocumentNotFound(id.clone()))?;
    
    Ok(Json(doc.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_document_by_id_signature() {
        // Compile-time type check
        let _: fn(
            State<SqlitePool>,
            Path<String>,
        ) -> _ = get_document_by_id;
    }
}

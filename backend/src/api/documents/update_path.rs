//! PUT /api/documents/:id/path

use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::path::PathBuf;

use super::create_auto::CreateDocumentResponse;
use crate::error::Result;
use crate::models::DocumentId;
use crate::services::document_service;

#[derive(Debug, Deserialize)]
pub struct UpdateDocumentPathRequest {
    pub file_path: String,
}

/// PUT /api/documents/:id/path - Update document file path
pub async fn update_document_path(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(req): Json<UpdateDocumentPathRequest>,
) -> Result<Json<CreateDocumentResponse>> {
    let file_path = PathBuf::from(&req.file_path);

    let doc =
        document_service::update_document_path(&pool, &DocumentId::new(&id), file_path).await?;

    Ok(Json(doc.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_update_document_path_signature() {
        // Compile-time type check
        let _: fn(State<SqlitePool>, Path<String>, Json<UpdateDocumentPathRequest>) -> _ =
            update_document_path;
    }
}

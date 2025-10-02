//! GET /api/documents/number/:number

use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::SqlitePool;

use super::create_auto::CreateDocumentResponse;
use crate::error::{Error, Result};
use crate::services::document_service;

/// GET /api/documents/number/:number - Get document by number
pub async fn get_document_by_number(
    State(pool): State<SqlitePool>,
    Path(number): Path<String>,
) -> Result<Json<CreateDocumentResponse>> {
    let doc = document_service::get_document_by_number(&pool, &number)
        .await?
        .ok_or_else(|| Error::DocumentNotFound(number.clone()))?;

    Ok(Json(doc.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_document_by_number_signature() {
        // Compile-time type check
        let _: fn(State<SqlitePool>, Path<String>) -> _ = get_document_by_number;
    }
}

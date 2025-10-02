//! GET /api/documents - Get all documents

use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;
use sqlx::SqlitePool;

use super::create_auto::CreateDocumentResponse;
use crate::error::Result;
use crate::storage::query;

#[derive(Debug, Deserialize)]
pub struct GetAllQuery {
    pub include_deleted: Option<bool>,
}

/// GET /api/documents - Get all documents
pub async fn get_all_documents(
    State(pool): State<SqlitePool>,
    Query(query_params): Query<GetAllQuery>,
) -> Result<Json<Vec<CreateDocumentResponse>>> {
    let include_deleted = query_params.include_deleted.unwrap_or(false);
    let documents = query::get_all_documents(&pool, include_deleted).await?;
    let response: Vec<CreateDocumentResponse> = documents.into_iter().map(|d| d.into()).collect();
    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_documents_signature() {
        // Compile-time type check
        let _: fn(State<SqlitePool>, Query<GetAllQuery>) -> _ = get_all_documents;
    }
}

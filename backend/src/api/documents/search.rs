//! GET /api/documents/search

use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::error::Result;
use crate::storage::query;
use super::create_auto::CreateDocumentResponse;

#[derive(Debug, Deserialize)]
pub struct SearchDocumentsQuery {
    pub q: Option<String>,
    pub type_code: Option<String>,
    pub department: Option<char>,
    pub section: Option<char>,
    pub business_task: Option<String>,
    pub include_deleted: Option<bool>,
}

/// GET /api/documents/search - Search documents
pub async fn search_documents(
    State(pool): State<SqlitePool>,
    Query(query_params): Query<SearchDocumentsQuery>,
) -> Result<Json<Vec<CreateDocumentResponse>>> {
    let include_deleted = query_params.include_deleted.unwrap_or(false);
    
    let documents = if let Some(q) = query_params.q {
        // Text search in document number
        query::search_documents_by_text(&pool, &q, include_deleted).await?
    } else {
        // Filter by parameters
        let mut docs = query::get_all_documents(&pool, include_deleted).await?;
        
        if let Some(type_code) = &query_params.type_code {
            docs.retain(|d| &d.document_type.0 == type_code);
        }
        
        if let Some(dept) = query_params.department {
            docs.retain(|d| d.department.0 == dept);
        }
        
        if let Some(section) = query_params.section {
            docs.retain(|d| d.section.0 == section);
        }
        
        if let Some(task_id) = &query_params.business_task {
            docs.retain(|d| {
                if let Some(ref t) = d.business_task {
                    &t.0 == task_id
                } else {
                    false
                }
            });
        }
        
        docs
    };
    
    let response: Vec<CreateDocumentResponse> = documents
        .into_iter()
        .map(|d| d.into())
        .collect();
    
    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_documents_signature() {
        // Compile-time type check
        let _: fn(
            State<SqlitePool>,
            Query<SearchDocumentsQuery>,
        ) -> _ = search_documents;
    }
}

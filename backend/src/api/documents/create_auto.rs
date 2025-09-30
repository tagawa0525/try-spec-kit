//! POST /api/documents (auto-generated)

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::PathBuf;

use crate::error::Result;
use crate::models::{DeptCode, DocumentPath, SectionCode, TaskId, TypeCode, UserId};
use crate::services::document_service;

#[derive(Debug, Deserialize)]
pub struct CreateDocumentAutoRequest {
    pub type_code: String,
    pub dept_code: char,
    pub section_code: char,
    pub user_id: String,
    pub file_path: String,
    pub business_task: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateDocumentResponse {
    pub id: String,
    pub document_number: String,
    pub document_type: String,
    pub department: char,
    pub section: char,
    pub business_task: Option<String>,
    pub user_id: String,
    pub file_path: String,
    pub created_at: String,
    pub updated_at: String,
    pub generated: bool,
    pub deleted: bool,
}

impl From<DocumentPath> for CreateDocumentResponse {
    fn from(doc: DocumentPath) -> Self {
        Self {
            id: doc.id.0,
            document_number: doc.document_number,
            document_type: doc.document_type.0,
            department: doc.department.0,
            section: doc.section.0,
            business_task: doc.business_task.map(|t| t.0),
            user_id: doc.user.0,
            file_path: doc.file_path.to_string_lossy().to_string(),
            created_at: doc.created_at.to_rfc3339(),
            updated_at: doc.updated_at.to_rfc3339(),
            generated: doc.generated,
            deleted: doc.deleted,
        }
    }
}

/// POST /api/documents - Create document with auto-generated number
pub async fn create_document_auto(
    State(pool): State<SqlitePool>,
    Json(req): Json<CreateDocumentAutoRequest>,
) -> Result<(StatusCode, Json<CreateDocumentResponse>)> {
    let file_path = PathBuf::from(&req.file_path);

    let doc = document_service::create_document_auto(
        &pool,
        TypeCode::new(&req.type_code),
        DeptCode::new(req.dept_code),
        SectionCode::new(req.section_code),
        UserId::new(&req.user_id),
        file_path,
        req.business_task.map(|t| TaskId::new(&t)),
    )
    .await?;

    Ok((StatusCode::CREATED, Json(doc.into())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_document_auto_signature() {
        // Compile-time type check
        let _: fn(State<SqlitePool>, Json<CreateDocumentAutoRequest>) -> _ = create_document_auto;
    }
}

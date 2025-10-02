//! POST /api/documents/manual

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::path::PathBuf;

use super::create_auto::CreateDocumentResponse;
use crate::error::Result;
use crate::models::{DeptCode, SectionCode, TaskId, TypeCode, UserId};
use crate::services::document_service::{self, ManualDocumentRequest};

#[derive(Debug, Deserialize)]
pub struct CreateDocumentManualRequest {
    pub document_number: String,
    pub type_code: String,
    pub dept_code: char,
    pub section_code: char,
    pub user_id: String,
    pub file_path: String,
    pub business_task: Option<String>,
}

/// POST /api/documents/manual - Create document with manual number
pub async fn create_document_manual(
    State(pool): State<SqlitePool>,
    Json(req): Json<CreateDocumentManualRequest>,
) -> Result<(StatusCode, Json<CreateDocumentResponse>)> {
    let file_path = PathBuf::from(&req.file_path);

    let doc = document_service::create_document_manual(
        &pool,
        ManualDocumentRequest {
            document_number: req.document_number,
            type_code: TypeCode::new(&req.type_code),
            dept_code: DeptCode::new(req.dept_code),
            section_code: SectionCode::new(req.section_code),
            user_id: UserId::new(&req.user_id),
            file_path,
            business_task: req.business_task.filter(|t| !t.is_empty()).map(|t| TaskId::new(&t)),
        },
    )
    .await?;

    Ok((StatusCode::CREATED, Json(doc.into())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_document_manual_signature() {
        // Compile-time type check
        let _: fn(State<SqlitePool>, Json<CreateDocumentManualRequest>) -> _ =
            create_document_manual;
    }
}

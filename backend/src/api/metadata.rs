//! Metadata API handlers (departments, document types)

use axum::{extract::State, Json};
use sqlx::SqlitePool;

use crate::error::Result;
use crate::storage::{department, document_type};

/// GET /api/departments
pub async fn list_departments(State(pool): State<SqlitePool>) -> Result<Json<serde_json::Value>> {
    let deps = department::list_departments(&pool).await?;
    Ok(Json(serde_json::to_value(deps)?))
}

/// GET /api/document-types
pub async fn list_document_types(State(pool): State<SqlitePool>) -> Result<Json<serde_json::Value>> {
    let types = document_type::list_active_document_types(&pool).await?;
    Ok(Json(serde_json::to_value(types)?))
}

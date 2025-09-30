//! Document Path storage operations

use sqlx::SqlitePool;
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use crate::error::Result;
use crate::models::{DocumentPath, DocumentId, TypeCode, DeptCode, SectionCode, TaskId, UserId};

/// Create a new document path
pub async fn create_document_path(pool: &SqlitePool, doc: &DocumentPath) -> Result<()> {
    let task_id = doc.business_task.as_ref().map(|t| t.0.clone());
    let file_path_str = doc.file_path.to_string_lossy().to_string();
    let dept_code = doc.department.0.to_string();
    let section_code = doc.section.0.to_string();
    let created_at = doc.created_at.to_rfc3339();
    let updated_at = doc.updated_at.to_rfc3339();
    let generated = doc.generated as i32;
    let deleted = doc.deleted as i32;
    
    sqlx::query!(
        r#"
        INSERT INTO documents (
            id, document_number, document_type_code, department_code, section_code,
            business_task_id, user_id, file_path, created_at, updated_at, generated, deleted
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        doc.id.0,
        doc.document_number,
        doc.document_type.0,
        dept_code,
        section_code,
        task_id,
        doc.user.0,
        file_path_str,
        created_at,
        updated_at,
        generated,
        deleted
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get a document path by ID
pub async fn get_document_path(pool: &SqlitePool, id: &DocumentId) -> Result<Option<DocumentPath>> {
    let row = sqlx::query!(
        r#"
        SELECT id, document_number, document_type_code, department_code, section_code,
               business_task_id, user_id, file_path, created_at, updated_at, generated, deleted
        FROM documents
        WHERE id = ?
        "#,
        id.0
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => {
            let dept_char = r.department_code.chars().next().unwrap_or('?');
            let section_char = r.section_code.chars().next().unwrap_or('?');
            
            Ok(Some(DocumentPath {
                id: DocumentId::new(r.id),
                document_number: r.document_number,
                document_type: TypeCode::new(r.document_type_code),
                department: DeptCode::new(dept_char),
                section: SectionCode::new(section_char),
                business_task: r.business_task_id.map(TaskId::new),
                user: UserId::new(r.user_id),
                file_path: PathBuf::from(r.file_path),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                generated: r.generated != 0,
                deleted: r.deleted != 0,
            }))
        }
        None => Ok(None),
    }
}

/// Get a document path by document number
pub async fn get_document_path_by_number(pool: &SqlitePool, number: &str) -> Result<Option<DocumentPath>> {
    let row = sqlx::query!(
        r#"
        SELECT id, document_number, document_type_code, department_code, section_code,
               business_task_id, user_id, file_path, created_at, updated_at, generated, deleted
        FROM documents
        WHERE document_number = ?
        "#,
        number
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => {
            let dept_char = r.department_code.chars().next().unwrap_or('?');
            let section_char = r.section_code.chars().next().unwrap_or('?');
            
            Ok(Some(DocumentPath {
                id: DocumentId::new(r.id),
                document_number: r.document_number,
                document_type: TypeCode::new(r.document_type_code),
                department: DeptCode::new(dept_char),
                section: SectionCode::new(section_char),
                business_task: r.business_task_id.map(TaskId::new),
                user: UserId::new(r.user_id),
                file_path: PathBuf::from(r.file_path),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                generated: r.generated != 0,
                deleted: r.deleted != 0,
            }))
        }
        None => Ok(None),
    }
}

/// List all document paths (including deleted)
pub async fn list_document_paths(pool: &SqlitePool, include_deleted: bool) -> Result<Vec<DocumentPath>> {
    let deleted_filter = if include_deleted { 1 } else { 0 };
    
    let rows = sqlx::query!(
        r#"
        SELECT id, document_number, document_type_code, department_code, section_code,
               business_task_id, user_id, file_path, created_at, updated_at, generated, deleted
        FROM documents
        WHERE (deleted = 0 OR ? = 1)
        ORDER BY created_at DESC
        "#,
        deleted_filter
    )
    .fetch_all(pool)
    .await?;

    let docs = rows
        .into_iter()
        .filter_map(|r| {
            let dept_char = r.department_code.chars().next()?;
            let section_char = r.section_code.chars().next()?;
            
            Some(DocumentPath {
                id: DocumentId::new(r.id),
                document_number: r.document_number,
                document_type: TypeCode::new(r.document_type_code),
                department: DeptCode::new(dept_char),
                section: SectionCode::new(section_char),
                business_task: r.business_task_id.map(TaskId::new),
                user: UserId::new(r.user_id),
                file_path: PathBuf::from(r.file_path),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                generated: r.generated != 0,
                deleted: r.deleted != 0,
            })
        })
        .collect();

    Ok(docs)
}

/// Update a document path (only file_path can be updated)
pub async fn update_document_path(pool: &SqlitePool, id: &DocumentId, new_path: PathBuf) -> Result<()> {
    let file_path_str = new_path.to_string_lossy().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query!(
        r#"
        UPDATE documents
        SET file_path = ?, updated_at = ?
        WHERE id = ?
        "#,
        file_path_str,
        now,
        id.0
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Logically delete a document path
pub async fn delete_document_path(pool: &SqlitePool, id: &DocumentId) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    sqlx::query!(
        r#"
        UPDATE documents
        SET deleted = 1, updated_at = ?
        WHERE id = ?
        "#,
        now,
        id.0
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Restore a logically deleted document path
pub async fn restore_document_path(pool: &SqlitePool, id: &DocumentId) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    sqlx::query!(
        r#"
        UPDATE documents
        SET deleted = 0, updated_at = ?
        WHERE id = ?
        "#,
        now,
        id.0
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db::init_db_pool;
    use crate::storage::{department, section, user, document_type};
    use crate::models::{Department, Section, User, DocumentType, PathGenerationRule};

    async fn setup_test_data(pool: &SqlitePool) {
        // Create department
        let dept = Department::new('G', "総務");
        department::create_department(pool, &dept).await.unwrap();
        
        // Create section
        let sec = Section {
            code: SectionCode::new('I'),
            name: "インフラ".to_string(),
            department: DeptCode::new('G'),
        };
        section::create_section(pool, &sec).await.unwrap();
        
        // Create user
        let u = User::new("user001", "田川太郎", 'G', 'I');
        user::create_user(pool, &u).await.unwrap();
        
        // Create document type
        let rule = PathGenerationRule::example_agi();
        let doc_type = DocumentType::new("A", "契約書", "/docs/contracts/", rule);
        document_type::create_document_type(pool, &doc_type).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_and_get_document_path() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        setup_test_data(&pool).await;
        
        let doc = DocumentPath::new_auto(
            "AGI2509001",
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/contracts/AGI2509001.pdf"),
        );
        
        create_document_path(&pool, &doc).await.unwrap();
        
        let retrieved = get_document_path(&pool, &doc.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().document_number, "AGI2509001");
    }

    #[tokio::test]
    async fn test_get_document_path_by_number() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        setup_test_data(&pool).await;
        
        let doc = DocumentPath::new_auto(
            "AGI2509001",
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/contracts/AGI2509001.pdf"),
        );
        
        create_document_path(&pool, &doc).await.unwrap();
        
        let retrieved = get_document_path_by_number(&pool, "AGI2509001").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, doc.id);
    }

    #[tokio::test]
    async fn test_logical_deletion() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        setup_test_data(&pool).await;
        
        let doc = DocumentPath::new_auto(
            "AGI2509001",
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/test.pdf"),
        );
        
        create_document_path(&pool, &doc).await.unwrap();
        
        // Delete
        delete_document_path(&pool, &doc.id).await.unwrap();
        
        let retrieved = get_document_path(&pool, &doc.id).await.unwrap().unwrap();
        assert!(retrieved.deleted);
        
        // Should not appear in non-deleted list
        let list = list_document_paths(&pool, false).await.unwrap();
        assert_eq!(list.len(), 0);
        
        // Should appear in full list
        let full_list = list_document_paths(&pool, true).await.unwrap();
        assert_eq!(full_list.len(), 1);
        
        // Restore
        restore_document_path(&pool, &doc.id).await.unwrap();
        
        let restored = get_document_path(&pool, &doc.id).await.unwrap().unwrap();
        assert!(!restored.deleted);
    }

    #[tokio::test]
    async fn test_update_document_path() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        setup_test_data(&pool).await;
        
        let doc = DocumentPath::new_auto(
            "AGI2509001",
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/old/path.pdf"),
        );
        
        create_document_path(&pool, &doc).await.unwrap();
        
        let new_path = PathBuf::from("/new/path.pdf");
        update_document_path(&pool, &doc.id, new_path.clone()).await.unwrap();
        
        let updated = get_document_path(&pool, &doc.id).await.unwrap().unwrap();
        assert_eq!(updated.file_path, new_path);
    }
}

//! Document creation and management service

use std::path::PathBuf;
use chrono::Utc;
use sqlx::SqlitePool;
use crate::error::Result;
use crate::models::{DocumentPath, DocumentId, TypeCode, DeptCode, SectionCode, TaskId, UserId};
use crate::storage::{document_type, document_path};
use crate::services::generation_service;

/// Create a document with auto-generated number
pub async fn create_document_auto(
    pool: &SqlitePool,
    type_code: TypeCode,
    dept_code: DeptCode,
    section_code: SectionCode,
    user_id: UserId,
    file_path: PathBuf,
    business_task: Option<TaskId>,
) -> Result<DocumentPath> {
    // Validate file path is absolute
    if !file_path.is_absolute() {
        return Err(crate::error::Error::Validation(
            "File path must be absolute".to_string()
        ));
    }
    
    // Get document type and its generation rule
    let doc_type = document_type::get_document_type(pool, &type_code).await?
        .ok_or_else(|| crate::error::Error::NotFound(
            format!("Document type '{}' not found", type_code.0)
        ))?;
    
    // Check if document type is active
    if !doc_type.active {
        return Err(crate::error::Error::Validation(
            format!("Document type '{}' is not active", type_code.0)
        ));
    }
    
    // Generate document number
    let document_number = generation_service::generate_document_number(
        pool,
        &doc_type.generation_rule,
        &type_code,
        &dept_code,
        &section_code,
    ).await?;
    
    // Create document path
    let now = Utc::now();
    let doc = DocumentPath {
        id: DocumentId::new(uuid::Uuid::new_v4().to_string()),
        document_number: document_number.clone(),
        document_type: type_code,
        department: dept_code,
        section: section_code,
        business_task,
        user: user_id,
        file_path,
        created_at: now,
        updated_at: now,
        generated: true,
        deleted: false,
    };
    
    // Save to database
    document_path::create_document_path(pool, &doc).await?;
    
    Ok(doc)
}

/// Create a document with manual number
pub async fn create_document_manual(
    pool: &SqlitePool,
    document_number: String,
    type_code: TypeCode,
    dept_code: DeptCode,
    section_code: SectionCode,
    user_id: UserId,
    file_path: PathBuf,
    business_task: Option<TaskId>,
) -> Result<DocumentPath> {
    // Validate file path is absolute
    if !file_path.is_absolute() {
        return Err(crate::error::Error::Validation(
            "File path must be absolute".to_string()
        ));
    }
    
    // Validate document number is not empty
    if document_number.trim().is_empty() {
        return Err(crate::error::Error::Validation(
            "Document number cannot be empty".to_string()
        ));
    }
    
    // Check if document type exists
    let doc_type = document_type::get_document_type(pool, &type_code).await?
        .ok_or_else(|| crate::error::Error::NotFound(
            format!("Document type '{}' not found", type_code.0)
        ))?;
    
    // Check if document type is active
    if !doc_type.active {
        return Err(crate::error::Error::Validation(
            format!("Document type '{}' is not active", type_code.0)
        ));
    }
    
    // Check if document number already exists
    if let Some(_existing) = document_path::get_document_path_by_number(pool, &document_number).await? {
        return Err(crate::error::Error::Validation(
            format!("Document number '{}' already exists", document_number)
        ));
    }
    
    // Create document path
    let now = Utc::now();
    let doc = DocumentPath {
        id: DocumentId::new(uuid::Uuid::new_v4().to_string()),
        document_number,
        document_type: type_code,
        department: dept_code,
        section: section_code,
        business_task,
        user: user_id,
        file_path,
        created_at: now,
        updated_at: now,
        generated: false,
        deleted: false,
    };
    
    // Save to database
    document_path::create_document_path(pool, &doc).await?;
    
    Ok(doc)
}

/// Update document path (file location)
pub async fn update_document_path(
    pool: &SqlitePool,
    id: &DocumentId,
    new_file_path: PathBuf,
) -> Result<DocumentPath> {
    // Validate file path is absolute
    if !new_file_path.is_absolute() {
        return Err(crate::error::Error::Validation(
            "File path must be absolute".to_string()
        ));
    }
    
    // Get existing document
    let mut doc = document_path::get_document_path(pool, id).await?
        .ok_or_else(|| crate::error::Error::NotFound(
            format!("Document '{}' not found", id.0)
        ))?;
    
    // Check if already deleted
    if doc.deleted {
        return Err(crate::error::Error::Validation(
            "Cannot update deleted document".to_string()
        ));
    }
    
    // Update path and timestamp
    doc.file_path = new_file_path.clone();
    doc.updated_at = Utc::now();
    
    // Save to database
    document_path::update_document_path(pool, &doc.id, new_file_path).await?;
    
    Ok(doc)
}

/// Logically delete a document
pub async fn delete_document(
    pool: &SqlitePool,
    id: &DocumentId,
) -> Result<()> {
    // Get existing document
    let doc = document_path::get_document_path(pool, id).await?
        .ok_or_else(|| crate::error::Error::NotFound(
            format!("Document '{}' not found", id.0)
        ))?;
    
    // Check if already deleted
    if doc.deleted {
        return Err(crate::error::Error::Validation(
            "Document is already deleted".to_string()
        ));
    }
    
    // Delete the document (logical deletion)
    document_path::delete_document_path(pool, id).await?;
    
    Ok(())
}

/// Get document by ID
pub async fn get_document_by_id(
    pool: &SqlitePool,
    id: &DocumentId,
) -> Result<Option<DocumentPath>> {
    document_path::get_document_path(pool, id).await
}

/// Get document by number
pub async fn get_document_by_number(
    pool: &SqlitePool,
    document_number: &str,
) -> Result<Option<DocumentPath>> {
    document_path::get_document_path_by_number(pool, document_number).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db::init_db_pool;
    use crate::storage::{department, section, user};
    use crate::models::{Department, Section, User, PathGenerationRule, DocumentType};

    async fn setup_test_data(pool: &SqlitePool) -> anyhow::Result<()> {
        // Create department
        let dept = Department::new('G', "総務");
        department::create_department(pool, &dept).await?;
        
        // Create section
        let sec = Section {
            code: SectionCode::new('I'),
            name: "インフラ".to_string(),
            department: DeptCode::new('G'),
        };
        section::create_section(pool, &sec).await?;
        
        // Create user
        let u = User::new("user001", "田川太郎", 'G', 'I');
        user::create_user(pool, &u).await?;
        
        // Create document type
        let rule = PathGenerationRule::example_agi();
        let doc_type = DocumentType::new("A", "契約書", "/docs/contracts/", rule);
        document_type::create_document_type(pool, &doc_type).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_document_auto() -> anyhow::Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        setup_test_data(&pool).await?;
        
        let doc = create_document_auto(
            &pool,
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/contracts/test.pdf"),
            None,
        ).await?;
        
        assert!(doc.generated);
        assert!(!doc.document_number.is_empty());
        assert_eq!(doc.document_type.0, "A");
        Ok(())
    }

    #[tokio::test]
    async fn test_create_document_manual() -> anyhow::Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        setup_test_data(&pool).await?;
        
        let doc = create_document_manual(
            &pool,
            "MANUAL-001".to_string(),
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/contracts/manual.pdf"),
            None,
        ).await?;
        
        assert!(!doc.generated);
        assert_eq!(doc.document_number, "MANUAL-001");
        Ok(())
    }

    #[tokio::test]
    async fn test_update_document_path() -> anyhow::Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        setup_test_data(&pool).await?;
        
        let doc = create_document_auto(
            &pool,
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/contracts/old.pdf"),
            None,
        ).await?;
        
        let updated = update_document_path(
            &pool,
            &doc.id,
            PathBuf::from("/docs/contracts/new.pdf"),
        ).await?;
        
        assert_eq!(updated.file_path, PathBuf::from("/docs/contracts/new.pdf"));
        assert!(updated.updated_at > doc.created_at);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_document() -> anyhow::Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        setup_test_data(&pool).await?;
        
        let doc = create_document_auto(
            &pool,
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/contracts/test.pdf"),
            None,
        ).await?;
        
        delete_document(&pool, &doc.id).await?;
        
        let deleted = get_document_by_id(&pool, &doc.id).await?
            .ok_or_else(|| crate::error::Error::NotFound(
                format!("Document with id {} not found after deletion", doc.id.0)
            ))?;
        assert!(deleted.deleted);
        Ok(())
    }
}

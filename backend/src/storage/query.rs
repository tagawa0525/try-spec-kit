//! Query operations for document paths

use crate::error::Result;
use crate::models::{DeptCode, DocumentId, DocumentPath, SectionCode, TaskId, TypeCode, UserId};
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use std::path::PathBuf;

/// Get all documents (respects deleted flag by default)
pub async fn get_all_documents(
    pool: &SqlitePool,
    include_deleted: bool,
) -> Result<Vec<DocumentPath>> {
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

/// Get documents by type
pub async fn get_documents_by_type(
    pool: &SqlitePool,
    type_code: &TypeCode,
    include_deleted: bool,
) -> Result<Vec<DocumentPath>> {
    let deleted_filter = if include_deleted { 1 } else { 0 };

    let rows = sqlx::query!(
        r#"
        SELECT id, document_number, document_type_code, department_code, section_code,
               business_task_id, user_id, file_path, created_at, updated_at, generated, deleted
        FROM documents
        WHERE document_type_code = ? AND (deleted = 0 OR ? = 1)
        ORDER BY created_at DESC
        "#,
        type_code.0,
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

/// Get documents by department
pub async fn get_documents_by_department(
    pool: &SqlitePool,
    dept_code: &DeptCode,
    include_deleted: bool,
) -> Result<Vec<DocumentPath>> {
    let dept_code_str = dept_code.0.to_string();
    let deleted_filter = if include_deleted { 1 } else { 0 };

    let rows = sqlx::query!(
        r#"
        SELECT id, document_number, document_type_code, department_code, section_code,
               business_task_id, user_id, file_path, created_at, updated_at, generated, deleted
        FROM documents
        WHERE department_code = ? AND (deleted = 0 OR ? = 1)
        ORDER BY created_at DESC
        "#,
        dept_code_str,
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

/// Get documents by section
pub async fn get_documents_by_section(
    pool: &SqlitePool,
    section_code: &SectionCode,
    include_deleted: bool,
) -> Result<Vec<DocumentPath>> {
    let section_code_str = section_code.0.to_string();
    let deleted_filter = if include_deleted { 1 } else { 0 };

    let rows = sqlx::query!(
        r#"
        SELECT id, document_number, document_type_code, department_code, section_code,
               business_task_id, user_id, file_path, created_at, updated_at, generated, deleted
        FROM documents
        WHERE section_code = ? AND (deleted = 0 OR ? = 1)
        ORDER BY created_at DESC
        "#,
        section_code_str,
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

/// Get documents by business task
pub async fn get_documents_by_task(
    pool: &SqlitePool,
    task_id: &TaskId,
    include_deleted: bool,
) -> Result<Vec<DocumentPath>> {
    let deleted_filter = if include_deleted { 1 } else { 0 };

    let rows = sqlx::query!(
        r#"
        SELECT id, document_number, document_type_code, department_code, section_code,
               business_task_id, user_id, file_path, created_at, updated_at, generated, deleted
        FROM documents
        WHERE business_task_id = ? AND (deleted = 0 OR ? = 1)
        ORDER BY created_at DESC
        "#,
        task_id.0,
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

/// Search documents by text (searches document number and file path)
pub async fn search_documents_by_text(
    pool: &SqlitePool,
    search_text: &str,
    include_deleted: bool,
) -> Result<Vec<DocumentPath>> {
    let deleted_filter = if include_deleted { 1 } else { 0 };
    let search_pattern = format!("%{}%", search_text);

    let rows = sqlx::query!(
        r#"
        SELECT id, document_number, document_type_code, department_code, section_code,
               business_task_id, user_id, file_path, created_at, updated_at, generated, deleted
        FROM documents
        WHERE (document_number LIKE ? OR file_path LIKE ?) 
              AND (deleted = 0 OR ? = 1)
        ORDER BY created_at DESC
        "#,
        search_pattern,
        search_pattern,
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

/// Search documents with multiple criteria (builder pattern)
pub struct DocumentQuery {
    type_code: Option<TypeCode>,
    department: Option<DeptCode>,
    section: Option<SectionCode>,
    task: Option<TaskId>,
    user: Option<UserId>,
    include_deleted: bool,
}

impl DocumentQuery {
    pub fn new() -> Self {
        Self {
            type_code: None,
            department: None,
            section: None,
            task: None,
            user: None,
            include_deleted: false,
        }
    }

    pub fn type_code(mut self, code: TypeCode) -> Self {
        self.type_code = Some(code);
        self
    }

    pub fn department(mut self, dept: DeptCode) -> Self {
        self.department = Some(dept);
        self
    }

    pub fn section(mut self, sec: SectionCode) -> Self {
        self.section = Some(sec);
        self
    }

    pub fn task(mut self, task_id: TaskId) -> Self {
        self.task = Some(task_id);
        self
    }

    pub fn user(mut self, user_id: UserId) -> Self {
        self.user = Some(user_id);
        self
    }

    pub fn include_deleted(mut self, include: bool) -> Self {
        self.include_deleted = include;
        self
    }

    pub async fn execute(self, _pool: &SqlitePool) -> Result<Vec<DocumentPath>> {
        // Build WHERE clause dynamically
        let mut where_clauses = Vec::new();
        let mut params: Vec<String> = Vec::new();

        if let Some(ref tc) = self.type_code {
            where_clauses.push(format!("document_type_code = ?{}", params.len() + 1));
            params.push(tc.0.clone());
        }

        if let Some(ref dept) = self.department {
            where_clauses.push(format!("department_code = ?{}", params.len() + 1));
            params.push(dept.0.to_string());
        }

        if let Some(ref sec) = self.section {
            where_clauses.push(format!("section_code = ?{}", params.len() + 1));
            params.push(sec.0.to_string());
        }

        if let Some(ref task) = self.task {
            where_clauses.push(format!("business_task_id = ?{}", params.len() + 1));
            params.push(task.0.clone());
        }

        if let Some(ref user) = self.user {
            where_clauses.push(format!("user_id = ?{}", params.len() + 1));
            params.push(user.0.clone());
        }

        if !self.include_deleted {
            where_clauses.push("deleted = 0".to_string());
        }

        // For now, this is a placeholder implementation
        // A full implementation would use dynamic query building with proper parameter binding
        Ok(Vec::new())
    }
}

impl Default for DocumentQuery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        Department, DocumentPath, DocumentType, PathGenerationRule, Section, User,
    };
    use crate::storage::db::init_db_pool;
    use crate::storage::{department, document_path, document_type, section, user};

    async fn setup_test_data(pool: &SqlitePool) -> Result<(DocumentPath, DocumentPath)> {
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

        // Create document types
        let rule_a = PathGenerationRule::example_agi();
        let doc_type_a = DocumentType::new("A", "契約書", "/docs/contracts/", rule_a);
        document_type::create_document_type(pool, &doc_type_a).await?;

        let rule_b = PathGenerationRule::example_ringi();
        let doc_type_b = DocumentType::new("りん議", "稟議書", "/docs/ringi/", rule_b);
        document_type::create_document_type(pool, &doc_type_b).await?;

        // Create documents
        let doc1 = DocumentPath::new_auto(
            "AGI-2509001",
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/contracts/AGI-2509001.pdf"),
        );

        let doc2 = DocumentPath::new_auto(
            "りん議I-25009",
            TypeCode::new("りん議"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/ringi/りん議I-25009.pdf"),
        );

        document_path::create_document_path(pool, &doc1).await?;
        document_path::create_document_path(pool, &doc2).await?;

        Ok((doc1, doc2))
    }

    #[tokio::test]
    async fn test_get_all_documents() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        setup_test_data(&pool).await?;

        let docs = get_all_documents(&pool, false).await?;
        assert_eq!(docs.len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_documents_by_type() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        setup_test_data(&pool).await?;

        let docs = get_documents_by_type(&pool, &TypeCode::new("A"), false).await?;
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].document_number, "AGI-2509001");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_documents_by_department() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        setup_test_data(&pool).await?;

        let docs = get_documents_by_department(&pool, &DeptCode::new('G'), false).await?;
        assert_eq!(docs.len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_documents_by_section() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        setup_test_data(&pool).await?;

        let docs = get_documents_by_section(&pool, &SectionCode::new('I'), false).await?;
        assert_eq!(docs.len(), 2);
        Ok(())
    }
}

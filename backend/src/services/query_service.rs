//! Query service for document searches

use crate::error::Result;
use crate::models::{DeptCode, DocumentPath, SectionCode, TaskId, TypeCode};
use crate::storage::query;
use sqlx::SqlitePool;

/// Get all documents
pub async fn get_all_documents(
    pool: &SqlitePool,
    include_deleted: bool,
) -> Result<Vec<DocumentPath>> {
    query::get_all_documents(pool, include_deleted).await
}

/// Get documents by type
pub async fn get_documents_by_type(
    pool: &SqlitePool,
    type_code: &TypeCode,
    include_deleted: bool,
) -> Result<Vec<DocumentPath>> {
    query::get_documents_by_type(pool, type_code, include_deleted).await
}

/// Get documents by department
pub async fn get_documents_by_department(
    pool: &SqlitePool,
    dept_code: &DeptCode,
    include_deleted: bool,
) -> Result<Vec<DocumentPath>> {
    query::get_documents_by_department(pool, dept_code, include_deleted).await
}

/// Get documents by section
pub async fn get_documents_by_section(
    pool: &SqlitePool,
    section_code: &SectionCode,
    include_deleted: bool,
) -> Result<Vec<DocumentPath>> {
    query::get_documents_by_section(pool, section_code, include_deleted).await
}

/// Get documents by business task
pub async fn get_documents_by_task(
    pool: &SqlitePool,
    task_id: &TaskId,
    include_deleted: bool,
) -> Result<Vec<DocumentPath>> {
    query::get_documents_by_task(pool, task_id, include_deleted).await
}

/// Document query builder for complex searches
pub struct DocumentQueryBuilder {
    type_code: Option<TypeCode>,
    department: Option<DeptCode>,
    section: Option<SectionCode>,
    task: Option<TaskId>,
    include_deleted: bool,
}

impl DocumentQueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self {
            type_code: None,
            department: None,
            section: None,
            task: None,
            include_deleted: false,
        }
    }

    /// Filter by document type
    pub fn type_code(mut self, code: TypeCode) -> Self {
        self.type_code = Some(code);
        self
    }

    /// Filter by department
    pub fn department(mut self, dept: DeptCode) -> Self {
        self.department = Some(dept);
        self
    }

    /// Filter by section
    pub fn section(mut self, sec: SectionCode) -> Self {
        self.section = Some(sec);
        self
    }

    /// Filter by business task
    pub fn task(mut self, task_id: TaskId) -> Self {
        self.task = Some(task_id);
        self
    }

    /// Include deleted documents in results
    pub fn include_deleted(mut self, include: bool) -> Self {
        self.include_deleted = include;
        self
    }

    /// Execute the query
    pub async fn execute(self, pool: &SqlitePool) -> Result<Vec<DocumentPath>> {
        // Get all documents first
        let mut docs = query::get_all_documents(pool, self.include_deleted).await?;

        // Filter by type_code if specified
        if let Some(ref tc) = self.type_code {
            docs.retain(|d| &d.document_type == tc);
        }

        // Filter by department if specified
        if let Some(ref dept) = self.department {
            docs.retain(|d| &d.department == dept);
        }

        // Filter by section if specified
        if let Some(ref sec) = self.section {
            docs.retain(|d| &d.section == sec);
        }

        // Filter by task if specified
        if let Some(ref task) = self.task {
            docs.retain(|d| {
                if let Some(ref dt) = d.business_task {
                    dt == task
                } else {
                    false
                }
            });
        }

        Ok(docs)
    }
}

impl Default for DocumentQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Department, DocumentType, PathGenerationRule, Section, User, UserId};
    use crate::storage::db::init_db_pool;
    use crate::storage::{department, document_path, document_type, section, user};
    use std::path::PathBuf;

    async fn setup_test_data(pool: &SqlitePool) -> Result<()> {
        // Create organization
        let dept = Department::new('G', "総務");
        department::create_department(pool, &dept).await?;

        let sec = Section {
            code: SectionCode::new('I'),
            name: "インフラ".to_string(),
            department: DeptCode::new('G'),
        };
        section::create_section(pool, &sec).await?;

        let usr = User::new("user001", "田川太郎", 'G', 'I');
        user::create_user(pool, &usr).await?;

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
        Ok(())
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
        assert_eq!(docs[0].document_type.0, "A");
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
    async fn test_query_builder() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        setup_test_data(&pool).await?;

        let docs = DocumentQueryBuilder::new()
            .type_code(TypeCode::new("A"))
            .department(DeptCode::new('G'))
            .execute(&pool)
            .await?;

        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].document_type.0, "A");
        Ok(())
    }

    #[tokio::test]
    async fn test_query_builder_section() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        setup_test_data(&pool).await?;

        let docs = DocumentQueryBuilder::new()
            .section(SectionCode::new('I'))
            .execute(&pool)
            .await?;

        assert_eq!(docs.len(), 2);
        Ok(())
    }
}

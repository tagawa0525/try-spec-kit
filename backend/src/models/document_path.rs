//! Document Path entity

use crate::models::{DeptCode, DocumentId, SectionCode, TaskId, TypeCode, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Document Path (文書パス)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentPath {
    /// 文書固有識別子
    pub id: DocumentId,
    /// 文書番号 (例: "AGI2509001", "りん議I-25009")
    pub document_number: String,
    /// 文書種類
    pub document_type: TypeCode,
    /// 所属部門
    pub department: DeptCode,
    /// 所属課
    pub section: SectionCode,
    /// 関連業務タスク (オプショナル)
    pub business_task: Option<TaskId>,
    /// 作成ユーザー
    pub user: UserId,
    /// ファイルパス (絶対パス)
    pub file_path: PathBuf,
    /// 作成日時
    pub created_at: DateTime<Utc>,
    /// 更新日時
    pub updated_at: DateTime<Utc>,
    /// 自動生成フラグ (true=auto-generated, false=manual)
    pub generated: bool,
    /// 論理削除フラグ
    pub deleted: bool,
}

impl DocumentPath {
    /// Create a new document path (for auto-generated documents)
    pub fn new_auto(
        document_number: impl Into<String>,
        document_type: TypeCode,
        department: DeptCode,
        section: SectionCode,
        user: UserId,
        file_path: PathBuf,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: DocumentId::new(uuid::Uuid::new_v4().to_string()),
            document_number: document_number.into(),
            document_type,
            department,
            section,
            business_task: None,
            user,
            file_path,
            created_at: now,
            updated_at: now,
            generated: true,
            deleted: false,
        }
    }

    /// Create a new document path (for manually added documents)
    pub fn new_manual(
        document_number: impl Into<String>,
        document_type: TypeCode,
        department: DeptCode,
        section: SectionCode,
        user: UserId,
        file_path: PathBuf,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: DocumentId::new(uuid::Uuid::new_v4().to_string()),
            document_number: document_number.into(),
            document_type,
            department,
            section,
            business_task: None,
            user,
            file_path,
            created_at: now,
            updated_at: now,
            generated: false, // Manual documents
            deleted: false,
        }
    }

    /// Associate with a business task
    pub fn with_task(mut self, task_id: TaskId) -> Self {
        self.business_task = Some(task_id);
        self
    }

    /// Update the file path
    pub fn update_path(&mut self, new_path: PathBuf) {
        self.file_path = new_path;
        self.updated_at = Utc::now();
    }

    /// Logically delete the document
    pub fn delete(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    // Getters
    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn document_number(&self) -> &str {
        &self.document_number
    }

    pub fn document_type(&self) -> &TypeCode {
        &self.document_type
    }

    pub fn department(&self) -> &DeptCode {
        &self.department
    }

    pub fn section(&self) -> &SectionCode {
        &self.section
    }

    pub fn business_task(&self) -> Option<&TaskId> {
        self.business_task.as_ref()
    }

    pub fn user(&self) -> &UserId {
        &self.user
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn generated(&self) -> bool {
        self.generated
    }

    pub fn deleted(&self) -> bool {
        self.deleted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_path_new_auto() {
        let doc = DocumentPath::new_auto(
            "AGI2509001",
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/contracts/AGI2509001.pdf"),
        );

        assert_eq!(doc.document_number, "AGI2509001");
        assert!(doc.generated);
        assert!(!doc.deleted);
        assert!(doc.business_task.is_none());
    }

    #[test]
    fn test_document_path_new_manual() {
        let doc = DocumentPath::new_manual(
            "MANUAL-001",
            TypeCode::new("D"),
            DeptCode::new('K'),
            SectionCode::new('T'),
            UserId::new("user002"),
            PathBuf::from("/external/import.pdf"),
        );

        assert_eq!(doc.document_number, "MANUAL-001");
        assert!(!doc.generated);
        assert!(!doc.deleted);
    }

    #[test]
    fn test_document_path_with_task() {
        let doc = DocumentPath::new_auto(
            "AGI2509001",
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/test.pdf"),
        )
        .with_task(TaskId::new("task001"));

        assert!(doc.business_task.is_some());
        assert_eq!(doc.business_task.map(|t| t.0), Some("task001".to_string()));
    }

    #[test]
    fn test_document_path_update_path() {
        let mut doc = DocumentPath::new_auto(
            "AGI2509001",
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/old/path.pdf"),
        );

        let old_updated_at = doc.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));

        doc.update_path(PathBuf::from("/new/path.pdf"));

        assert_eq!(doc.file_path, PathBuf::from("/new/path.pdf"));
        assert!(doc.updated_at > old_updated_at);
    }

    #[test]
    fn test_document_path_delete() {
        let mut doc = DocumentPath::new_auto(
            "AGI2509001",
            TypeCode::new("A"),
            DeptCode::new('G'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/test.pdf"),
        );

        assert!(!doc.deleted);
        doc.delete();
        assert!(doc.deleted);
    }

    #[test]
    fn test_document_path_multibyte_number() {
        let doc = DocumentPath::new_auto(
            "りん議I-25009",
            TypeCode::new("りん議"),
            DeptCode::new('K'),
            SectionCode::new('I'),
            UserId::new("user001"),
            PathBuf::from("/docs/ringi/りん議I-25009.pdf"),
        );

        assert_eq!(doc.document_number, "りん議I-25009");
        assert_eq!(doc.document_type.0, "りん議");
    }
}

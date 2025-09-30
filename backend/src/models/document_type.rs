//! Document Type entity

use serde::{Deserialize, Serialize};
use crate::models::{TypeCode, PathGenerationRule};

/// Document Type (文書種類)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentType {
    /// 文書種類コード (1-3文字、マルチバイト可)
    pub code: TypeCode,
    /// 説明
    pub description: String,
    /// ルートディレクトリ
    pub root_directory: String,
    /// 番号生成ルール
    pub generation_rule: PathGenerationRule,
    /// アクティブ/非アクティブ状態
    pub active: bool,
}

impl DocumentType {
    pub fn new(
        code: impl Into<String>,
        description: impl Into<String>,
        root_directory: impl Into<String>,
        generation_rule: PathGenerationRule,
    ) -> Self {
        Self {
            code: TypeCode::new(code),
            description: description.into(),
            root_directory: root_directory.into(),
            generation_rule,
            active: true,
        }
    }

    pub fn inactive(mut self) -> Self {
        self.active = false;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_type_new() {
        let rule = PathGenerationRule::example_agi();
        let doc_type = DocumentType::new("A", "契約書", "/docs/contracts/", rule);
        
        assert_eq!(doc_type.code.0, "A");
        assert_eq!(doc_type.description, "契約書");
        assert_eq!(doc_type.root_directory, "/docs/contracts/");
        assert!(doc_type.active);
    }

    #[test]
    fn test_document_type_multibyte_code() {
        let rule = PathGenerationRule::example_ringi();
        let doc_type = DocumentType::new("りん議", "稟議書", "/docs/ringi/", rule);
        
        assert_eq!(doc_type.code.0, "りん議");
        assert_eq!(doc_type.description, "稟議書");
    }

    #[test]
    fn test_document_type_inactive() {
        let rule = PathGenerationRule::example_agi();
        let doc_type = DocumentType::new("A", "契約書", "/docs/", rule).inactive();
        assert!(!doc_type.active);
    }
}

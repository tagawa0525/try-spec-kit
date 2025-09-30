//! Section entity

use serde::{Deserialize, Serialize};
use crate::models::{DeptCode, SectionCode};

/// Section (課)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Section {
    /// 課コード (1文字、例: "I" = インフラ, "T" = 技術)
    pub code: SectionCode,
    /// 課名
    pub name: String,
    /// 親部門への参照
    pub department: DeptCode,
}

impl Section {
    pub fn new(code: char, name: impl Into<String>, department: char) -> Self {
        Self {
            code: SectionCode::new(code),
            name: name.into(),
            department: DeptCode::new(department),
        }
    }
    
    // Getters
    pub fn code(&self) -> &SectionCode {
        &self.code
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn department(&self) -> &DeptCode {
        &self.department
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_new() {
        let section = Section::new('I', "インフラ", 'G');
        assert_eq!(section.code.0, 'I');
        assert_eq!(section.name, "インフラ");
        assert_eq!(section.department.0, 'G');
    }
}

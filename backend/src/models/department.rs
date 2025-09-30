//! Department entity

use serde::{Deserialize, Serialize};
use crate::models::{DeptCode, SectionCode};

/// Department (部門)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Department {
    /// 部門コード (1文字、例: "G" = 総務, "K" = 分析)
    pub code: DeptCode,
    /// 部門名
    pub name: String,
    /// この部門に属する課のリスト
    pub sections: Vec<SectionCode>,
}

impl Department {
    pub fn new(code: char, name: impl Into<String>) -> Self {
        Self {
            code: DeptCode::new(code),
            name: name.into(),
            sections: Vec::new(),
        }
    }

    pub fn with_sections(mut self, sections: Vec<SectionCode>) -> Self {
        self.sections = sections;
        self
    }

    pub fn add_section(&mut self, section: SectionCode) {
        if !self.sections.contains(&section) {
            self.sections.push(section);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_department_new() {
        let dept = Department::new('G', "総務");
        assert_eq!(dept.code.0, 'G');
        assert_eq!(dept.name, "総務");
        assert!(dept.sections.is_empty());
    }

    #[test]
    fn test_department_add_section() {
        let mut dept = Department::new('G', "総務");
        dept.add_section(SectionCode::new('I'));
        dept.add_section(SectionCode::new('T'));
        assert_eq!(dept.sections.len(), 2);
    }

    #[test]
    fn test_department_with_sections() {
        let dept = Department::new('G', "総務")
            .with_sections(vec![SectionCode::new('I'), SectionCode::new('T')]);
        assert_eq!(dept.sections.len(), 2);
    }
}

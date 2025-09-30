//! Newtype wrappers for type safety

use serde::{Deserialize, Serialize};
use std::fmt;

/// Department code (single uppercase letter)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeptCode(pub char);

impl DeptCode {
    pub fn new(code: char) -> Self {
        Self(code)
    }
}

impl fmt::Display for DeptCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Section code (single uppercase letter)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SectionCode(pub char);

impl SectionCode {
    pub fn new(code: char) -> Self {
        Self(code)
    }
}

impl fmt::Display for SectionCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// User ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub String);

impl UserId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Business Task ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub String);

impl TaskId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Document ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(pub String);

impl DocumentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for DocumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Document Type Code (1-3 characters, can include multi-byte)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeCode(pub String);

impl TypeCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
}

impl fmt::Display for TypeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dept_code_new() {
        let dept = DeptCode::new('G');
        assert_eq!(dept.0, 'G');
        assert_eq!(dept.to_string(), "G");
    }

    #[test]
    fn test_type_code_multibyte() {
        let type_code = TypeCode::new("りん議");
        assert_eq!(type_code.0, "りん議");
        assert_eq!(type_code.to_string(), "りん議");
    }

    #[test]
    fn test_user_id_new() {
        let user_id = UserId::new("user001");
        assert_eq!(user_id.0, "user001");
    }
}

//! User entity

use serde::{Deserialize, Serialize};
use crate::models::{DeptCode, SectionCode, UserId, Permissions};

/// User (ユーザー)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// ユーザー固有識別子
    pub id: UserId,
    /// ユーザー名
    pub name: String,
    /// 所属部門
    pub department: DeptCode,
    /// 所属課
    pub section: SectionCode,
    /// アクセス権限
    pub permissions: Permissions,
}

impl User {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        department: char,
        section: char,
    ) -> Self {
        Self {
            id: UserId::new(id),
            name: name.into(),
            department: DeptCode::new(department),
            section: SectionCode::new(section),
            permissions: Permissions::default(),
        }
    }

    pub fn with_permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = permissions;
        self
    }
    
    // Getters
    pub fn id(&self) -> &UserId {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn department(&self) -> &DeptCode {
        &self.department
    }
    
    pub fn section(&self) -> &SectionCode {
        &self.section
    }
    
    pub fn permissions(&self) -> &Permissions {
        &self.permissions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_new() {
        let user = User::new("user001", "田川太郎", 'G', 'I');
        assert_eq!(user.id.0, "user001");
        assert_eq!(user.name, "田川太郎");
        assert_eq!(user.department.0, 'G');
        assert_eq!(user.section.0, 'I');
        assert!(user.permissions.can_read);
    }

    #[test]
    fn test_user_with_permissions() {
        let user = User::new("user001", "Admin", 'G', 'I')
            .with_permissions(Permissions::all());
        assert!(user.permissions.can_delete);
    }
}

//! User permissions

use serde::{Deserialize, Serialize};

/// User access permissions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Permissions {
    pub can_create: bool,
    pub can_update: bool,
    pub can_delete: bool,
    pub can_read: bool,
}

impl Default for Permissions {
    fn default() -> Self {
        Self {
            can_create: true,
            can_update: true,
            can_delete: false, // Deletion requires special permission
            can_read: true,
        }
    }
}

impl Permissions {
    /// Create permissions with all access
    pub fn all() -> Self {
        Self {
            can_create: true,
            can_update: true,
            can_delete: true,
            can_read: true,
        }
    }

    /// Create read-only permissions
    pub fn read_only() -> Self {
        Self {
            can_create: false,
            can_update: false,
            can_delete: false,
            can_read: true,
        }
    }

    /// Check if user has write access (create or update)
    pub fn can_write(&self) -> bool {
        self.can_create || self.can_update
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permissions_default() {
        let perms = Permissions::default();
        assert!(perms.can_create);
        assert!(perms.can_update);
        assert!(!perms.can_delete); // Default: no delete permission
        assert!(perms.can_read);
    }

    #[test]
    fn test_permissions_read_only() {
        let perms = Permissions::read_only();
        assert!(!perms.can_create);
        assert!(!perms.can_update);
        assert!(!perms.can_delete);
        assert!(perms.can_read);
        assert!(!perms.can_write());
    }

    #[test]
    fn test_permissions_all() {
        let perms = Permissions::all();
        assert!(perms.can_create);
        assert!(perms.can_update);
        assert!(perms.can_delete);
        assert!(perms.can_read);
        assert!(perms.can_write());
    }
}

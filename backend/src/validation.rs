//! Input validation utilities

use crate::error::{Error, Result};
use std::path::Path;

/// Validate that a path is absolute
pub fn validate_absolute_path(path: &Path) -> Result<()> {
    if !path.is_absolute() {
        return Err(Error::RelativePathNotAllowed);
    }
    Ok(())
}

/// Validate document type code format (1-3 characters, can include multi-byte)
pub fn validate_type_code(code: &str) -> Result<()> {
    if code.is_empty() || code.chars().count() > 3 {
        return Err(Error::InvalidTypeCode(code.to_string()));
    }
    Ok(())
}

/// Validate department code (single uppercase letter)
pub fn validate_dept_code(code: char) -> Result<()> {
    if !code.is_ascii_uppercase() {
        return Err(Error::DepartmentNotFound(code));
    }
    Ok(())
}

/// Validate section code (single uppercase letter)
pub fn validate_section_code(code: char) -> Result<()> {
    if !code.is_ascii_uppercase() {
        return Err(Error::SectionNotFound(code));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_absolute_path() {
        assert!(validate_absolute_path(Path::new("/absolute/path")).is_ok());
        assert!(validate_absolute_path(Path::new("relative/path")).is_err());
    }

    #[test]
    fn test_validate_type_code() {
        assert!(validate_type_code("A").is_ok());
        assert!(validate_type_code("ABC").is_ok());
        assert!(validate_type_code("りん議").is_ok());
        assert!(validate_type_code("").is_err());
        assert!(validate_type_code("ABCD").is_err());
    }

    #[test]
    fn test_validate_dept_code() {
        assert!(validate_dept_code('G').is_ok());
        assert!(validate_dept_code('K').is_ok());
        assert!(validate_dept_code('a').is_err());
        assert!(validate_dept_code('1').is_err());
    }
}

//! Organization management service (departments, sections, users)

use crate::error::Result;
use crate::models::{Department, DeptCode, Section, SectionCode, User, UserId};
use crate::storage::{department, section, user};
use sqlx::SqlitePool;

/// Create a new department
pub async fn create_department(pool: &SqlitePool, dept: Department) -> Result<Department> {
    department::create_department(pool, &dept).await?;
    Ok(dept)
}

/// Get a department by code
pub async fn get_department(pool: &SqlitePool, code: &DeptCode) -> Result<Option<Department>> {
    department::get_department(pool, code).await
}

/// List all departments
pub async fn list_departments(pool: &SqlitePool) -> Result<Vec<Department>> {
    department::list_departments(pool).await
}

/// Update a department
pub async fn update_department(pool: &SqlitePool, dept: Department) -> Result<Department> {
    department::update_department(pool, &dept).await?;
    Ok(dept)
}

/// Delete a department
pub async fn delete_department(pool: &SqlitePool, code: &DeptCode) -> Result<()> {
    department::delete_department(pool, code).await
}

/// Create a new section
pub async fn create_section(pool: &SqlitePool, sec: Section) -> Result<Section> {
    // Validate that department exists
    let _dept = department::get_department(pool, &sec.department)
        .await?
        .ok_or_else(|| {
            crate::error::Error::NotFound(format!("Department '{}' not found", sec.department.0))
        })?;

    section::create_section(pool, &sec).await?;
    Ok(sec)
}

/// Get a section by code
pub async fn get_section(pool: &SqlitePool, code: &SectionCode) -> Result<Option<Section>> {
    section::get_section(pool, code).await
}

/// List all sections
pub async fn list_sections(pool: &SqlitePool) -> Result<Vec<Section>> {
    section::list_sections(pool).await
}

/// List sections by department
pub async fn list_sections_by_department(
    pool: &SqlitePool,
    dept_code: &DeptCode,
) -> Result<Vec<Section>> {
    section::list_sections_by_department(pool, dept_code).await
}

/// Update a section
pub async fn update_section(pool: &SqlitePool, sec: Section) -> Result<Section> {
    section::update_section(pool, &sec).await?;
    Ok(sec)
}

/// Delete a section
pub async fn delete_section(pool: &SqlitePool, code: &SectionCode) -> Result<()> {
    section::delete_section(pool, code).await
}

/// Create a new user
pub async fn create_user(pool: &SqlitePool, usr: User) -> Result<User> {
    // Validate that department exists
    let _dept = department::get_department(pool, &usr.department)
        .await?
        .ok_or_else(|| {
            crate::error::Error::NotFound(format!("Department '{}' not found", usr.department.0))
        })?;

    // Validate that section exists
    let _sec = section::get_section(pool, &usr.section)
        .await?
        .ok_or_else(|| {
            crate::error::Error::NotFound(format!("Section '{}' not found", usr.section.0))
        })?;

    user::create_user(pool, &usr).await?;
    Ok(usr)
}

/// Get a user by ID
pub async fn get_user(pool: &SqlitePool, id: &UserId) -> Result<Option<User>> {
    user::get_user(pool, id).await
}

/// List all users
pub async fn list_users(pool: &SqlitePool) -> Result<Vec<User>> {
    user::list_users(pool).await
}

/// List users by department
pub async fn list_users_by_department(
    pool: &SqlitePool,
    dept_code: &DeptCode,
) -> Result<Vec<User>> {
    user::list_users_by_department(pool, dept_code).await
}

/// List users by section
pub async fn list_users_by_section(
    pool: &SqlitePool,
    section_code: &SectionCode,
) -> Result<Vec<User>> {
    user::list_users_by_section(pool, section_code).await
}

/// Update a user
pub async fn update_user(pool: &SqlitePool, usr: User) -> Result<User> {
    user::update_user(pool, &usr).await?;
    Ok(usr)
}

/// Delete a user
pub async fn delete_user(pool: &SqlitePool, id: &UserId) -> Result<()> {
    user::delete_user(pool, id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db::init_db_pool;

    #[tokio::test]
    async fn test_create_and_get_department() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;

        let dept = Department::new('G', "総務");
        create_department(&pool, dept.clone()).await?;

        let retrieved = get_department(&pool, &DeptCode::new('G')).await?;
        assert!(retrieved.is_some());
        if let Some(dept) = retrieved {
            assert_eq!(dept.name, "総務");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_create_section_with_validation() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;

        // Create department first
        let dept = Department::new('G', "総務");
        create_department(&pool, dept).await?;

        // Create section
        let sec = Section {
            code: SectionCode::new('I'),
            name: "インフラ".to_string(),
            department: DeptCode::new('G'),
        };
        create_section(&pool, sec).await?;

        let retrieved = get_section(&pool, &SectionCode::new('I')).await?;
        assert!(retrieved.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_section_invalid_department() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;

        // Try to create section without department
        let sec = Section {
            code: SectionCode::new('I'),
            name: "インフラ".to_string(),
            department: DeptCode::new('X'), // Non-existent
        };

        let result = create_section(&pool, sec).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_user_with_validation() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;

        // Setup organization
        let dept = Department::new('G', "総務");
        create_department(&pool, dept).await?;

        let sec = Section {
            code: SectionCode::new('I'),
            name: "インフラ".to_string(),
            department: DeptCode::new('G'),
        };
        create_section(&pool, sec).await?;

        // Create user
        let usr = User::new("user001", "田川太郎", 'G', 'I');
        create_user(&pool, usr).await?;

        let retrieved = get_user(&pool, &UserId::new("user001")).await?;
        assert!(retrieved.is_some());
        if let Some(user) = retrieved {
            assert_eq!(user.name, "田川太郎");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_list_sections_by_department() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;

        // Setup
        let dept = Department::new('G', "総務");
        create_department(&pool, dept).await?;

        let sec1 = Section {
            code: SectionCode::new('I'),
            name: "インフラ".to_string(),
            department: DeptCode::new('G'),
        };
        let sec2 = Section {
            code: SectionCode::new('J'),
            name: "人事".to_string(),
            department: DeptCode::new('G'),
        };
        create_section(&pool, sec1).await?;
        create_section(&pool, sec2).await?;

        let sections = list_sections_by_department(&pool, &DeptCode::new('G')).await?;
        assert_eq!(sections.len(), 2);
        Ok(())
    }
}

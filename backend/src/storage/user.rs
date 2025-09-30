//! User storage operations

use sqlx::SqlitePool;
use crate::error::Result;
use crate::models::{User, UserId, DeptCode, SectionCode, Permissions};

/// Create a new user
pub async fn create_user(pool: &SqlitePool, user: &User) -> Result<()> {
    let dept_code = user.department.0.to_string();
    let section_code = user.section.0.to_string();
    let can_create = user.permissions.can_create as i32;
    let can_update = user.permissions.can_update as i32;
    let can_delete = user.permissions.can_delete as i32;
    let can_read = user.permissions.can_read as i32;
    
    sqlx::query!(
        r#"
        INSERT INTO users (id, name, department_code, section_code, can_create, can_update, can_delete, can_read)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        user.id.0,
        user.name,
        dept_code,
        section_code,
        can_create,
        can_update,
        can_delete,
        can_read
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get a user by ID
pub async fn get_user(pool: &SqlitePool, id: &UserId) -> Result<Option<User>> {
    let row = sqlx::query!(
        r#"
        SELECT id, name, department_code, section_code, can_create, can_update, can_delete, can_read
        FROM users
        WHERE id = ?
        "#,
        id.0
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => {
            let dept_char = r.department_code.chars().next().unwrap_or('?');
            let section_char = r.section_code.chars().next().unwrap_or('?');
            
            Ok(Some(User {
                id: UserId::new(r.id),
                name: r.name,
                department: DeptCode::new(dept_char),
                section: SectionCode::new(section_char),
                permissions: Permissions {
                    can_create: r.can_create != 0,
                    can_update: r.can_update != 0,
                    can_delete: r.can_delete != 0,
                    can_read: r.can_read != 0,
                },
            }))
        }
        None => Ok(None),
    }
}

/// List all users
pub async fn list_users(pool: &SqlitePool) -> Result<Vec<User>> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name, department_code, section_code, can_create, can_update, can_delete, can_read
        FROM users
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    let users = rows
        .into_iter()
        .filter_map(|r| {
            let dept_char = r.department_code.chars().next()?;
            let section_char = r.section_code.chars().next()?;
            Some(User {
                id: UserId::new(r.id),
                name: r.name,
                department: DeptCode::new(dept_char),
                section: SectionCode::new(section_char),
                permissions: Permissions {
                    can_create: r.can_create != 0,
                    can_update: r.can_update != 0,
                    can_delete: r.can_delete != 0,
                    can_read: r.can_read != 0,
                },
            })
        })
        .collect();

    Ok(users)
}

/// List all users in a department
pub async fn list_users_by_department(
    pool: &SqlitePool,
    dept_code: &DeptCode,
) -> Result<Vec<User>> {
    let dept_code_str = dept_code.0.to_string();
    
    let rows = sqlx::query!(
        r#"
        SELECT id, name, department_code, section_code, can_create, can_update, can_delete, can_read
        FROM users
        WHERE department_code = ?
        ORDER BY id
        "#,
        dept_code_str
    )
    .fetch_all(pool)
    .await?;

    let users = rows
        .into_iter()
        .filter_map(|r| {
            let dept_char = r.department_code.chars().next()?;
            let section_char = r.section_code.chars().next()?;
            Some(User {
                id: UserId::new(r.id),
                name: r.name,
                department: DeptCode::new(dept_char),
                section: SectionCode::new(section_char),
                permissions: Permissions {
                    can_create: r.can_create != 0,
                    can_update: r.can_update != 0,
                    can_delete: r.can_delete != 0,
                    can_read: r.can_read != 0,
                },
            })
        })
        .collect();

    Ok(users)
}

/// List users by section
pub async fn list_users_by_section(pool: &SqlitePool, section_code: &SectionCode) -> Result<Vec<User>> {
    let section_code_str = section_code.0.to_string();
    
    let rows = sqlx::query!(
        r#"
        SELECT id, name, department_code, section_code, can_create, can_update, can_delete, can_read
        FROM users
        WHERE section_code = ?
        ORDER BY id
        "#,
        section_code_str
    )
    .fetch_all(pool)
    .await?;

    let users = rows
        .into_iter()
        .filter_map(|r| {
            let dept_char = r.department_code.chars().next()?;
            let section_char = r.section_code.chars().next()?;
            Some(User {
                id: UserId::new(r.id),
                name: r.name,
                department: DeptCode::new(dept_char),
                section: SectionCode::new(section_char),
                permissions: Permissions {
                    can_create: r.can_create != 0,
                    can_update: r.can_update != 0,
                    can_delete: r.can_delete != 0,
                    can_read: r.can_read != 0,
                },
            })
        })
        .collect();

    Ok(users)
}

/// Update a user
pub async fn update_user(pool: &SqlitePool, user: &User) -> Result<()> {
    let dept_code = user.department.0.to_string();
    let section_code = user.section.0.to_string();
    let can_create = user.permissions.can_create as i32;
    let can_update = user.permissions.can_update as i32;
    let can_delete = user.permissions.can_delete as i32;
    let can_read = user.permissions.can_read as i32;
    
    sqlx::query!(
        r#"
        UPDATE users
        SET name = ?, department_code = ?, section_code = ?, 
            can_create = ?, can_update = ?, can_delete = ?, can_read = ?
        WHERE id = ?
        "#,
        user.name,
        dept_code,
        section_code,
        can_create,
        can_update,
        can_delete,
        can_read,
        user.id.0
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a user
pub async fn delete_user(pool: &SqlitePool, id: &UserId) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = ?
        "#,
        id.0
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db::init_db_pool;
    use crate::storage::{department, section};

    #[tokio::test]
    async fn test_create_and_get_user() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        // Create department and section first
        let dept = crate::models::Department::new('G', "総務");
        department::create_department(&pool, &dept).await.unwrap();
        
        let sec = crate::models::Section {
            code: SectionCode::new('I'),
            name: "インフラ".to_string(),
            department: DeptCode::new('G'),
        };
        section::create_section(&pool, &sec).await.unwrap();
        
        let user = User::new("user001", "田川太郎", 'G', 'I');
        create_user(&pool, &user).await.unwrap();
        
        let retrieved = get_user(&pool, &UserId::new("user001")).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "田川太郎");
    }

    #[tokio::test]
    async fn test_list_users_by_department() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        // Create department and section
        let dept = crate::models::Department::new('G', "総務");
        department::create_department(&pool, &dept).await.unwrap();
        
        let sec = crate::models::Section {
            code: SectionCode::new('I'),
            name: "インフラ".to_string(),
            department: DeptCode::new('G'),
        };
        section::create_section(&pool, &sec).await.unwrap();
        
        // Create users
        let user1 = User::new("user001", "田川太郎", 'G', 'I');
        let user2 = User::new("user002", "山田花子", 'G', 'I');
        create_user(&pool, &user1).await.unwrap();
        create_user(&pool, &user2).await.unwrap();
        
        let list = list_users_by_department(&pool, &DeptCode::new('G')).await.unwrap();
        assert_eq!(list.len(), 2);
    }
}

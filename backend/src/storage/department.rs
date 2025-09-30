//! Department storage operations

use sqlx::SqlitePool;
use crate::error::Result;
use crate::models::{Department, DeptCode, SectionCode};

/// Create a new department
pub async fn create_department(pool: &SqlitePool, dept: &Department) -> Result<()> {
    let code = dept.code.0.to_string();
    
    sqlx::query!(
        r#"
        INSERT INTO departments (code, name)
        VALUES (?, ?)
        "#,
        code,
        dept.name
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get a department by code
pub async fn get_department(pool: &SqlitePool, code: &DeptCode) -> Result<Option<Department>> {
    let code_str = code.0.to_string();
    
    let row = sqlx::query!(
        r#"
        SELECT code, name
        FROM departments
        WHERE code = ?
        "#,
        code_str
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => {
            let code_char = r.code.chars().next().unwrap_or('?');
            let dept_code = DeptCode::new(code_char);
            
            // Get associated sections
            let sections = get_department_sections(pool, &dept_code).await?;
            
            Ok(Some(Department {
                code: dept_code,
                name: r.name,
                sections,
            }))
        }
        None => Ok(None),
    }
}

/// Get all sections for a department
async fn get_department_sections(pool: &SqlitePool, dept_code: &DeptCode) -> Result<Vec<SectionCode>> {
    let code_str = dept_code.0.to_string();
    
    let rows = sqlx::query!(
        r#"
        SELECT code
        FROM sections
        WHERE department_code = ?
        "#,
        code_str
    )
    .fetch_all(pool)
    .await?;

    let sections = rows
        .into_iter()
        .filter_map(|r| r.code.chars().next())
        .map(SectionCode::new)
        .collect();

    Ok(sections)
}

/// List all departments
pub async fn list_departments(pool: &SqlitePool) -> Result<Vec<Department>> {
    let rows = sqlx::query!(
        r#"
        SELECT code, name
        FROM departments
        ORDER BY code
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut departments = Vec::new();
    for row in rows {
        let code_char = row.code.chars().next().unwrap_or('?');
        let dept_code = DeptCode::new(code_char);
        let sections = get_department_sections(pool, &dept_code).await?;
        
        departments.push(Department {
            code: dept_code,
            name: row.name,
            sections,
        });
    }

    Ok(departments)
}

/// Update a department
pub async fn update_department(pool: &SqlitePool, dept: &Department) -> Result<()> {
    let code = dept.code.0.to_string();
    
    sqlx::query!(
        r#"
        UPDATE departments
        SET name = ?
        WHERE code = ?
        "#,
        dept.name,
        code
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a department
pub async fn delete_department(pool: &SqlitePool, code: &DeptCode) -> Result<()> {
    let code_str = code.0.to_string();
    
    sqlx::query!(
        r#"
        DELETE FROM departments
        WHERE code = ?
        "#,
        code_str
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db::init_db_pool;

    #[tokio::test]
    async fn test_create_and_get_department() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        let dept = Department::new('G', "総務");
        create_department(&pool, &dept).await.unwrap();
        
        let retrieved = get_department(&pool, &DeptCode::new('G')).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "総務");
    }

    #[tokio::test]
    async fn test_list_departments() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        let dept1 = Department::new('G', "総務");
        let dept2 = Department::new('K', "分析");
        create_department(&pool, &dept1).await.unwrap();
        create_department(&pool, &dept2).await.unwrap();
        
        let list = list_departments(&pool).await.unwrap();
        assert_eq!(list.len(), 2);
    }

    #[tokio::test]
    async fn test_update_department() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        let mut dept = Department::new('G', "総務");
        create_department(&pool, &dept).await.unwrap();
        
        dept.name = "総務部".to_string();
        update_department(&pool, &dept).await.unwrap();
        
        let retrieved = get_department(&pool, &DeptCode::new('G')).await.unwrap();
        assert_eq!(retrieved.unwrap().name, "総務部");
    }

    #[tokio::test]
    async fn test_delete_department() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        let dept = Department::new('G', "総務");
        create_department(&pool, &dept).await.unwrap();
        
        delete_department(&pool, &DeptCode::new('G')).await.unwrap();
        
        let retrieved = get_department(&pool, &DeptCode::new('G')).await.unwrap();
        assert!(retrieved.is_none());
    }
}

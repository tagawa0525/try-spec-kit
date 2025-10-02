//! Section storage operations

use crate::error::Result;
use crate::models::{DeptCode, Section, SectionCode};
use sqlx::SqlitePool;

/// Create a new section
pub async fn create_section(pool: &SqlitePool, section: &Section) -> Result<()> {
    let code = section.code.0.to_string();
    let dept_code = section.department.0.to_string();

    sqlx::query!(
        r#"
        INSERT INTO sections (code, name, department_code)
        VALUES (?, ?, ?)
        "#,
        code,
        section.name,
        dept_code
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get a section by code
pub async fn get_section(pool: &SqlitePool, code: &SectionCode) -> Result<Option<Section>> {
    let code_str = code.0.to_string();

    let row = sqlx::query!(
        r#"
        SELECT code, name, department_code
        FROM sections
        WHERE code = ?
        "#,
        code_str
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => {
            let code_char = r.code.chars().next().unwrap_or('?');
            let dept_char = r.department_code.chars().next().unwrap_or('?');

            Ok(Some(Section {
                code: SectionCode::new(code_char),
                name: r.name,
                department: DeptCode::new(dept_char),
            }))
        }
        None => Ok(None),
    }
}

/// List all sections
pub async fn list_sections(pool: &SqlitePool) -> Result<Vec<Section>> {
    let rows = sqlx::query!(
        r#"
        SELECT code, name, department_code
        FROM sections
        ORDER BY department_code, code
        "#
    )
    .fetch_all(pool)
    .await?;

    let sections = rows
        .into_iter()
        .filter_map(|r| {
            let code_char = r.code.chars().next()?;
            let dept_char = r.department_code.chars().next()?;
            Some(Section {
                code: SectionCode::new(code_char),
                name: r.name,
                department: DeptCode::new(dept_char),
            })
        })
        .collect();

    Ok(sections)
}

/// List sections by department
pub async fn list_sections_by_department(
    pool: &SqlitePool,
    dept_code: &DeptCode,
) -> Result<Vec<Section>> {
    let dept_code_str = dept_code.0.to_string();

    let rows = sqlx::query!(
        r#"
        SELECT code, name, department_code
        FROM sections
        WHERE department_code = ?
        ORDER BY code
        "#,
        dept_code_str
    )
    .fetch_all(pool)
    .await?;

    let sections = rows
        .into_iter()
        .filter_map(|r| {
            let code_char = r.code.chars().next()?;
            let dept_char = r.department_code.chars().next()?;
            Some(Section {
                code: SectionCode::new(code_char),
                name: r.name,
                department: DeptCode::new(dept_char),
            })
        })
        .collect();

    Ok(sections)
}

/// Update a section
pub async fn update_section(pool: &SqlitePool, section: &Section) -> Result<()> {
    let dept_code_str = section.department.0.to_string();
    let code_str = section.code.0.to_string();

    sqlx::query!(
        r#"
        UPDATE sections
        SET name = ?, department_code = ?
        WHERE code = ?
        "#,
        section.name,
        dept_code_str,
        code_str
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a section
pub async fn delete_section(pool: &SqlitePool, code: &SectionCode) -> Result<()> {
    let code_str = code.0.to_string();

    sqlx::query!(
        r#"
        DELETE FROM sections
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
    use crate::storage::department;

    #[tokio::test]
    async fn test_create_and_get_section() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;

        // Create department first
        let dept = crate::models::Department::new('G', "総務");
        department::create_department(&pool, &dept).await?;

        let section = Section {
            code: SectionCode::new('I'),
            name: "インフラ".to_string(),
            department: DeptCode::new('G'),
        };
        create_section(&pool, &section).await?;

        let retrieved = get_section(&pool, &SectionCode::new('I')).await?;
        assert!(retrieved.is_some());
        if let Some(sec) = retrieved {
            assert_eq!(sec.name, "インフラ");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_list_sections_by_department() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;

        // Create department
        let dept = crate::models::Department::new('G', "総務");
        department::create_department(&pool, &dept).await?;

        // Create sections
        let section1 = Section {
            code: SectionCode::new('I'),
            name: "インフラ".to_string(),
            department: DeptCode::new('G'),
        };
        let section2 = Section {
            code: SectionCode::new('T'),
            name: "技術".to_string(),
            department: DeptCode::new('G'),
        };
        create_section(&pool, &section1).await?;
        create_section(&pool, &section2).await?;

        let list = list_sections_by_department(&pool, &DeptCode::new('G')).await?;
        assert_eq!(list.len(), 2);
        Ok(())
    }
}

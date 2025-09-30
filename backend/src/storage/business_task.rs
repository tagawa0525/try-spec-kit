//! Business Task storage operations

use sqlx::SqlitePool;
use crate::error::Result;
use crate::models::{BusinessTask, TaskId, DeptCode, SectionCode};

/// Create a new business task
pub async fn create_business_task(pool: &SqlitePool, task: &BusinessTask) -> Result<()> {
    let dept_code = task.department.map(|d| d.0.to_string());
    let section_code = task.section.map(|s| s.0.to_string());
    let active = task.active as i32;
    
    sqlx::query!(
        r#"
        INSERT INTO business_tasks (id, description, department_code, section_code, active)
        VALUES (?, ?, ?, ?, ?)
        "#,
        task.id.0,
        task.description,
        dept_code,
        section_code,
        active
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get a business task by ID
pub async fn get_business_task(pool: &SqlitePool, id: &TaskId) -> Result<Option<BusinessTask>> {
    let row = sqlx::query!(
        r#"
        SELECT id, description, department_code, section_code, active
        FROM business_tasks
        WHERE id = ?
        "#,
        id.0
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => {
            let department = r.department_code
                .and_then(|s| s.chars().next())
                .map(DeptCode::new);
            
            let section = r.section_code
                .and_then(|s| s.chars().next())
                .map(SectionCode::new);
            
            Ok(Some(BusinessTask {
                id: TaskId::new(r.id),
                description: r.description,
                department,
                section,
                active: r.active != 0,
            }))
        }
        None => Ok(None),
    }
}

/// List all business tasks
pub async fn list_business_tasks(pool: &SqlitePool) -> Result<Vec<BusinessTask>> {
    let rows = sqlx::query!(
        r#"
        SELECT id, description, department_code, section_code, active
        FROM business_tasks
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    let tasks = rows
        .into_iter()
        .map(|r| {
            let department = r.department_code
                .and_then(|s| s.chars().next())
                .map(DeptCode::new);
            
            let section = r.section_code
                .and_then(|s| s.chars().next())
                .map(SectionCode::new);
            
            BusinessTask {
                id: TaskId::new(r.id),
                description: r.description,
                department,
                section,
                active: r.active != 0,
            }
        })
        .collect();

    Ok(tasks)
}

/// List active business tasks
pub async fn list_active_business_tasks(pool: &SqlitePool) -> Result<Vec<BusinessTask>> {
    let rows = sqlx::query!(
        r#"
        SELECT id, description, department_code, section_code, active
        FROM business_tasks
        WHERE active = 1
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    let tasks = rows
        .into_iter()
        .map(|r| {
            let department = r.department_code
                .and_then(|s| s.chars().next())
                .map(DeptCode::new);
            
            let section = r.section_code
                .and_then(|s| s.chars().next())
                .map(SectionCode::new);
            
            BusinessTask {
                id: TaskId::new(r.id),
                description: r.description,
                department,
                section,
                active: r.active != 0,
            }
        })
        .collect();

    Ok(tasks)
}

/// List business tasks by department
pub async fn list_business_tasks_by_department(pool: &SqlitePool, dept_code: &DeptCode) -> Result<Vec<BusinessTask>> {
    let dept_code_str = dept_code.0.to_string();
    
    let rows = sqlx::query!(
        r#"
        SELECT id, description, department_code, section_code, active
        FROM business_tasks
        WHERE department_code = ?
        ORDER BY id
        "#,
        dept_code_str
    )
    .fetch_all(pool)
    .await?;

    let tasks = rows
        .into_iter()
        .map(|r| {
            let department = r.department_code
                .and_then(|s| s.chars().next())
                .map(DeptCode::new);
            
            let section = r.section_code
                .and_then(|s| s.chars().next())
                .map(SectionCode::new);
            
            BusinessTask {
                id: TaskId::new(r.id),
                description: r.description,
                department,
                section,
                active: r.active != 0,
            }
        })
        .collect();

    Ok(tasks)
}

/// Update a business task
pub async fn update_business_task(pool: &SqlitePool, task: &BusinessTask) -> Result<()> {
    let dept_code = task.department.map(|d| d.0.to_string());
    let section_code = task.section.map(|s| s.0.to_string());
    let active = task.active as i32;
    
    sqlx::query!(
        r#"
        UPDATE business_tasks
        SET description = ?, department_code = ?, section_code = ?, active = ?
        WHERE id = ?
        "#,
        task.description,
        dept_code,
        section_code,
        active,
        task.id.0
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a business task
pub async fn delete_business_task(pool: &SqlitePool, id: &TaskId) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM business_tasks
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
    use crate::storage::department;

    #[tokio::test]
    async fn test_create_and_get_business_task() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        
        let task = BusinessTask::new("task001", "契約書作成");
        create_business_task(&pool, &task).await?;
        
        let retrieved = get_business_task(&pool, &TaskId::new("task001")).await?;
        assert!(retrieved.is_some());
        if let Some(t) = retrieved {
            assert_eq!(t.description, "契約書作成");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_business_task_with_department() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        
        // Create department first
        let dept = crate::models::Department::new('G', "総務");
        department::create_department(&pool, &dept).await?;
        
        let task = BusinessTask::new("task001", "部門タスク")
            .with_department('G');
        create_business_task(&pool, &task).await?;
        
        let list = list_business_tasks_by_department(&pool, &DeptCode::new('G')).await?;
        assert_eq!(list.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_list_active_business_tasks() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        
        let task1 = BusinessTask::new("task001", "アクティブタスク");
        let task2 = BusinessTask::new("task002", "非アクティブタスク").inactive();
        create_business_task(&pool, &task1).await?;
        create_business_task(&pool, &task2).await?;
        
        let active_list = list_active_business_tasks(&pool).await?;
        assert_eq!(active_list.len(), 1);
        assert_eq!(active_list[0].id.0, "task001");
        Ok(())
    }
}

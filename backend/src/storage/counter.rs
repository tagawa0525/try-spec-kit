//! Counter management for auto-increment document numbers

use sqlx::SqlitePool;
use crate::error::Result;

/// Get the next counter value for a given scope key
/// This function increments the counter atomically and returns the new value
pub async fn get_next_counter(pool: &SqlitePool, scope_key: &str) -> Result<i64> {
    let mut tx = pool.begin().await?;
    
    // Try to get existing counter
    let existing = sqlx::query!(
        r#"
        SELECT current_value
        FROM counters
        WHERE scope_key = ?
        "#,
        scope_key
    )
    .fetch_optional(&mut *tx)
    .await?;

    let next_value = match existing {
        Some(row) => {
            // Increment existing counter
            let next = row.current_value + 1;
            sqlx::query!(
                r#"
                UPDATE counters
                SET current_value = ?, updated_at = CURRENT_TIMESTAMP
                WHERE scope_key = ?
                "#,
                next,
                scope_key
            )
            .execute(&mut *tx)
            .await?;
            next
        }
        None => {
            // Initialize new counter
            sqlx::query!(
                r#"
                INSERT INTO counters (scope_key, current_value)
                VALUES (?, 1)
                "#,
                scope_key
            )
            .execute(&mut *tx)
            .await?;
            1
        }
    };

    tx.commit().await?;
    Ok(next_value)
}

/// Get current counter value without incrementing (for preview/testing)
pub async fn get_current_counter(pool: &SqlitePool, scope_key: &str) -> Result<Option<i64>> {
    let row = sqlx::query!(
        r#"
        SELECT current_value
        FROM counters
        WHERE scope_key = ?
        "#,
        scope_key
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.current_value))
}

/// Reset a counter to a specific value (for administrative purposes)
pub async fn reset_counter(pool: &SqlitePool, scope_key: &str, value: i64) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO counters (scope_key, current_value)
        VALUES (?, ?)
        ON CONFLICT(scope_key) DO UPDATE SET current_value = ?, updated_at = CURRENT_TIMESTAMP
        "#,
        scope_key,
        value,
        value
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a counter (for cleanup)
pub async fn delete_counter(pool: &SqlitePool, scope_key: &str) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM counters
        WHERE scope_key = ?
        "#,
        scope_key
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// List all counters (for administrative/debugging purposes)
pub async fn list_counters(pool: &SqlitePool) -> Result<Vec<(String, i64)>> {
    let rows = sqlx::query!(
        r#"
        SELECT scope_key, current_value
        FROM counters
        ORDER BY scope_key
        "#
    )
    .fetch_all(pool)
    .await?;

    let counters = rows
        .into_iter()
        .map(|r| (r.scope_key, r.current_value))
        .collect();

    Ok(counters)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db::init_db_pool;

    #[tokio::test]
    async fn test_get_next_counter_initializes_at_1() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        
        let value = get_next_counter(&pool, "test_scope").await?;
        assert_eq!(value, 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_next_counter_increments() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        
        let val1 = get_next_counter(&pool, "test_scope").await?;
        let val2 = get_next_counter(&pool, "test_scope").await?;
        let val3 = get_next_counter(&pool, "test_scope").await?;
        
        assert_eq!(val1, 1);
        assert_eq!(val2, 2);
        assert_eq!(val3, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_current_counter() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        
        // No counter yet
        let current = get_current_counter(&pool, "test_scope").await?;
        assert!(current.is_none());
        
        // Create counter
        get_next_counter(&pool, "test_scope").await?;
        
        // Now it should exist
        let current = get_current_counter(&pool, "test_scope").await?;
        assert_eq!(current, Some(1));
        Ok(())
    }

    #[tokio::test]
    async fn test_reset_counter() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        
        get_next_counter(&pool, "test_scope").await?;
        get_next_counter(&pool, "test_scope").await?;
        
        // Reset to 10
        reset_counter(&pool, "test_scope", 10).await?;
        
        let current = get_current_counter(&pool, "test_scope").await?;
        assert_eq!(current, Some(10));
        
        let next = get_next_counter(&pool, "test_scope").await?;
        assert_eq!(next, 11);
        Ok(())
    }

    #[tokio::test]
    async fn test_different_scopes_independent() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        
        let val1a = get_next_counter(&pool, "scope_a").await?;
        let val1b = get_next_counter(&pool, "scope_b").await?;
        let val2a = get_next_counter(&pool, "scope_a").await?;
        
        assert_eq!(val1a, 1);
        assert_eq!(val1b, 1);
        assert_eq!(val2a, 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_list_counters() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        
        get_next_counter(&pool, "scope_a").await?;
        get_next_counter(&pool, "scope_b").await?;
        get_next_counter(&pool, "scope_b").await?;
        
        let counters = list_counters(&pool).await?;
        assert_eq!(counters.len(), 2);
        assert!(counters.contains(&("scope_a".to_string(), 1)));
        assert!(counters.contains(&("scope_b".to_string(), 2)));
        Ok(())
    }
}

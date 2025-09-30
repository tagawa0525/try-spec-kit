//! Document Type storage operations

use sqlx::SqlitePool;
use crate::error::Result;
use crate::models::{DocumentType, TypeCode, PathGenerationRule};

/// Create a new document type
pub async fn create_document_type(pool: &SqlitePool, doc_type: &DocumentType) -> Result<()> {
    // First, insert the generation rule and get its ID
    let rule_json = serde_json::to_string(&doc_type.generation_rule)?;
    let counter_scope = format!("{:?}", doc_type.generation_rule.counter_scope);
    let counter_digits = doc_type.generation_rule.counter_digits as i32;
    
    let rule_id = sqlx::query!(
        r#"
        INSERT INTO generation_rules (components, separators, counter_scope, counter_digits)
        VALUES (?, ?, ?, ?)
        RETURNING id
        "#,
        rule_json,
        "[]", // separators stored in the components JSON
        counter_scope,
        counter_digits
    )
    .fetch_one(pool)
    .await?
    .id;

    // Then insert the document type
    let active = doc_type.active as i32;
    
    sqlx::query!(
        r#"
        INSERT INTO document_types (code, description, root_directory, generation_rule_id, active)
        VALUES (?, ?, ?, ?, ?)
        "#,
        doc_type.code.0,
        doc_type.description,
        doc_type.root_directory,
        rule_id,
        active
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get a document type by code
pub async fn get_document_type(pool: &SqlitePool, code: &TypeCode) -> Result<Option<DocumentType>> {
    let row = sqlx::query!(
        r#"
        SELECT dt.code, dt.description, dt.root_directory, dt.active,
               gr.id as rule_id, gr.components as rule_components
        FROM document_types dt
        LEFT JOIN generation_rules gr ON dt.generation_rule_id = gr.id
        WHERE dt.code = ?
        "#,
        code.0
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => {
            let generation_rule = serde_json::from_str::<PathGenerationRule>(&r.rule_components)
                .unwrap_or_else(|_| PathGenerationRule::example_agi());
            
            Ok(Some(DocumentType {
                code: TypeCode::new(r.code),
                description: r.description,
                root_directory: r.root_directory,
                generation_rule,
                active: r.active != 0,
            }))
        }
        None => Ok(None),
    }
}

/// List all document types
pub async fn list_document_types(pool: &SqlitePool) -> Result<Vec<DocumentType>> {
    let rows = sqlx::query!(
        r#"
        SELECT dt.code, dt.description, dt.root_directory, dt.active,
               gr.id as rule_id, gr.components as rule_components
        FROM document_types dt
        LEFT JOIN generation_rules gr ON dt.generation_rule_id = gr.id
        ORDER BY dt.code
        "#
    )
    .fetch_all(pool)
    .await?;

    let doc_types = rows
        .into_iter()
        .filter_map(|r| {
            let generation_rule = r.rule_components
                .and_then(|json| serde_json::from_str::<PathGenerationRule>(&json).ok())
                .unwrap_or_else(PathGenerationRule::example_agi);
            
            Some(DocumentType {
                code: TypeCode::new(r.code),
                description: r.description,
                root_directory: r.root_directory,
                generation_rule,
                active: r.active != 0,
            })
        })
        .collect();

    Ok(doc_types)
}

/// List active document types
pub async fn list_active_document_types(pool: &SqlitePool) -> Result<Vec<DocumentType>> {
    let rows = sqlx::query!(
        r#"
        SELECT dt.code, dt.description, dt.root_directory, dt.active,
               gr.id as rule_id, gr.components as rule_components
        FROM document_types dt
        LEFT JOIN generation_rules gr ON dt.generation_rule_id = gr.id
        WHERE dt.active = 1
        ORDER BY dt.code
        "#
    )
    .fetch_all(pool)
    .await?;

    let doc_types = rows
        .into_iter()
        .filter_map(|r| {
            let generation_rule = r.rule_components
                .and_then(|json| serde_json::from_str::<PathGenerationRule>(&json).ok())
                .unwrap_or_else(PathGenerationRule::example_agi);
            
            Some(DocumentType {
                code: TypeCode::new(r.code),
                description: r.description,
                root_directory: r.root_directory,
                generation_rule,
                active: r.active != 0,
            })
        })
        .collect();

    Ok(doc_types)
}

/// Update a document type (note: generation rule is immutable)
pub async fn update_document_type(pool: &SqlitePool, doc_type: &DocumentType) -> Result<()> {
    let active = doc_type.active as i32;
    
    sqlx::query!(
        r#"
        UPDATE document_types
        SET description = ?, root_directory = ?, active = ?
        WHERE code = ?
        "#,
        doc_type.description,
        doc_type.root_directory,
        active,
        doc_type.code.0
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a document type
pub async fn delete_document_type(pool: &SqlitePool, code: &TypeCode) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM document_types
        WHERE code = ?
        "#,
        code.0
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
    async fn test_create_and_get_document_type() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        let rule = PathGenerationRule::example_agi();
        let doc_type = DocumentType::new("A", "契約書", "/docs/contracts/", rule);
        create_document_type(&pool, &doc_type).await.unwrap();
        
        let retrieved = get_document_type(&pool, &TypeCode::new("A")).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().description, "契約書");
    }

    #[tokio::test]
    async fn test_document_type_multibyte() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        let rule = PathGenerationRule::example_ringi();
        let doc_type = DocumentType::new("りん議", "稟議書", "/docs/ringi/", rule);
        create_document_type(&pool, &doc_type).await.unwrap();
        
        let retrieved = get_document_type(&pool, &TypeCode::new("りん議")).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().code.0, "りん議");
    }

    #[tokio::test]
    async fn test_list_active_document_types() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        let rule1 = PathGenerationRule::example_agi();
        let doc_type1 = DocumentType::new("A", "契約書", "/docs/", rule1);
        
        let rule2 = PathGenerationRule::example_ringi();
        let doc_type2 = DocumentType::new("B", "報告書", "/docs/", rule2).inactive();
        
        create_document_type(&pool, &doc_type1).await.unwrap();
        create_document_type(&pool, &doc_type2).await.unwrap();
        
        let active_list = list_active_document_types(&pool).await.unwrap();
        assert_eq!(active_list.len(), 1);
        assert_eq!(active_list[0].code.0, "A");
    }
}

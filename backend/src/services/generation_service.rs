//! Document number generation service

use chrono::{Datelike, Utc};
use sqlx::SqlitePool;
use crate::error::Result;
use crate::models::{PathGenerationRule, RuleComponent, CounterScope, TypeCode, DeptCode, SectionCode};
use crate::storage::counter;

/// Generate a document number based on the generation rule
pub async fn generate_document_number(
    pool: &SqlitePool,
    rule: &PathGenerationRule,
    type_code: &TypeCode,
    dept_code: &DeptCode,
    section_code: &SectionCode,
) -> Result<String> {
    let now = Utc::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    
    // Build scope key for counter
    let scope_key = build_scope_key(rule, type_code, dept_code, section_code, year, month as u8);
    
    // Get next counter value
    let counter_value = counter::get_next_counter(pool, &scope_key).await?;
    
    // Build document number
    let mut parts = Vec::new();
    
    for component in &rule.components {
        let part = match component {
            RuleComponent::TypeName => type_code.0.clone(),
            RuleComponent::DeptCode => dept_code.0.to_string(),
            RuleComponent::SectionCode => section_code.0.to_string(),
            RuleComponent::Year { digits } => {
                if *digits == 2 {
                    format!("{:02}", year % 100)
                } else {
                    format!("{:04}", year)
                }
            }
            RuleComponent::Month => format!("{:02}", month),
            RuleComponent::Day => format!("{:02}", day),
            RuleComponent::AutoIncrement => {
                format!("{:0width$}", counter_value, width = rule.counter_digits as usize)
            }
        };
        parts.push(part);
    }
    
    // Join parts with separators
    let document_number = if rule.separators.is_empty() {
        parts.join("")
    } else if rule.separators.len() == 1 {
        parts.join(&rule.separators[0])
    } else {
        // Multiple separators: interleave with parts
        let mut result = String::new();
        for (i, part) in parts.iter().enumerate() {
            result.push_str(part);
            if i < parts.len() - 1 {
                let sep_idx = i.min(rule.separators.len() - 1);
                result.push_str(&rule.separators[sep_idx]);
            }
        }
        result
    };
    
    Ok(document_number)
}

/// Build scope key for counter based on the counter scope
pub fn build_scope_key(
    rule: &PathGenerationRule,
    type_code: &TypeCode,
    dept_code: &DeptCode,
    section_code: &SectionCode,
    year: i32,
    month: u8,
) -> String {
    match rule.counter_scope {
        CounterScope::TypeOnly => {
            type_code.0.clone()
        }
        CounterScope::TypeAndYear => {
            format!("{}_{}", type_code.0, year)
        }
        CounterScope::TypeSectionYear => {
            format!("{}_{}_{}", type_code.0, section_code.0, year)
        }
        CounterScope::TypeDeptSectionYearMonth => {
            format!("{}_{}_{}_{:04}_{:02}", type_code.0, dept_code.0, section_code.0, year, month)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db::init_db_pool;

    #[tokio::test]
    async fn test_generate_document_number_agi() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        let rule = PathGenerationRule::example_agi();
        let type_code = TypeCode::new("A");
        let dept_code = DeptCode::new('G');
        let section_code = SectionCode::new('I');
        
        let number = generate_document_number(&pool, &rule, &type_code, &dept_code, &section_code)
            .await
            .unwrap();
        
        // Should be like "AGI2510001" (AGI + YY + MM + counter)
        assert!(number.starts_with("AGI"));
        assert_eq!(number.len(), 9); // AGI(3) + YY(2) + MM(2) + NNN(3) = 10 but may vary
    }

    #[tokio::test]
    async fn test_generate_document_number_ringi() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        let rule = PathGenerationRule::example_ringi();
        let type_code = TypeCode::new("りん議");
        let dept_code = DeptCode::new('G');
        let section_code = SectionCode::new('I');
        
        let number = generate_document_number(&pool, &rule, &type_code, &dept_code, &section_code)
            .await
            .unwrap();
        
        // Should be like "りん議I-25001" (りん議 + I + - + YY + counter)
        assert!(number.starts_with("りん議"));
        assert!(number.contains("I-"));
    }

    #[test]
    fn test_build_scope_key_type_only() {
        let rule = PathGenerationRule::new(
            vec![RuleComponent::TypeName, RuleComponent::AutoIncrement],
            CounterScope::TypeOnly,
            3,
        );
        let type_code = TypeCode::new("A");
        let dept_code = DeptCode::new('G');
        let section_code = SectionCode::new('I');
        
        let key = build_scope_key(&rule, &type_code, &dept_code, &section_code, 2025, 10);
        assert_eq!(key, "A");
    }

    #[test]
    fn test_build_scope_key_type_and_year() {
        let rule = PathGenerationRule::new(
            vec![RuleComponent::TypeName, RuleComponent::Year { digits: 2 }, RuleComponent::AutoIncrement],
            CounterScope::TypeAndYear,
            3,
        );
        let type_code = TypeCode::new("A");
        let dept_code = DeptCode::new('G');
        let section_code = SectionCode::new('I');
        
        let key = build_scope_key(&rule, &type_code, &dept_code, &section_code, 2025, 10);
        assert_eq!(key, "A_2025");
    }

    #[test]
    fn test_build_scope_key_full() {
        let rule = PathGenerationRule::example_agi();
        let type_code = TypeCode::new("A");
        let dept_code = DeptCode::new('G');
        let section_code = SectionCode::new('I');
        
        let key = build_scope_key(&rule, &type_code, &dept_code, &section_code, 2025, 9);
        assert_eq!(key, "A_G_I_2025_09");
    }

    #[tokio::test]
    async fn test_counter_increment() {
        let pool = init_db_pool("sqlite::memory:").await.unwrap();
        
        let rule = PathGenerationRule::example_agi();
        let type_code = TypeCode::new("A");
        let dept_code = DeptCode::new('G');
        let section_code = SectionCode::new('I');
        
        let num1 = generate_document_number(&pool, &rule, &type_code, &dept_code, &section_code)
            .await
            .unwrap();
        let num2 = generate_document_number(&pool, &rule, &type_code, &dept_code, &section_code)
            .await
            .unwrap();
        
        // Counter should increment
        assert_ne!(num1, num2);
        // Last 3 digits should be 001, 002
        assert!(num1.ends_with("001"));
        assert!(num2.ends_with("002"));
    }
}

//! Path Generation Rule entity

use serde::{Deserialize, Serialize};

/// Rule components for document number generation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RuleComponent {
    /// Document type name (e.g., "A", "りん議")
    TypeName,
    /// Department code (e.g., "G")
    DeptCode,
    /// Section code (e.g., "I")
    SectionCode,
    /// Year with specified digits (2 or 4)
    Year { digits: u8 },
    /// Month (2 digits, 01-12)
    Month,
    /// Day (2 digits, 01-31)
    Day,
    /// Auto-increment counter
    AutoIncrement,
}

/// Counter scope for auto-increment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CounterScope {
    /// Counter per type only (e.g., "A" → 001, 002, 003...)
    TypeOnly,
    /// Counter per type and year (e.g., "A_2025" → 001, "A_2026" → 001)
    TypeAndYear,
    /// Counter per type, section, and year (e.g., "A_I_2025" → 001)
    TypeSectionYear,
    /// Counter per type, dept, section, year, month (e.g., "A_G_I_2025_09" → 001)
    TypeDeptSectionYearMonth,
}

/// Path generation rule
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathGenerationRule {
    /// Rule ID (optional, auto-assigned by database)
    pub id: Option<i64>,
    /// Ordered list of rule components
    pub components: Vec<RuleComponent>,
    /// Separator characters to insert between components
    /// Empty vec = no separators, single value = same separator everywhere,
    /// multiple values = different separators between each component
    pub separators: Vec<String>,
    /// Counter scope for auto-increment
    pub counter_scope: CounterScope,
    /// Number of digits for counter (e.g., 3 → 001, 002, ...)
    pub counter_digits: u8,
}

impl PathGenerationRule {
    /// Create a new generation rule
    pub fn new(
        components: Vec<RuleComponent>,
        counter_scope: CounterScope,
        counter_digits: u8,
    ) -> Self {
        Self {
            id: None,
            components,
            separators: Vec::new(),
            counter_scope,
            counter_digits,
        }
    }

    /// Add separators to the rule
    pub fn with_separators(mut self, separators: Vec<String>) -> Self {
        self.separators = separators;
        self
    }

    /// Example: AGI[YYMM][NNN]
    pub fn example_agi() -> Self {
        Self::new(
            vec![
                RuleComponent::TypeName,
                RuleComponent::DeptCode,
                RuleComponent::SectionCode,
                RuleComponent::Year { digits: 2 },
                RuleComponent::Month,
                RuleComponent::AutoIncrement,
            ],
            CounterScope::TypeDeptSectionYearMonth,
            3,
        )
    }

    /// Example: りん議I-[YY][NNN]
    pub fn example_ringi() -> Self {
        Self::new(
            vec![
                RuleComponent::TypeName,
                RuleComponent::SectionCode,
                RuleComponent::Year { digits: 2 },
                RuleComponent::AutoIncrement,
            ],
            CounterScope::TypeSectionYear,
            3,
        )
        .with_separators(vec!["-".to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_component_serialization() -> Result<(), serde_json::Error> {
        let component = RuleComponent::Year { digits: 2 };
        let json = serde_json::to_string(&component)?;
        assert!(json.contains("Year"));
        assert!(json.contains("\"digits\":2"));
        Ok(())
    }

    #[test]
    fn test_path_generation_rule_example_agi() {
        let rule = PathGenerationRule::example_agi();
        assert_eq!(rule.components.len(), 6);
        assert_eq!(rule.counter_digits, 3);
        assert!(matches!(rule.counter_scope, CounterScope::TypeDeptSectionYearMonth));
    }

    #[test]
    fn test_path_generation_rule_example_ringi() {
        let rule = PathGenerationRule::example_ringi();
        assert_eq!(rule.components.len(), 4);
        assert_eq!(rule.separators.len(), 1);
        assert_eq!(rule.separators[0], "-");
    }

    #[test]
    fn test_counter_scope_serialization() -> Result<(), serde_json::Error> {
        let scope = CounterScope::TypeAndYear;
        let json = serde_json::to_string(&scope)?;
        assert_eq!(json, "\"TypeAndYear\"");
        Ok(())
    }
}

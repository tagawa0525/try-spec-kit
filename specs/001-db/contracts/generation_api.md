# Generation API Contract

**Purpose**: 文書番号自動生成エンジン  
**Source Requirements**: FR-003, FR-006, FR-007, FR-008

## Core Generation Function

```rust
/// 文書番号を生成
///
/// # Arguments
/// * `rule` - パス生成ルール
/// * `context` - 生成コンテキスト（部門、課、ユーザー情報）
///
/// # Returns
/// 生成された文書番号（例: "AGI-2509001", "りん議I-25009"）
///
/// # Errors
/// - `Error::CounterOverflow` - カウンター上限到達
/// - `Error::InvalidRuleComponent` - ルール構成要素不正
///
/// # Performance
/// < 10ms
pub fn generate_document_number(
    rule: &PathGenerationRule,
    context: &GenerationContext,
) -> Result<String, Error>;

pub struct GenerationContext {
    pub document_type: TypeCode,
    pub department: DeptCode,
    pub section: SectionCode,
    pub user: UserId,
    pub current_date: DateTime<Utc>,
}
```

## Counter Management

```rust
/// カウンターを取得・増分
///
/// # Arguments
/// * `scope_key` - カウンタースコープキー（種類+年+月等）
///
/// # Returns
/// 次の連番
///
/// # Errors
/// - `Error::CounterOverflow` - 999等の上限到達
pub fn get_next_counter(scope_key: &str) -> Result<u32, Error>;

/// カウンタースコープキーを生成
///
/// # Arguments
/// * `scope` - スコープ定義
/// * `context` - 生成コンテキスト
///
/// # Returns
/// スコープキー（例: "A:G:I:2025:09", "りん議:I:2025"）
pub fn build_scope_key(
    scope: &CounterScope,
    context: &GenerationContext,
) -> String;
```

## Integration Tests

```rust
#[test]
fn test_generate_with_all_components() -> anyhow::Result<()> {
    // AGI-2509001形式
    let rule = create_rule_agi();
    let context = create_context_gi_sept_2025();
    
    let number = generate_document_number(&rule, &context)?;
    
    assert_eq!(number, "AGI-2509001");
    Ok(())
}

#[test]
fn test_generate_with_separator() -> anyhow::Result<()> {
    // りん議I-25009形式
    let rule = create_rule_ringi();
    let context = create_context_i_2025();
    
    let number = generate_document_number(&rule, &context)?;
    
    assert!(number.starts_with("りん議I-25"));
    assert_eq!(number.split('-').count(), 2);
    Ok(())
}

#[test]
fn test_counter_increment() -> anyhow::Result<()> {
    let scope = "A:G:I:2025:09";
    
    let n1 = get_next_counter(scope)?;
    let n2 = get_next_counter(scope)?;
    
    assert_eq!(n1 + 1, n2);
    Ok(())
}

#[test]
fn test_counter_reset_per_scope() -> anyhow::Result<()> {
    // 異なるスコープで独立したカウンター
    let scope_a = "A:G:I:2025:09";
    let scope_b = "A:G:I:2025:10"; // 月が異なる
    
    get_next_counter(scope_a)?; // 001
    let n_b = get_next_counter(scope_b)?; // 別スコープなので001
    
    assert_eq!(n_b, 1);
    Ok(())
}
```

---

**Contract Status**: 定義完了

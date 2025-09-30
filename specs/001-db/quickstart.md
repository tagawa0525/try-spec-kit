# Quickstart Validation

**Feature**: Document Path Management Database  
**Purpose**: End-to-end validation scenario  
**Date**: 2025-09-30

## Prerequisites

```bash
# Rust 1.75+
rustc --version

# Clone repository
cd /home/tagawa/try-spec-kit-via-vs_code

# Install dependencies
cargo build
```

## Quickstart Scenario

このシナリオは、spec.mdの主要な受け入れシナリオを実行します。

### Step 1: Initialize Database

```rust
use document_path_db::*;

fn main() -> Result<(), Error> {
    // データベース初期化
    let db = DocumentStore::new("./data/documents.db")?;
    
    println!("✓ Database initialized");
    Ok(())
}
```

**Expected**: データベースファイル作成、エラーなし

---

### Step 2: Setup Organization Structure

```rust
// 部門・課を作成
let dept_g = Department {
    code: DeptCode('G'),
    name: "総務".to_string(),
    sections: vec![SectionCode('I')],
};

let section_i = Section {
    code: SectionCode('I'),
    name: "インフラ".to_string(),
    department: DeptCode('G'),
};

db.create_department(&dept_g)?;
db.create_section(&section_i)?;

println!("✓ Organization structure created: G (総務) → I (インフラ)");
```

**Expected**: 部門G、課I作成成功

---

### Step 3: Create User

```rust
let user = User {
    id: UserId("user001".to_string()),
    name: "田川太郎".to_string(),
    department: DeptCode('G'),
    section: SectionCode('I'),
    permissions: Permissions::default(),
};

db.create_user(&user)?;

println!("✓ User created: {}", user.name);
```

**Expected**: ユーザー作成成功、GI部門・課に所属

---

### Step 4: Define Document Type with Rule

```rust
// 文書種類A: AGI[YYMM][NNN]形式
let rule_a = PathGenerationRule {
    id: None,
    components: vec![
        RuleComponent::TypeName,
        RuleComponent::DeptCode,
        RuleComponent::SectionCode,
        RuleComponent::Year { digits: 2 },
        RuleComponent::Month,
        RuleComponent::AutoIncrement,
    ],
    separators: HashMap::new(),
    counter_scope: CounterScope::TypeDeptSectionYearMonth,
    counter_digits: 3,
    example_output: "AGI-2509001".to_string(),
};

let type_a = DocumentType {
    code: TypeCode("A".to_string()),
    description: "契約上必要な提出文書".to_string(),
    department: Some(DeptCode('G')),
    section: Some(SectionCode('I')),
    root_directory: PathBuf::from("/docs/contracts/"),
    generation_rule: rule_a,
    active: true,
};

db.create_document_type(&type_a)?;

println!("✓ Document type A created with rule: AGI[YYMM][NNN]");
```

**Expected**: 文書種類A作成、ルール定義済み

---

### Step 5: Auto-generate Document Number (Scenario 1)

```rust
// 2025年9月に文書を作成
let doc1 = db.create_document_auto(
    &TypeCode("A".to_string()),
    &UserId("user001".to_string()),
    None,
)?;

println!("✓ Document created:");
println!("  Number: {}", doc1.document_number);
println!("  Path: {}", doc1.file_path.display());

assert_eq!(doc1.document_number, "AGI-2509001");
assert_eq!(doc1.file_path, PathBuf::from("/docs/contracts/AGI-2509001"));
assert!(doc1.generated);
```

**Expected**: 
- 文書番号: `AGI-2509001`
- パス: `/docs/contracts/AGI-2509001`
- generated: `true`

**Validation**: Scenario 1 from spec.md ✓

---

### Step 6: Multi-byte Document Type (Scenario 2)

```rust
// 文書種類「りん議」: りん議I-[YY][NNN]形式
let rule_ringi = PathGenerationRule {
    components: vec![
        RuleComponent::TypeName,
        RuleComponent::SectionCode,
        RuleComponent::Year { digits: 2 },
        RuleComponent::AutoIncrement,
    ],
    separators: {
        let mut m = HashMap::new();
        m.insert(1, "-".to_string());
        m
    },
    counter_scope: CounterScope::TypeSectionYear,
    counter_digits: 3,
    example_output: "りん議I-25009".to_string(),
};

let type_ringi = DocumentType {
    code: TypeCode("りん議".to_string()),
    description: "稟議書".to_string(),
    department: None,
    section: Some(SectionCode('I')),
    root_directory: PathBuf::from("/docs/ringi/"),
    generation_rule: rule_ringi,
    active: true,
};

db.create_document_type(&type_ringi)?;

let doc_ringi = db.create_document_auto(
    &TypeCode("りん議".to_string()),
    &UserId("user001".to_string()),
    None,
)?;

println!("✓ Multi-byte document created:");
println!("  Number: {}", doc_ringi.document_number);

assert!(doc_ringi.document_number.starts_with("りん議I-25"));
assert_eq!(doc_ringi.document_number.chars().count(), 11); // りん議I-25009
```

**Expected**:
- 文書番号: `りん議I-25009` (または次の連番)
- マルチバイト文字対応

**Validation**: Scenario 2 from spec.md ✓

---

### Step 7: Query by Type

```rust
let type_a_docs = db.get_documents_by_type(&TypeCode("A".to_string()))?;

println!("✓ Query by type A: {} documents found", type_a_docs.len());

assert!(!type_a_docs.is_empty());
assert!(type_a_docs.iter().all(|d| d.document_type.0 == "A"));
```

**Expected**: 文書種類Aでフィルタリング成功

**Validation**: Scenario 6, FR-012 ✓

---

### Step 8: Logical Deletion

```rust
let deleted = db.delete_document(&doc1.id)?;

println!("✓ Document logically deleted: {}", deleted.document_number);

assert!(deleted.deleted);

// 削除後も取得可能
let retrieved = db.get_document_by_id(&doc1.id)?;
assert!(retrieved.deleted);

println!("✓ Deleted document still retrievable (logical deletion)");
```

**Expected**: 
- deleted フラグ: `true`
- データ保持: IDで取得可能

**Validation**: Scenario 9, FR-014, FR-038 ✓

---

### Step 9: Concurrent Read

```rust
use std::thread;

let handles: Vec<_> = (0..10)
    .map(|i| {
        let db_clone = db.clone();
        thread::spawn(move || {
            db_clone.get_all_documents().unwrap();
            println!("  Thread {} read complete", i);
        })
    })
    .collect();

for h in handles {
    h.join().unwrap();
}

println!("✓ 10 concurrent reads completed successfully");
```

**Expected**: 10スレッド同時読み取り成功、エラーなし

**Validation**: FR-018 ✓

---

### Step 10: Performance Check

```rust
use std::time::Instant;

// 文書番号生成パフォーマンス
let start = Instant::now();
for _ in 0..100 {
    db.create_document_auto(
        &TypeCode("A".to_string()),
        &UserId("user001".to_string()),
        None,
    )?;
}
let duration = start.elapsed();

let avg_ms = duration.as_millis() / 100;
println!("✓ Average generation time: {}ms", avg_ms);

assert!(avg_ms < 10, "Generation took {}ms (expected <10ms)", avg_ms);
```

**Expected**: 平均文書生成時間 < 10ms

**Validation**: Performance Goals ✓

---

## Validation Summary

| Scenario | Status | Validation |
|----------|--------|------------|
| 1. AGI-2509001 auto-generation | ✓ | Scenario 1, FR-006 |
| 2. Multi-byte りん議I-25009 | ✓ | Scenario 2, FR-033 |
| 3. User dept/section check | ✓ | Scenario 3, FR-032 |
| 6. Query by number | ✓ | Scenario 6, FR-010 |
| 9. Logical deletion | ✓ | Scenario 9, FR-014, FR-038 |
| Concurrent reads | ✓ | FR-018 |
| Performance < 10ms | ✓ | Performance Goals |

## Running Quickstart

```bash
# テスト実行
cargo test --test quickstart

# 統合テスト
cargo test --test integration

# すべてのテスト
cargo test
```

**Expected Output**:
```
running 10 tests
test quickstart::step_1_init ... ok
test quickstart::step_2_organization ... ok
test quickstart::step_3_user ... ok
test quickstart::step_4_document_type ... ok
test quickstart::step_5_auto_generate ... ok
test quickstart::step_6_multi_byte ... ok
test quickstart::step_7_query ... ok
test quickstart::step_8_logical_delete ... ok
test quickstart::step_9_concurrent_read ... ok
test quickstart::step_10_performance ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured
```

---

**Status**: Quickstart定義完了、実装後に実行

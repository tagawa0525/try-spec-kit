# Document Path API Contract

**Purpose**: 文書パスのCRUD操作  
**Source Requirements**: FR-001, FR-009~FR-014, FR-017

## Core Types

```rust
use std::path::PathBuf;
use chrono::{DateTime, Utc};

pub struct DocumentPath {
    pub id: DocumentId,
    pub document_number: String,
    pub document_type: TypeCode,
    pub department: DeptCode,
    pub section: SectionCode,
    pub business_task: Option<TaskId>,
    pub user: UserId,
    pub file_path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub generated: bool,
    pub deleted: bool,
}

pub struct DocumentId(String);
pub struct TypeCode(String);
pub struct DeptCode(char);
pub struct SectionCode(char);
pub struct TaskId(String);
pub struct UserId(String);
```

## API Functions

### Create Document (Auto-generated)

```rust
/// 文書パスを自動生成して作成
/// 
/// # Arguments
/// * `type_code` - 文書種類コード
/// * `user_id` - 作成ユーザー
/// * `task_id` - 関連業務タスク（オプショナル）
///
/// # Returns
/// 生成された文書番号とパス
///
/// # Errors
/// - `Error::InvalidTypeCode` - 存在しない文書種類
/// - `Error::UserNotFound` - 存在しないユーザー
/// - `Error::UnauthorizedDocumentType` - ユーザーが文書種類使用不可
/// - `Error::Storage` - 永続化エラー
///
/// # Performance
/// < 10ms (文書番号生成 + DB書き込み)
pub fn create_document_auto(
    type_code: &TypeCode,
    user_id: &UserId,
    task_id: Option<&TaskId>,
) -> Result<DocumentPath, Error>;
```

**Contract Test**:
```rust
#[test]
fn test_create_document_auto_signature() {
    // コンパイル時型チェック
    let _: fn(&TypeCode, &UserId, Option<&TaskId>) -> Result<DocumentPath, Error> 
        = create_document_auto;
}
```

---

### Create Document (Manual)

```rust
/// 文書パスを手動で追加
///
/// # Arguments
/// * `document_number` - 手動入力の文書番号
/// * `type_code` - 文書種類
/// * `file_path` - ファイルパス（絶対パスのみ）
/// * `user_id` - 作成ユーザー
/// * `task_id` - 関連業務タスク（オプショナル）
///
/// # Returns
/// 作成された文書パス
///
/// # Errors
/// - `Error::RelativePathNotAllowed` - 相対パス指定
/// - `Error::DuplicateDocumentNumber` - 重複文書番号
/// - `Error::InvalidTypeCode` - 存在しない文書種類
///
/// # Performance
/// < 10ms
pub fn create_document_manual(
    document_number: String,
    type_code: &TypeCode,
    file_path: PathBuf,
    user_id: &UserId,
    task_id: Option<&TaskId>,
) -> Result<DocumentPath, Error>;
```

**Validation**:
- `file_path.is_absolute()` must be `true`
- `document_number` must be unique
- `type_code` must exist
- `user_id` must be authorized for `type_code`

---

### Get Document by ID

```rust
/// 文書パスをIDで取得
///
/// # Arguments
/// * `id` - 文書ID
///
/// # Returns
/// 文書パス（削除済みも含む）
///
/// # Errors
/// - `Error::DocumentNotFound` - 存在しない文書ID
///
/// # Performance
/// < 10ms (単一キー検索)
pub fn get_document_by_id(id: &DocumentId) -> Result<DocumentPath, Error>;
```

---

### Get Document by Number

```rust
/// 文書パスを文書番号で取得
///
/// # Arguments
/// * `document_number` - 文書番号（例: "AGI-2509001", "りん議I-25009"）
///
/// # Returns
/// 文書パス
///
/// # Errors
/// - `Error::DocumentNotFound` - 存在しない文書番号
///
/// # Performance
/// < 10ms (インデックス検索)
pub fn get_document_by_number(document_number: &str) -> Result<DocumentPath, Error>;
```

---

### Update Document Path

```rust
/// 文書パスを更新（パス値のみ、文書番号は変更不可）
///
/// # Arguments
/// * `id` - 文書ID
/// * `new_path` - 新しいファイルパス（絶対パスのみ）
///
/// # Returns
/// 更新後の文書パス
///
/// # Errors
/// - `Error::DocumentNotFound` - 存在しない文書ID
/// - `Error::RelativePathNotAllowed` - 相対パス指定
/// - `Error::ConcurrentModification` - 同時更新競合
///
/// # Performance
/// < 20ms (排他的書き込み)
pub fn update_document_path(
    id: &DocumentId,
    new_path: PathBuf,
) -> Result<DocumentPath, Error>;
```

**Concurrency**: 排他的書き込みトランザクション（FR-019）

---

### Delete Document (Logical)

```rust
/// 文書パスを論理削除（データは保持）
///
/// # Arguments
/// * `id` - 文書ID
///
/// # Returns
/// 削除された文書パス（deleted=true）
///
/// # Errors
/// - `Error::DocumentNotFound` - 存在しない文書ID
/// - `Error::ConcurrentModification` - 同時更新競合
///
/// # Performance
/// < 20ms (排他的書き込み)
pub fn delete_document(id: &DocumentId) -> Result<DocumentPath, Error>;
```

**Note**: 物理削除なし（FR-038, FR-039）、deleted フラグを true に設定

---

### Search Documents

```rust
/// 文書パスを検索
///
/// # Arguments
/// * `query` - 検索クエリ（文書番号、パス、種類等）
///
/// # Returns
/// マッチした文書パスのリスト
///
/// # Errors
/// - `Error::Storage` - 検索エラー
///
/// # Performance
/// < 100ms (10,000件中)
pub fn search_documents(query: &str) -> Result<Vec<DocumentPath>, Error>;
```

---

## Integration Test Scenarios

### Scenario 1: Auto-generation (FR-006)
```rust
#[test]
fn test_auto_generate_document_type_a() -> anyhow::Result<()> {
    // Given: GI部門・課に所属するユーザー、文書種類A
    let user = create_test_user("G", "I");
    let type_a = create_test_type_a(); // AGI[YYMM][NNN]形式
    
    // When: 2025年9月に文書を作成
    let doc = create_document_auto(&type_a.code, &user.id, None)?;
    
    // Then: 文書番号は "AGI-2509001"
    assert_eq!(doc.document_number, "AGI-2509001");
    assert!(doc.file_path.to_string_lossy().starts_with("/docs/contracts/"));
    Ok(())
}
```

### Scenario 2: Multi-byte type code (FR-033)
```rust
#[test]
fn test_multi_byte_type_code() -> anyhow::Result<()> {
    // Given: 文書種類「りん議」、I課ユーザー
    let user = create_test_user("K", "I");
    let type_ringi = create_test_type_ringi(); // りん議I-[YY][NNN]形式
    
    // When: 2025年に文書を作成
    let doc = create_document_auto(&type_ringi.code, &user.id, None)?;
    
    // Then: 文書番号は "りん議I-25009" 形式
    assert!(doc.document_number.starts_with("りん議I-25"));
    assert_eq!(doc.document_number.len(), "りん議I-25009".len());
    Ok(())
}
```

### Scenario 3: Manual path addition (FR-009)
```rust
#[test]
fn test_manual_document_addition() -> anyhow::Result<()> {
    // Given: 手動文書番号とパス
    let user = create_test_user("G", "I");
    let type_d = create_test_type_d();
    
    // When: 手動で文書を追加
    let doc = create_document_manual(
        "CUSTOM-001".to_string(),
        &type_d.code,
        PathBuf::from("/external/docs/import.pdf"),
        &user.id,
        None,
    )?;
    
    // Then: generated=false
    assert!(!doc.generated);
    assert_eq!(doc.document_number, "CUSTOM-001");
    Ok(())
}
```

### Scenario 4: Logical deletion (FR-014, FR-038)
```rust
#[test]
fn test_logical_deletion() -> anyhow::Result<()> {
    // Given: 作成済み文書
    let doc = create_test_document();
    
    // When: 文書を削除
    let deleted = delete_document(&doc.id)?;
    
    // Then: deleted=true、データは保持
    assert!(deleted.deleted);
    
    // And: IDで取得可能
    let retrieved = get_document_by_id(&doc.id)?;
    assert!(retrieved.deleted);
    Ok(())
}
```

---

## Performance Validation

```rust
#[test]
fn test_performance_create_under_10ms() -> anyhow::Result<()> {
    let start = Instant::now();
    let _doc = create_document_auto(&test_type(), &test_user(), None)?;
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 10, "Creation took {}ms", duration.as_millis());
    Ok(())
}

#[test]
fn test_performance_query_under_100ms() -> anyhow::Result<()> {
    // Setup: 10,000 documents
    setup_10k_documents();
    
    let start = Instant::now();
    let _results = search_documents("AGI")?;
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 100, "Query took {}ms", duration.as_millis());
    Ok(())
}
```

---

**Contract Status**: 定義完了、テスト実装待ち

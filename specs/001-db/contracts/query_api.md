# Query API Contract

**Purpose**: 文書パスのクエリ・検索機能  
**Source Requirements**: FR-011, FR-012, FR-029~FR-031

## Query Functions

```rust
/// すべての文書パスを取得（削除済みを除く）
///
/// # Returns
/// アクティブな文書パスのリスト
///
/// # Performance
/// < 100ms (10,000件)
pub fn get_all_documents() -> Result<Vec<DocumentPath>, Error>;

/// 文書種類でフィルタ
pub fn get_documents_by_type(type_code: &TypeCode) -> Result<Vec<DocumentPath>, Error>;

/// 部門でフィルタ
pub fn get_documents_by_department(dept_code: &DeptCode) -> Result<Vec<DocumentPath>, Error>;

/// 課でフィルタ
pub fn get_documents_by_section(section_code: &SectionCode) -> Result<Vec<DocumentPath>, Error>;

/// 業務タスクでフィルタ
pub fn get_documents_by_task(task_id: &TaskId) -> Result<Vec<DocumentPath>, Error>;

/// 削除済み文書を含めて取得
pub fn get_all_documents_including_deleted() -> Result<Vec<DocumentPath>, Error>;
```

## Advanced Query Builder

```rust
pub struct DocumentQuery {
    type_code: Option<TypeCode>,
    department: Option<DeptCode>,
    section: Option<SectionCode>,
    task: Option<TaskId>,
    include_deleted: bool,
    created_after: Option<DateTime<Utc>>,
    created_before: Option<DateTime<Utc>>,
}

impl DocumentQuery {
    pub fn new() -> Self;
    pub fn with_type(mut self, code: TypeCode) -> Self;
    pub fn with_department(mut self, dept: DeptCode) -> Self;
    pub fn with_section(mut self, section: SectionCode) -> Self;
    pub fn with_task(mut self, task: TaskId) -> Self;
    pub fn include_deleted(mut self) -> Self;
    pub fn created_after(mut self, date: DateTime<Utc>) -> Self;
    pub fn execute(self) -> Result<Vec<DocumentPath>, Error>;
}
```

## Integration Tests

```rust
#[test]
fn test_query_all_documents() -> anyhow::Result<()> {
    setup_test_documents(100);
    
    let docs = get_all_documents()?;
    
    assert_eq!(docs.len(), 100);
    assert!(docs.iter().all(|d| !d.deleted));
    Ok(())
}

#[test]
fn test_query_by_type() -> anyhow::Result<()> {
    setup_mixed_types();
    
    let type_a = TypeCode("A".into());
    let docs = get_documents_by_type(&type_a)?;
    
    assert!(docs.iter().all(|d| d.document_type == type_a));
    Ok(())
}

#[test]
fn test_query_by_department() -> anyhow::Result<()> {
    setup_mixed_departments();
    
    let dept_g = DeptCode('G');
    let docs = get_documents_by_department(&dept_g)?;
    
    assert!(docs.iter().all(|d| d.department == dept_g));
    Ok(())
}

#[test]
fn test_query_builder_complex() -> anyhow::Result<()> {
    setup_test_data();
    
    let docs = DocumentQuery::new()
        .with_department(DeptCode('G'))
        .with_type(TypeCode("A".into()))
        .created_after(Utc.ymd(2025, 9, 1).and_hms(0, 0, 0))
        .execute()?;
    
    assert!(docs.iter().all(|d| 
        d.department == DeptCode('G') &&
        d.document_type == TypeCode("A".into()) &&
        d.created_at >= Utc.ymd(2025, 9, 1).and_hms(0, 0, 0)
    ));
    Ok(())
}

#[test]
fn test_performance_query_10k_documents() -> anyhow::Result<()> {
    setup_10k_documents();
    
    let start = Instant::now();
    let _docs = get_documents_by_type(&TypeCode("A".into()))?;
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 100);
    Ok(())
}
```

---

**Contract Status**: 定義完了

# API Contracts

**Feature**: Document Path Management Database  
**Date**: 2025-09-30  
**Purpose**: Define public API contracts for library interface

## Contract Overview

このディレクトリには、文書パス管理データベースの公開APIコントラクトを定義します。Rustライブラリとして使用する際のインターフェース仕様です。

## Contract Files

| File | Purpose | Source Requirement |
|------|---------|-------------------|
| [document_path_api.md](./document_path_api.md) | 文書パスCRUD操作 | FR-001, FR-009~FR-014, FR-017 |
| [document_type_api.md](./document_type_api.md) | 文書種類管理 | FR-002, FR-025 |
| [generation_api.md](./generation_api.md) | 文書番号自動生成 | FR-003, FR-006, FR-007, FR-008 |
| [query_api.md](./query_api.md) | クエリ・検索機能 | FR-011, FR-012, FR-029~FR-031 |
| [organization_api.md](./organization_api.md) | 組織構造管理 | FR-005, FR-026, FR-027, FR-028 |

## Contract Testing Strategy

各契約に対して以下のテストを作成：

1. **Contract Tests** (`tests/contract/`):
   - 関数シグネチャ検証
   - 型定義検証
   - エラー型検証
   - これらのテストは実装なしで失敗する

2. **Integration Tests** (`tests/integration/`):
   - 受け入れシナリオ（spec.mdから）
   - エンドツーエンドフロー
   - 実装後に成功する

## API Design Principles

1. **Result-based Error Handling**: すべての操作は`Result<T, Error>`を返す
2. **Newtype Pattern**: 型安全性のためnewtypeを使用（`DocumentId`, `TypeCode`等）
3. **Builder Pattern**: 複雑なエンティティ構築にビルダーを使用
4. **Immutability**: 可能な限り不変データ構造
5. **UTF-8 Native**: Rust `String`でマルチバイト文字ネイティブサポート

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Document not found: {0}")]
    DocumentNotFound(String),
    
    #[error("Invalid path: must be absolute path")]
    RelativePathNotAllowed,
    
    #[error("Invalid type code: {0}")]
    InvalidTypeCode(String),
    
    #[error("Department not found: {0}")]
    DepartmentNotFound(char),
    
    #[error("Section not found: {0}")]
    SectionNotFound(char),
    
    #[error("User not authorized for document type")]
    UnauthorizedDocumentType,
    
    #[error("Concurrent modification detected")]
    ConcurrentModification,
    
    #[error("Storage error: {0}")]
    Storage(#[from] sled::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
```

## Performance Contracts

契約には以下のパフォーマンス保証を含む：

- **文書番号生成**: < 10ms
- **単一文書取得**: < 10ms
- **クエリ操作**: < 100ms (10,000件中)
- **並行読み取り**: 10+ 同時ユーザー

---

**Next**: 個別契約ファイルの詳細定義

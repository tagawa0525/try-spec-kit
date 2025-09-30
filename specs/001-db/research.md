# Phase 0: Research & Technical Decisions

**Feature**: Document Path Management Database  
**Date**: 2025-09-30  
**Status**: Complete

## Research Questions

### 1. 永続化層の選択

**Question**: 約10,000件の文書パスデータを永続化するための最適なストレージ技術は？

**Options Evaluated**:

| Option | Pros | Cons | Fit Score |
|--------|------|------|-----------|
| **SQLite3 + Sqlx** | 非同期対応、型安全（コンパイル時検証）、axumと統合容易 | マイグレーション管理必要 | ⭐⭐⭐⭐⭐ |
| **SQLite3 (rusqlite)** | 成熟、広く使用、SQLクエリ柔軟性、書き込み低頻度に最適 | 同期API、axumのasyncと組み合わせにくい | ⭐⭐⭐⭐ |
| **sled** (embedded DB) | Rustネイティブ、ACID保証、並行性サポート、型安全 | 比較的新しい、書き込み頻度高い用途向け | ⭐⭐⭐ |
| **PostgreSQL** (外部DB) | 強力、スケーラブル | 重量級、10k件には過剰 | ⭐⭐ |

**Decision**: **SQLite3 + Sqlx** (非同期SQLiteクライアント)

**Rationale**:
- **非同期対応**: axum (Tokio) との統合が自然、async/awaitネイティブ
- **型安全性**: コンパイル時SQLクエリ検証（`sqlx::query!`マクロ）、憲法原則VIに準拠
- **書き込み低頻度最適**: 文書パス作成は低頻度、読み取り・検索が主用途
- **WALモード**: 並行読み取り性能が高い（書き込み中でも読み取り可能）
- **マイグレーション**: sqlx-cliでスキーマバージョン管理
- **クロスプラットフォーム**: Unix/Linux/Windows全対応
- **論理削除対応**: deleted列でのフラグ管理が自然

**Implementation Approach**:
```rust
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub struct DocumentStore {
    pool: SqlitePool,
}

impl DocumentStore {
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        // WALモード有効化（並行読み取り改善）
        sqlx::query("PRAGMA journal_mode=WAL")
            .execute(&pool)
            .await?;
        
        Ok(Self { pool })
    }
    
    // 読み取り（並行可能、型安全）
    pub async fn get_document(&self, id: &str) -> Result<Option<DocumentPath>, Error> {
        let doc = sqlx::query_as!(
            DocumentPath,
            r#"
            SELECT id, document_number, document_type, department, section,
                   business_task, user_id, file_path, created_at, updated_at,
                   generated as "generated: bool", deleted as "deleted: bool"
            FROM documents
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(doc)
    }
    
    // 書き込み（排他的、トランザクション）
    pub async fn create_document(&self, doc: &DocumentPath) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;
        
        sqlx::query!(
            r#"
            INSERT INTO documents (id, document_number, document_type, ...)
            VALUES (?, ?, ?, ...)
            "#,
            doc.id, doc.document_number, doc.document_type,
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        Ok(())
    }
}
```

**Schema Migration**:
```bash
# sqlx-cliでマイグレーション管理
sqlx migrate add create_documents_table
sqlx migrate run
```

**Alternatives Rejected**:
- rusqlite: 同期API、async/awaitとの統合が複雑
- sled: 書き込み頻度が高い用途向け、本要件では過剰
- PostgreSQL: 規模に対して過剰、展開複雑性増加

---

### 2. 文書番号生成ルールのモデリング

**Question**: 柔軟な文書番号生成ルール（各文書種類が独自フォーマット）をどう実装するか？

**Options Evaluated**:

| Approach | Flexibility | Complexity | Type Safety |
|----------|-------------|------------|-------------|
| **Enum-based DSL** | 高 | 中 | ⭐⭐⭐⭐⭐ |
| **String template** | 中 | 低 | ⭐⭐ |
| **Custom parser** | 最高 | 高 | ⭐⭐⭐ |

**Decision**: **Enum-based DSL**（ドメイン固有言語）

**Rationale**:
- 型安全性が最も高い（Rustコンパイラがルール検証）
- 仕様の例（AGI-2509001、りん議I-25009）を表現可能
- 拡張性とメンテナンス性のバランスが良い

**Design**:
```rust
pub enum RuleComponent {
    TypeName,                    // 文書種類名（"A", "りん議"等）
    DeptCode,                    // 部門コード（1文字）
    SectionCode,                 // 課コード（1文字）
    Year { digits: u8 },         // 年（2桁または4桁）
    Month,                       // 月（2桁）
    AutoIncrement { digits: u8 }, // 連番（桁数指定）
    Separator(String),           // セパレーター（"-", ""等）
}

pub struct PathGenerationRule {
    components: Vec<RuleComponent>,
    counter_scope: CounterScope, // 何でリセットするか
}

pub enum CounterScope {
    TypeOnly,
    TypeAndYear,
    TypeSectionYear,
    TypeDeptSectionYearMonth,
}
```

**Example Encoding**:
```rust
// AGI-2509001 = A + G + I + 2509 + 001
vec![
    RuleComponent::TypeName,
    RuleComponent::DeptCode,
    RuleComponent::SectionCode,
    RuleComponent::Year { digits: 2 },
    RuleComponent::Month,
    RuleComponent::AutoIncrement { digits: 3 },
]

// りん議I-25009 = りん議 + I + "-" + 25 + 009
vec![
    RuleComponent::TypeName,
    RuleComponent::SectionCode,
    RuleComponent::Separator("-".into()),
    RuleComponent::Year { digits: 2 },
    RuleComponent::AutoIncrement { digits: 3 },
]
```

---

### 3. マルチバイト文字（UTF-8）対応

**Question**: 日本語文書種類コード（りん議、教育）をどうハンドリングするか？

**Decision**: **Rustの`String`型を使用**（UTF-8ネイティブ）

**Rationale**:
- Rustの`String`はUTF-8保証（憲法原則VIに準拠）
- `str`の文字境界チェックが自動（パニック防止）
- ファイルパス生成時にOS固有のエンコーディング変換が必要な場合は`std::path::Path`が処理

**Validation Strategy**:
```rust
// 文書種類コードの検証
fn validate_type_code(code: &str) -> Result<(), Error> {
    if code.is_empty() || code.len() > 12 { // バイト長ではなく文字数
        return Err(Error::InvalidTypeCode);
    }
    // ファイルパス禁止文字チェック
    if code.contains(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..]) {
        return Err(Error::InvalidPathCharacter);
    }
    Ok(())
}
```

---

### 4. Windows UNCパス対応

**Question**: `\\server\share\path`形式のパスをどう扱うか？

**Decision**: **`std::path::PathBuf`を使用**、プラットフォーム固有処理なし

**Rationale**:
- `PathBuf`はWindows UNCパスをネイティブサポート
- クロスプラットフォーム互換性維持（Unix/Linuxでは絶対パス）
- 検証は`is_absolute()`メソッドで統一

**Validation**:
```rust
use std::path::Path;

fn validate_absolute_path(path_str: &str) -> Result<(), Error> {
    let path = Path::new(path_str);
    if !path.is_absolute() {
        return Err(Error::RelativePathNotAllowed);
    }
    // Windows UNC: \\server\share → is_absolute() = true
    // Unix: /home/user → is_absolute() = true
    Ok(())
}
```

---

### 5. 並行性モデル

**Question**: 複数ユーザー読み取り + 排他的書き込みをどう実装するか？

**Decision**: **SQLite3 WALモード + Sqlx非同期トランザクション**

**Rationale**:
- SQLite3はWALモードで並行読み取りをネイティブサポート
- 書き込み中でも読み取り可能（読み取り頻度が高い用途に最適）
- Sqlxの非同期トランザクションで排他的書き込み制御
- axum (Tokio) との統合が自然

**API Design**:
```rust
use sqlx::{SqlitePool, Transaction, Sqlite};

impl DocumentStore {
    // 読み取り（並行可能、型安全）
    pub async fn get_document(&self, id: &str) -> Result<Option<DocumentPath>, Error> {
        // Sqlxの読み取りは自動的に並行性対応（WALモード）
        // コンパイル時型チェック
        sqlx::query_as!(DocumentPath, "SELECT * FROM documents WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(Into::into)
    }
    
    // 書き込み（排他的、トランザクション）
    pub async fn create_document(&self, doc: DocumentPath) -> Result<(), Error> {
        let mut tx: Transaction<Sqlite> = self.pool.begin().await?;
        
        sqlx::query!(
            r#"
            INSERT INTO documents (id, document_number, ...)
            VALUES (?, ?, ...)
            "#,
            doc.id, doc.document_number,
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        Ok(())
    }
}
```

---

## Technology Stack Summary

| Layer | Technology | Version | Justification |
|-------|-----------|---------|---------------|
| **Language (Backend)** | Rust | 1.75+ | 憲法原則VI、型安全性、並行性 |
| **Language (Frontend)** | TypeScript | 5.0+ | 型安全性、Rust APIとの整合性 |
| **Web Framework** | axum | 0.7+ | 型安全、async/await、tower統合 |
| **Frontend Framework** | Svelte | 4.0+ | 軽量、コンパイル時最適化、少ないボイラープレート |
| **Build Tool (Frontend)** | Vite | 5.0+ | 高速ビルド、HMR |
| **Storage** | SQLite3 (sqlx) | 0.7+ | 非同期、型安全、書き込み低頻度最適 |
| **Serialization** | serde, serde_json | 1.0+ | Rust標準、型安全 |
| **Date/Time** | chrono | 0.4+ | タイムスタンプ管理、RFC3339対応 |
| **Async Runtime** | tokio | 1.35+ | axum必須、並行性基盤 |
| **HTTP Client (Frontend)** | fetch API | built-in | ブラウザ標準 |
| **Testing (Backend)** | cargo test | built-in | 憲法原則II（TDD必須） |
| **Testing (Frontend)** | vitest | 1.0+ | Vite統合、高速 |
| **Linting** | cargo clippy, eslint | built-in | 憲法原則VI（イディオム準拠） |
| **Formatting** | cargo fmt, prettier | built-in | 憲法原則VI（一貫性） |

**External Dependency Justification** (憲法原則VIに基づく):
- `sqlx`: 非同期SQLiteクライアント、コンパイル時型チェック、axum統合必須
- `serde`/`serde_json`: 事実上のRust標準シリアライゼーション、型安全性保証
- `chrono`: std::time::SystemTimeより高レベルAPI、RFC3339フォーマット対応
- `axum`: Web API必須、std代替なし
- `tokio`: async runtime必須、axum依存
- Svelte/TypeScript: フロントエンド要件（FR-023）、軽量で型安全なUI構築

---

## Best Practices Research

### Rust Document Management Patterns

**研究対象**: Rustでのファイルパス管理、データベース設計パターン

**Key Findings**:
1. **Newtype Pattern**: 型安全性向上のため、`DocumentId(String)`、`DeptCode(char)`等のnewtypeを使用
2. **Builder Pattern**: 複雑なエンティティ（DocumentPath、PathGenerationRule）の構築にはビルダーパターン推奨
3. **Error Handling**: `thiserror`クレートでカスタムエラー型定義、Result<T, E>で伝播
4. **Validation**: デー タ構築時に検証、無効状態を型システムで防止（Parse, don't validate原則）

**Application to This Feature**:
```rust
// Newtype examples
pub struct DocumentId(String);
pub struct DeptCode(char);
pub struct SectionCode(char);

// Builder pattern for PathGenerationRule
pub struct PathGenerationRuleBuilder {
    components: Vec<RuleComponent>,
    counter_scope: Option<CounterScope>,
}

impl PathGenerationRuleBuilder {
    pub fn add_component(mut self, comp: RuleComponent) -> Self {
        self.components.push(comp);
        self
    }
    
    pub fn build(self) -> Result<PathGenerationRule, Error> {
        // Validation
        if self.components.is_empty() {
            return Err(Error::EmptyRule);
        }
        Ok(PathGenerationRule { ... })
    }
}
```

---

### 6. Webフレームワーク選択（バックエンドAPI）

**Question**: Rust Web APIサーバーにどのフレームワークを使用するか？

**Options Evaluated**:

| Option | Pros | Cons | Fit Score |
|--------|------|------|-----------|
| **axum** | 型安全、tower統合、async/await、モダン | 比較的新しい | ⭐⭐⭐⭐⭐ |
| **actix-web** | 成熟、高パフォーマンス、広く使用 | Actor modelやや複雑 | ⭐⭐⭐⭐ |
| **rocket** | 開発者体験良い、マクロ豊富 | async対応遅れ | ⭐⭐⭐ |
| **warp** | 関数型スタイル、軽量 | ドキュメント少ない | ⭐⭐⭐ |

**Decision**: **axum** (Tokioベースのwebフレームワーク)

**Rationale**:
- **型安全性**: Extractor patternで型安全なリクエスト処理（憲法原則VIに準拠）
- **tower統合**: ミドルウェア、ロギング、タイムアウト等が標準的
- **async/await**: Tokio runtimeでネイティブ対応、並行性要件に最適
- **モダン**: Rust 2024 editionのイディオムに沿う
- **学習曲線**: actix-webよりシンプル、概念的に直感的

**Implementation Approach**:
```rust
use axum::{
    Router,
    routing::{get, post, put, delete},
    Json, extract::Path,
};

async fn main() {
    let app = Router::new()
        .route("/api/documents", post(create_document))
        .route("/api/documents/:id", get(get_document))
        .route("/api/documents/:id", put(update_document))
        .route("/api/documents/:id", delete(delete_document_logical))
        .route("/api/documents/search", get(search_documents));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

**Alternatives Rejected**:
- actix-web: Actor model複雑性が要件に不要
- rocket: async対応の成熟度がaxumに劣る
- warp: コミュニティ・ドキュメントがaxumに劣る

---

### 7. フロントエンド技術スタック

**Question**: Web UIにどのフロントエンド技術を使用するか？

**Options Evaluated**:

| Option | Pros | Cons | Fit Score |
|--------|------|------|-----------|
| **Svelte + TypeScript** | 軽量、コンパイル時最適化、少ないボイラープレート、学習容易 | エコシステムやや小 | ⭐⭐⭐⭐⭐ |
| **React + TypeScript** | 広く使用、型安全、エコシステム大 | やや冗長、ランタイム大 | ⭐⭐⭐⭐ |
| **Vue 3 + TypeScript** | 学習容易、composition API | エコシステムやや小 | ⭐⭐⭐⭐ |
| **Leptos** (Rust WASM) | Rustで統一 | 成熟度低い、学習曲線急 | ⭐⭐ |

**Decision**: **Svelte + TypeScript + Vite**

**Rationale**:
- **軽量・高速**: コンパイル時に最適化、ランタイムオーバーヘッド最小
- **TypeScript対応**: 型安全性（Rust APIとの整合性、バグ早期発見）
- **少ないボイラープレート**: Reactより簡潔、開発速度向上
- **リアクティビティ**: シンプルな構文（`$:` reactive statements）
- **Vite統合**: 高速ビルド、HMR（開発体験向上）
- **中規模アプリに最適**: 10,000件規模の文書管理に適切

**UI Requirements**:
- 文書一覧表示（ページネーション、フィルタリング）
- 文書作成フォーム（文書種類選択、自動番号生成プレビュー）
- 検索機能（文書番号、種類、部門、課、タスクで検索）
- 論理削除操作（削除フラグ変更）

**API Communication**:
```typescript
// TypeScript API client例（Svelte store統合）
import { writable } from 'svelte/store';

interface DocumentPath {
  id: string;
  document_number: string;
  document_type: string;
  department: string;
  section: string;
  file_path: string;
  created_at: string;
  deleted: boolean;
}

export const documents = writable<DocumentPath[]>([]);

export async function createDocument(
  typeCode: string,
  userId: string,
  taskId?: string
): Promise<DocumentPath> {
  const response = await fetch('/api/documents', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ type_code: typeCode, user_id: userId, task_id: taskId }),
  });
  const doc = await response.json();
  documents.update(docs => [...docs, doc]);
  return doc;
}
```

**Component Structure**:
```
src/
├── routes/
│   ├── +page.svelte           # ダッシュボード
│   ├── documents/
│   │   ├── +page.svelte       # 文書一覧
│   │   ├── [id]/+page.svelte  # 文書詳細
│   │   └── create/+page.svelte # 文書作成
│   └── search/+page.svelte    # 検索
├── lib/
│   ├── components/
│   │   ├── DocumentList.svelte
│   │   ├── DocumentForm.svelte
│   │   └── SearchBar.svelte
│   └── api/
│       └── documents.ts       # API client
└── app.html
```

**Alternatives Rejected**:
- React: ランタイムオーバーヘッド、ボイラープレート多い
- Vue 3: Svelteのコンパイル時最適化に劣る
- Leptos (Rust WASM): 成熟度不足、チーム学習コスト高

---

## Open Questions Resolved

| Question | Resolution |
|----------|-----------|
| 永続化層選択 | ✅ SQLite3 + Sqlx（非同期、型安全、書き込み低頻度に最適） |
| 文書番号生成ルール | ✅ Enum-based DSL（型安全、柔軟性） |
| マルチバイト対応 | ✅ Rust String（UTF-8ネイティブ） |
| Windows UNCパス | ✅ std::path::PathBuf（ネイティブサポート） |
| 並行性制御 | ✅ SQLite3 WALモード + Sqlx非同期トランザクション |
| Webフレームワーク | ✅ axum（型安全、async、tower統合） |
| フロントエンド | ✅ Svelte + TypeScript + Vite（軽量、コンパイル時最適化） |

**Status**: すべての技術的不明点が解決済み。Phase 1（設計）に進む準備完了。

---

**Next Phase**: Phase 1 - Design & Contracts

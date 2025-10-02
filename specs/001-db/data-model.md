# Data Model: Document Path Management

Path: /home/tagawa/try-spec-kit-via-vs_code/specs/001-db/data-model.md

Date: 2025-10-02

Entities

- Department
  - code: string (1 char)
  - name: string
  - sections: list of Section.code

- Section
  - code: string (1 char)
  - name: string
  - department_code: string (FK)

- User
  - id: string
  - name: string
  - department_code: string
  - section_code: string

- BusinessTask
  - id: string
  - name: string
  - department_code: string (optional)
  - section_code: string (optional)
  - active: boolean

- PathGenerationRule
  - id: integer
  - components: JSON (ordered list)
  - separators: JSON
  - counter_scope: string
  - counter_digits: integer

- DocumentType
  - code: string
  - description: string
  - root_directory: string
  - generation_rule_id: integer (FK)
  - active: boolean

- DocumentPath
  - id: uuid
  - document_number: string
  - document_type_code: string (FK)
  - department_code: string
  - section_code: string
  - business_task_id: string (nullable)
  - user_id: string
  - file_path: string
  - generated: boolean
  - deleted: boolean
  - created_at: timestamp
  - updated_at: timestamp

Relationships
- Department 1..* Section
- DocumentType -> PathGenerationRule
- DocumentPath -> DocumentType, User, BusinessTask

Validation rules
- file_path MUST be an absolute path (Unix, Windows local, or Windows UNC)
- document_number MUST conform to the DocumentType generation rule if generated=true
- generated=false entries may provide arbitrary document_number but MUST be unique
# Data Model

**Feature**: Document Path Management Database  
**Date**: 2025-09-30  
**Source**: Extracted from [spec.md](./spec.md) Key Entities section

## Entity Definitions

### Department（部門）

組織内の部門を表現。

**Fields**:
```rust
pub struct Department {
    /// 部門コード（1文字、例: "G" = 総務, "K" = 分析）
    pub code: DeptCode,
    /// 部門名
    pub name: String,
    /// この部門に属する課のリスト
    pub sections: Vec<SectionCode>,
}

/// Newtype for type safety
pub struct DeptCode(char);
```

**Validation Rules**:
- `code`: 必須、1文字、A-Z（大文字）
- `name`: 必須、非空文字列
- `sections`: 空リスト可（新設部門の場合）

**Relationships**:
- Has many: `Section` (1部門 → 複数課)
- Referenced by: `User`, `DocumentType`

---

### Section（課）

部門内の課を表現。

**Fields**:
```rust
pub struct Section {
    /// 課コード（1文字、例: "I" = インフラ, "T" = 技術）
    pub code: SectionCode,
    /// 課名
    pub name: String,
    /// 親部門への参照
    pub department: DeptCode,
}

/// Newtype for type safety
pub struct SectionCode(char);
```

**Validation Rules**:
- `code`: 必須、1文字、A-Z（大文字）
- `name`: 必須、非空文字列
- `department`: 必須、存在する部門コードを参照

**Relationships**:
- Belongs to: `Department` (多課 → 1部門)
- Has many: `User` (1課 → 複数ユーザー)
- Referenced by: `DocumentType`

---

### User（ユーザー）

システム利用者を表現。

**Fields**:
```rust
pub struct User {
    /// ユーザー固有識別子
    pub id: UserId,
    /// ユーザー名
    pub name: String,
    /// 所属部門
    pub department: DeptCode,
    /// 所属課
    pub section: SectionCode,
    /// アクセス権限（将来的にSSO/LDAP/ADから取得）
    pub permissions: Permissions,
}

pub struct UserId(String);

pub struct Permissions {
    pub can_create: bool,
    pub can_update: bool,
    pub can_delete: bool,
    pub can_read: bool,
}
```

**Validation Rules**:
- `id`: 必須、一意、非空文字列
- `name`: 必須、非空文字列
- `department`: 必須、存在する部門コードを参照
- `section`: 必須、存在する課コードを参照、かつ`department`に属する課であること
- `permissions`: デフォルト値あり（読み取り専用）

**Relationships**:
- Belongs to: `Department` (多ユーザー → 1部門)
- Belongs to: `Section` (多ユーザー → 1課)
- Has many: `DocumentPath` (1ユーザー → 複数文書作成)

---

### BusinessTask（業務タスク）

文書が支援する業務活動を表現。

**Fields**:
```rust
pub struct BusinessTask {
    /// タスク固有識別子
    pub id: TaskId,
    /// タスク名/説明
    pub description: String,
    /// 関連部門（オプショナル、部門横断タスクの場合はNone）
    pub department: Option<DeptCode>,
    /// 関連課（オプショナル、課横断タスクの場合はNone）
    pub section: Option<SectionCode>,
    /// アクティブ/非アクティブ状態
    pub active: bool,
}

pub struct TaskId(String);
```

**Validation Rules**:
- `id`: 必須、一意、非空文字列
- `description`: 必須、非空文字列
- `department`: オプショナル、指定する場合は存在する部門コード
- `section`: オプショナル、指定する場合は存在する課コードかつ`department`に属すること
- `active`: デフォルトtrue

**Relationships**:
- May belong to: `Department` (多タスク → 0..1部門)
- May belong to: `Section` (多タスク → 0..1課)
- Referenced by: `DocumentPath` (多文書 → 多タスク、N:N関係)

---

### DocumentType（文書種類）

文書カテゴリと番号生成ルールを定義。

**Fields**:
```rust
pub struct DocumentType {
    /// 文書種類コード（1-3文字、例: "A", "りん議", "教育"）
    pub code: TypeCode,
    /// 文書種類説明
    pub description: String,
    /// 関連部門（オプショナル、複数部門で使用可能な種類の場合はNone）
    pub department: Option<DeptCode>,
    /// 関連課（オプショナル、複数課で使用可能な種類の場合はNone）
    pub section: Option<SectionCode>,
    /// ルートディレクトリパス（絶対パスまたはWindows UNCパス）
    pub root_directory: PathBuf,
    /// パス生成ルール参照
    pub generation_rule: PathGenerationRule,
    /// アクティブ/非アクティブ状態
    pub active: bool,
}

pub struct TypeCode(String);
```

**Validation Rules**:
- `code`: 必須、1-12文字（マルチバイト考慮）、ファイルパス禁止文字なし
- `description`: 必須、非空文字列
- `department`: オプショナル、指定する場合は存在する部門コード
- `section`: オプショナル、指定する場合は存在する課コードかつ`department`に属すること
- `root_directory`: 必須、絶対パス、`PathBuf::is_absolute() == true`
- `generation_rule`: 必須、有効なルール定義
- `active`: デフォルトtrue

**Relationships**:
- May belong to: `Department` (多種類 → 0..1部門)
- May belong to: `Section` (多種類 → 0..1課)
- Has one: `PathGenerationRule` (1種類 → 1ルール、埋め込み)
- Referenced by: `DocumentPath`

---

### PathGenerationRule（パス生成ルール）

文書番号の自動構築ルールを定義（文書種類ごとに完全カスタマイズ可能）。

**Fields**:
```rust
pub struct PathGenerationRule {
    /// ルール識別子（DocumentTypeに埋め込みのためオプショナル）
    pub id: Option<RuleId>,
    /// ルール構成要素（順序付きリスト）
    pub components: Vec<RuleComponent>,
    /// セパレーター配置（components間の位置 → セパレーター文字列）
    pub separators: HashMap<usize, String>,
    /// 自動増分カウンタースコープ
    pub counter_scope: CounterScope,
    /// 自動増分桁数
    pub counter_digits: u8,
    /// 例示出力（ドキュメント目的）
    pub example_output: String,
}

pub struct RuleId(String);

pub enum RuleComponent {
    /// 文書種類名（例: "A", "りん議"）
    TypeName,
    /// 部門コード（1文字）
    DeptCode,
    /// 課コード（1文字）
    SectionCode,
    /// 年（2桁または4桁）
    Year { digits: u8 },
    /// 月（2桁）
    Month,
    /// 自動増分番号（桁数はcounter_digitsで指定）
    AutoIncrement,
}

pub enum CounterScope {
    /// 文書種類のみでカウンター識別
    TypeOnly,
    /// 文書種類 + 年でカウンター識別
    TypeAndYear,
    /// 文書種類 + 課 + 年でカウンター識別
    TypeSectionYear,
    /// 文書種類 + 部門 + 課 + 年 + 月でカウンター識別
    TypeDeptSectionYearMonth,
}
```

**Validation Rules**:
- `components`: 必須、非空リスト
- `components`: `AutoIncrement`を必ず1つ含む
- `separators`: キーはcomponents長未満のインデックス
- `counter_scope`: 必須
- `counter_digits`: 1-9の範囲
- `example_output`: 必須、非空文字列

**Example Encoding**:
```rust
// Example 1: AGI-2509001
PathGenerationRule {
    components: vec![
        RuleComponent::TypeName,          // "A"
        RuleComponent::DeptCode,          // "G"
        RuleComponent::SectionCode,       // "I"
        RuleComponent::Year { digits: 2 }, // "25"
        RuleComponent::Month,             // "09"
        RuleComponent::AutoIncrement,     // "001"
    ],
    separators: HashMap::new(), // セパレーターなし
    counter_scope: CounterScope::TypeDeptSectionYearMonth,
    counter_digits: 3,
    example_output: "AGI-2509001".to_string(),
}

// Example 2: りん議I-25009
PathGenerationRule {
    components: vec![
        RuleComponent::TypeName,          // "りん議"
        RuleComponent::SectionCode,       // "I"
        RuleComponent::Year { digits: 2 }, // "25"
        RuleComponent::AutoIncrement,     // "009"
    ],
    separators: {
        let mut m = HashMap::new();
        m.insert(1, "-".to_string()); // SectionCodeの後
        m
    },
    counter_scope: CounterScope::TypeSectionYear,
    counter_digits: 3,
    example_output: "りん議I-25009".to_string(),
}
```

**Relationships**:
- Embedded in: `DocumentType` (1ルール ← 1種類)

---

### DocumentPath（文書パス）

保存されたファイルパスとメタデータを表現。

**Fields**:
```rust
pub struct DocumentPath {
    /// システム生成固有識別子
    pub id: DocumentId,
    /// 文書番号（生成または手動入力）
    pub document_number: String,
    /// 文書種類参照
    pub document_type: TypeCode,
    /// 部門コード（文書番号またはユーザーコンテキストから抽出）
    pub department: DeptCode,
    /// 課コード（文書番号またはユーザーコンテキストから抽出）
    pub section: SectionCode,
    /// 業務タスク参照
    pub business_task: Option<TaskId>,
    /// 作成/所有ユーザー参照
    pub user: UserId,
    /// ファイルパス（絶対パス）
    pub file_path: PathBuf,
    /// 作成タイムスタンプ
    pub created_at: DateTime<Utc>,
    /// 最終更新タイムスタンプ
    pub updated_at: DateTime<Utc>,
    /// 生成フラグ（true=自動生成、false=手動入力）
    pub generated: bool,
    /// 削除フラグ（論理削除、監査目的で無期限保持）
    pub deleted: bool,
}

pub struct DocumentId(String);
```

**Validation Rules**:
- `id`: 必須、一意、システム生成（UUID推奨）
- `document_number`: 必須、非空文字列、形式は`generation_rule`に準拠（生成時）
- `document_type`: 必須、存在する文書種類コード
- `department`: 必須、存在する部門コード
- `section`: 必須、存在する課コード、かつ`department`に属すること
- `business_task`: オプショナル、指定する場合は存在するタスクID
- `user`: 必須、存在するユーザーID
- `file_path`: 必須、絶対パス、`PathBuf::is_absolute() == true`
- `created_at`: 自動設定（システム時刻）
- `updated_at`: 自動更新（更新時にシステム時刻）
- `generated`: デフォルトtrue（手動追加時はfalse）
- `deleted`: デフォルトfalse

**State Transitions**:
```
[新規作成] → Active (deleted=false)
Active → Deleted (deleted=true) ※物理削除なし、データ保持
Deleted → Active (deleted=false) ※復元可能
```

**Relationships**:
- Belongs to: `DocumentType` (多文書 → 1種類)
- Belongs to: `Department` (多文書 → 1部門)
- Belongs to: `Section` (多文書 → 1課)
- Belongs to: `User` (多文書 → 1ユーザー)
- May belong to: `BusinessTask` (多文書 → 0..1タスク)

---

## Entity Relationship Diagram

```
Department (1) ──< (N) Section
    │                     │
    │                     │
    └──< (N) User         │
    │         │           │
    │         └──< (N) DocumentPath
    │                     │
    └──< (0..1) DocumentType
    │                     │
    └──< (0..1) BusinessTask
                          │
                          └──< (0..N) DocumentPath

DocumentType (1) ──owns── (1) PathGenerationRule
DocumentType (1) ──< (N) DocumentPath
```

**Key Relationships**:
- **Department → Section**: 1部門は複数の課を持つ
- **Department/Section → User**: 各ユーザーは1部門1課に所属
- **User → DocumentPath**: 各ユーザーは複数文書を作成
- **DocumentType → PathGenerationRule**: 各文書種類は1つの生成ルールを持つ（埋め込み）
- **DocumentType → DocumentPath**: 各文書は1つの種類に属する
- **BusinessTask → DocumentPath**: 各文書は0個または1個のタスクに紐づく

---

## Indexes & Queries

### Primary Indexes
- `DocumentPath::id` (unique) - 主キー
- `User::id` (unique) - ユーザー検索
- `DocumentType::code` (unique) - 文書種類検索
- `Department::code` (unique) - 部門検索
- `Section::code` (unique within department) - 課検索
- `BusinessTask::id` (unique) - タスク検索

### Secondary Indexes
- `DocumentPath::document_number` (unique) - 文書番号検索（FR-010）
- `DocumentPath::document_type` - 種類別クエリ（FR-012）
- `DocumentPath::department` - 部門別クエリ（FR-029）
- `DocumentPath::section` - 課別クエリ（FR-030）
- `DocumentPath::business_task` - タスク別クエリ（FR-031）
- `DocumentPath::deleted` - 削除状態フィルタリング

### SQLite3 Implementation Strategy
```sql
-- Primary Tables
CREATE TABLE documents (
    id TEXT PRIMARY KEY,
    document_number TEXT UNIQUE NOT NULL,
    document_type TEXT NOT NULL,
    department TEXT NOT NULL,
    section TEXT NOT NULL,
    business_task TEXT,
    user_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    created_at TEXT NOT NULL,  -- RFC3339形式
    updated_at TEXT NOT NULL,  -- RFC3339形式
    generated INTEGER NOT NULL,  -- 0=false, 1=true
    deleted INTEGER NOT NULL DEFAULT 0  -- 0=false, 1=true
);

CREATE TABLE users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    department TEXT NOT NULL,
    section TEXT NOT NULL,
    can_create INTEGER NOT NULL,
    can_update INTEGER NOT NULL,
    can_delete INTEGER NOT NULL,
    can_read INTEGER NOT NULL
);

CREATE TABLE document_types (
    code TEXT PRIMARY KEY,
    description TEXT NOT NULL,
    department TEXT,
    section TEXT,
    root_directory TEXT NOT NULL,
    generation_rule TEXT NOT NULL,  -- JSON serialized PathGenerationRule
    active INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE departments (
    code TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE sections (
    code TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    department TEXT NOT NULL,
    FOREIGN KEY (department) REFERENCES departments(code)
);

CREATE TABLE business_tasks (
    id TEXT PRIMARY KEY,
    description TEXT NOT NULL,
    department TEXT,
    section TEXT,
    active INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE counters (
    scope_key TEXT PRIMARY KEY,
    value INTEGER NOT NULL DEFAULT 0
);

-- Indexes
CREATE INDEX idx_document_number ON documents(document_number);
CREATE INDEX idx_document_type ON documents(document_type);
CREATE INDEX idx_department ON documents(department);
CREATE INDEX idx_section ON documents(section);
CREATE INDEX idx_business_task ON documents(business_task);
CREATE INDEX idx_deleted ON documents(deleted);
CREATE INDEX idx_user_dept_section ON users(department, section);

-- WAL mode for concurrent reads (書き込み低頻度、読み取り高頻度に最適)
PRAGMA journal_mode=WAL;
PRAGMA synchronous=NORMAL;
```

---

## Validation Summary

| Entity | Critical Validations |
|--------|---------------------|
| Department | コード1文字、A-Z |
| Section | コード1文字、A-Z、部門存在確認 |
| User | 部門・課の所属整合性 |
| BusinessTask | 部門・課の所属整合性（オプショナル） |
| DocumentType | ルートディレクトリ絶対パス、ルール有効性 |
| PathGenerationRule | AutoIncrement必須、スコープ整合性 |
| DocumentPath | 絶対パス、部門・課・種類・ユーザー存在確認 |

---

**Status**: データモデル定義完了。次はcontract定義へ。

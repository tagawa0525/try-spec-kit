
# Implementation Plan: Document Path Management Database

**Branch**: `001-db` | **Date**: 2025-09-30 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/home/tagawa/try-spec-kit-via-vs_code/specs/001-db/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from file system structure or context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, `GEMINI.md` for Gemini CLI, `QWEN.md` for Qwen Code or `AGENTS.md` for opencode).
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
文書パス管理データベースシステム：部署・課・業務タスクに紐づく文書ファイルパスを、文書種類ごとのカスタマイズ可能な番号生成ルールに基づいて自動生成・管理するシステム。約10,000件規模の文書を扱い、複数ユーザーの同時読み取り・排他的書き込みをサポート。絶対パス（Unix/Linux、Windows UNC）に対応し、論理削除による監査証跡を保持。

## Technical Context
**Language/Version**: Rust 1.75+ (バックエンド), TypeScript 5.0+ (フロントエンド)  
**Primary Dependencies**: 
- Backend: axum (Web API), sqlx (非同期SQLite、型安全), serde (シリアライゼーション), chrono (タイムスタンプ), tokio (async runtime)
- Frontend: Svelte 4.0+, Vite (ビルドツール), TypeScript
- Storage: SQLite3 (sqlx、コンパイル時型チェック、非同期)
**Storage**: SQLite3（sqlx経由、ファイルベース埋め込みDB、~10,000レコード想定、WALモードで並行読み取り最適化）  
**Testing**: 
- Backend: cargo test（ユニット、統合、契約テスト）、sqlx::test（DBテスト）
- Frontend: vitest（コンポーネント、統合テスト）
**Target Platform**: 
- Backend: クロスプラットフォーム（Unix/Linux、Windows、Web APIサーバー）
- Frontend: モダンブラウザ（Chrome, Firefox, Safari, Edge）
**Project Type**: Webアプリケーション（Rustバックエンド + Svelteフロントエンド）  
**Performance Goals**: 
- 文書番号生成: <10ms
- クエリ応答: <100ms（10,000件中の検索）
- 同時読み取り: 10+ユーザー
- API応答時間: <200ms (p95)
**Constraints**: 
- 絶対パスのみサポート（相対パス不可）
- Windows UNCパス対応必須
- UTF-8マルチバイト文字対応（りん議、教育等）
- 論理削除（物理削除なし）
- 認証は将来的にSSO/LDAP/AD対応（当初は未実装）
- CORS対応（フロントエンド・バックエンド分離）
**Scale/Scope**: 
- ~10,000文書パス
- 6エンティティ（Department, Section, User, BusinessTask, DocumentType, DocumentPath, PathGenerationRule）
- 39機能要件
- 複数文書種類、各種類が独自の番号生成ルールを定義可能
- Web UI: 文書一覧、作成、検索、削除機能

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle I: Specification-First Development
- [x] Complete spec.md exists with all sections filled
- [x] Spec focuses on WHAT/WHY, not HOW
- [x] No technical implementation details in spec
- [x] Written for business stakeholders

### Principle II: Test-Driven Development
- [ ] Contract tests will be written before implementation (Phase 1)
- [ ] Integration tests will be written before implementation (Phase 1)
- [ ] All tests will use `cargo test` framework
- [ ] Tests will fail initially (red-green-refactor cycle)
- [ ] Integration tests will live in `tests/` directory

### Principle III: Template-Driven Automation
- [x] Using plan-template.md for this document
- [x] Following Execution Flow steps sequentially
- [ ] Will use tasks-template.md for Phase 2 (/tasks command)
- [x] Scripts used: setup-plan.sh

### Principle IV: Structured Documentation
- [x] Feature lives in `specs/001-db/`
- [x] spec.md completed
- [x] plan.md in progress (this file)
- [ ] research.md (Phase 0 output)
- [ ] data-model.md (Phase 1 output)
- [ ] contracts/ (Phase 1 output)
- [ ] quickstart.md (Phase 1 output)
- [ ] tasks.md (Phase 2, /tasks command)

### Principle V: Clarity Over Assumptions
- [x] Spec has Clarifications section with 8 Q&A pairs
- [x] All critical unknowns resolved
- [x] Technical Context fields filled (1 research item: 永続化層選択)
- [ ] Phase 0 will resolve remaining unknown (storage choice)

### Principle VI: Rust Idioms & Safety
- [ ] Will use Result<T, E> for error handling (Phase 1 design)
- [ ] Will ensure cargo build with no warnings (Phase 4 implementation)
- [ ] Will pass cargo clippy (Phase 4 implementation)
- [ ] Public APIs will have rustdoc comments (Phase 1 contracts)
- [ ] Will use std types preferentially (minimize external deps)
- [ ] Will use cargo fmt for formatting (Phase 4 implementation)

**Initial Gate Status**: ✅ PASS (1 research item to resolve in Phase 0)

---

## Post-Design Constitution Check
*Re-evaluated after Phase 1 design completion*

### Technology Decisions Review

**✅ Rust Idioms & Safety (Principle VI)**:
- sqlx使用: コンパイル時型チェック、非同期対応（憲法原則準拠）
- Result<T, E>: 全APIで使用（contracts定義済み）
- 外部依存正当化: sqlx（非同期DB必須）、axum（Web API必須）、serde（標準シリアライゼーション）
- std優先: PathBuf（UNCパス）、String（UTF-8）使用

**✅ Test-Driven Development (Principle II)**:
- 契約テスト定義済み: contracts/document_path_api.md, generation_api.md, query_api.md
- 統合テスト: quickstart.md に10シナリオ定義
- sqlx::test: DBテスト用マクロ使用予定
- tests/ディレクトリ構造: contract/, integration/, unit/

**✅ Specification-First (Principle I)**:
- spec.md完全（明確化セッション完了、39機能要件）
- 実装詳細なし（axum, sqlx, Svelteは計画段階で決定）

**✅ Template-Driven (Principle III)**:
- plan-template.md使用
- research.md, data-model.md, contracts/, quickstart.md生成
- migrations/ディレクトリ追加（sqlx-cli管理）

**✅ Structured Documentation (Principle IV)**:
- specs/001-db/構造完全
- SQLスキーマ定義: data-model.md
- API契約: contracts/3ファイル

**Complexity Assessment**:
- 外部依存数: 適切（sqlx, axum, tokio, serde, chrono - すべて正当化済み）
- フロントエンド分離: 適切（FR-023要件対応）
- プロジェクト構造: Webアプリ標準（backend/, frontend/）

**Post-Design Gate Status**: ✅ PASS（憲法違反なし、複雑性適切）

## Project Structure

### Documentation (this feature)
```
specs/001-db/
├── spec.md              # Feature specification (complete)
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
backend/
├── src/
│   ├── models/          # データモデル（Department, Section, User, BusinessTask, DocumentType, DocumentPath, PathGenerationRule）
│   ├── services/        # ビジネスロジック（文書番号生成、パス管理、クエリ）
│   ├── storage/         # 永続化層（SQLite3、sqlx）
│   ├── api/             # Web API エンドポイント（axum）
│   ├── lib.rs          # ライブラリAPI公開
│   └── main.rs         # APIサーバー エントリーポイント
├── migrations/         # SQLマイグレーション（sqlx-cli管理）
├── tests/
│   ├── contract/       # 契約テスト（API契約検証）
│   ├── integration/    # 統合テスト（シナリオベース）
│   └── unit/           # ユニットテスト
├── Cargo.toml          # 依存関係、メタデータ
└── documents.db        # SQLite3データベースファイル（実行時生成）

frontend/
├── src/
│   ├── routes/         # SvelteKitルート（ファイルベースルーティング）
│   │   ├── +page.svelte              # ダッシュボード
│   │   ├── documents/
│   │   │   ├── +page.svelte          # 文書一覧
│   │   │   ├── [id]/+page.svelte     # 文書詳細
│   │   │   └── create/+page.svelte   # 文書作成
│   │   └── search/+page.svelte       # 検索
│   ├── lib/
│   │   ├── components/
│   │   │   ├── DocumentList.svelte
│   │   │   ├── DocumentForm.svelte
│   │   │   └── SearchBar.svelte
│   │   └── api/
│   │       └── documents.ts   # API client
│   └── app.html
├── static/             # 静的アセット
├── package.json        # フロントエンド依存関係
├── svelte.config.js    # Svelte設定
└── vite.config.ts      # Viteビルド設定
```

**Structure Decision**: Webアプリケーション構成。Rustバックエンド（axum + sqlx）とSvelteフロントエンド（SvelteKit）に分離。バックエンドはライブラリとしても利用可能（`lib.rs`公開）。sqlx-cliでスキーママイグレーション管理。SvelteKitのファイルベースルーティングでフロントエンド構成。

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context}"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain}"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Entity name, fields, relationships
   - Validation rules from requirements
   - State transitions if applicable

2. **Generate API contracts** from functional requirements:
   - For each user action → endpoint
   - Use standard REST/GraphQL patterns
   - Output OpenAPI/GraphQL schema to `/contracts/`

3. **Generate contract tests** from contracts:
   - One test file per endpoint
   - Assert request/response schemas
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Each story → integration test scenario
   - Quickstart test = story validation steps

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/bash/update-agent-context.sh copilot`
     **IMPORTANT**: Execute it exactly as specified above. Do not add or remove any arguments.
   - If exists: Add only NEW tech from current plan
   - Preserve manual additions between markers
   - Update recent changes (keep last 3)
   - Keep under 150 lines for token efficiency
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, agent-specific file

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each contract → contract test task [P]
- Each entity → model creation task [P] 
- Each user story → integration test task
- Implementation tasks to make tests pass

**Ordering Strategy**:
- TDD order: Tests before implementation 
- Dependency order: Models before services before UI
- Mark [P] for parallel execution (independent files)

**Estimated Output**: 25-30 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |


## Phase 2: Task Planning Approach
*このセクションは/tasksコマンドによるtasks.md生成の戦略を記述（実行はしない）*

### Task Generation Strategy

**1. Contract-Driven Task Creation**:
- contracts/の各APIファイル → 個別タスクグループ
- 例: `document_path_api.md` → 7タスク（create_document_auto, create_document_manual, get_document_by_id, ...）
- 順序: 契約テスト → 実装 → 統合テスト（TDD原則）

**2. Data Model Task Creation**:
- data-model.mdの7エンティティ → 個別タスクグループ
- 例: `DocumentPath構造体` → migrations作成 → Rustモデル実装 → バリデーション実装
- 順序: マイグレーション → モデル → ビジネスロジック

**3. Test-First Ordering**:
- TDD原則（憲法Principle II）に従い、各機能ごとに：
  1. 契約テスト実装（コンパイル時型チェック）
  2. 統合テストシナリオ実装（quickstart.md参照）
  3. 本体実装
  4. パフォーマンス検証

**4. Parallelization Marking**:
- 独立タスクに`[P]`マーク:
  - Department, Section, User, BusinessTask, DocumentTypeエンティティは並列化可能
  - PathGenerationRule, DocumentPathは依存関係あり（順次）
  - contracts/の契約テスト定義は並列化可能
  - 本体実装は依存関係に応じて順次

**5. Quickstart Integration**:
- quickstart.md の10ステップ → 検証タスク:
  1. DB初期化検証
  2. 組織構造セットアップ検証
  3. ユーザー作成検証
  4. DocumentType定義検証
  5. 自動生成（AGI2509001）検証
  6. マルチバイト（りん議I-25009）検証
  7. クエリ検証
  8. 論理削除検証
  9. 並行読み取り検証
  10. パフォーマンス検証（<10ms）

**6. Task Count Estimate**:
- データモデル: 7エンティティ × 3タスク（migration, model, validation） = 21タスク
- API契約: 15関数 × 2タスク（契約テスト, 実装） = 30タスク
- 統合テスト: 10シナリオ × 1タスク = 10タスク
- インフラ: 5タスク（DB setup, axum server, Svelte frontend基盤, CI/CD, deployment）
- **合計推定**: 65-70タスク

**7. Backend/Frontend Split**:
- Backend（Rust+axum+sqlx）: 55-60タスク
- Frontend（Svelte+TypeScript）: 10-15タスク
  - API型定義（OpenAPI生成）
  - コンポーネント実装（DocumentList, DocumentForm, ...）
  - ストア実装（Svelte stores）
  - ルーティング（SvelteKit routes/）

**8. Dependency Chain**:
```
migrations (sqlx-cli)
  └→ models (Rust structs + sqlx::FromRow)
      └→ storage (DB操作 + sqlx::query!)
          └→ services (ビジネスロジック)
              └→ api (axum handlers)
                  └→ integration tests (quickstart scenarios)
```

**9. /tasks Command Output Target**:
- tasks.mdに上記戦略を実行した結果を記述
- tasks-template.mdに従った構造
- 各タスク: ID, タイトル, 説明, 受け入れ基準, ステータス, [P]マーク
- 順序: TDD原則に基づく依存関係順

---

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command) - 7研究質問解決
- [x] Phase 1: Design complete (/plan command) - data-model, contracts, quickstart作成
- [x] Phase 2: Task planning complete (/plan command - describe approach only) - 65-70タスク推定
- [ ] Phase 3: Tasks generated (/tasks command) - tasks.md生成待ち
- [ ] Phase 4: Implementation complete - コード実装待ち
- [ ] Phase 5: Validation passed - 検証待ち

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS - 憲法違反なし
- [x] All NEEDS CLARIFICATION resolved - 8件の明確化完了
- [x] Complexity deviations documented - すべて正当化済み（sqlx, axum, Svelte）

**Artifact Status**:
- [x] spec.md - 39機能要件、6エンティティ、8明確化
- [x] research.md - 7研究質問解決、技術スタック確定
- [x] data-model.md - 7エンティティ、SQLスキーマ
- [x] contracts/ - 5契約ファイル（README + 3 API契約）
- [x] quickstart.md - 10検証シナリオ
- [x] plan.md - 実装計画（本ドキュメント）
- [ ] tasks.md - /tasksコマンド待ち

---

## Next Steps

**✅ この計画ドキュメント（plan.md）は完成しました。**

次のコマンドを実行してください：

```bash
/tasks
```

`/tasks`コマンドは：
1. plan.mdのPhase 2戦略を読み込み
2. tasks-template.mdに従ったtasks.mdを生成
3. 65-70個の実装タスクを作成（TDD順序）
4. 並列化可能タスクに[P]マークを付与
5. 依存関係チェーンを反映

---
*Based on Constitution v1.1.0 - See `/memory/constitution.md`*

````

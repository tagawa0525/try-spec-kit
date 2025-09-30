
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


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [ ] Phase 0: Research complete (/plan command)
- [ ] Phase 1: Design complete (/plan command)
- [ ] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [ ] Initial Constitution Check: PASS
- [ ] Post-Design Constitution Check: PASS
- [ ] All NEEDS CLARIFICATION resolved
- [ ] Complexity deviations documented

---
*Based on Constitution v1.1.0 - See `/memory/constitution.md`*

````

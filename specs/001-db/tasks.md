# Tasks: Document Path Management Database

**Input**: Design documents from `/home/tagawa/try-spec-kit-via-vs_code/specs/001-db/`
**Prerequisites**: plan.md, research.md, data-model.md, contracts/, quickstart.md

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → Extract: Rust+axum+sqlx (backend), Svelte+TypeScript (frontend)
2. Load design documents:
   → data-model.md: 7 entities (Department, Section, User, BusinessTask, DocumentType, PathGenerationRule, DocumentPath)
   → contracts/: 4 API files (document_path_api, generation_api, query_api, organization_api)
   → quickstart.md: 10 validation scenarios
3. Generate tasks by category:
   → Setup: Project init, migrations, dependencies
   → Tests: Contract tests (7 functions), integration tests (10 scenarios)
   → Core: Models (7 entities), storage (sqlx queries), services (business logic)
   → API: axum endpoints (REST handlers)
   → Frontend: Svelte components, routes, stores
   → Integration: DB setup, CORS, logging
   → Polish: Performance tests, docs
4. Apply task rules:
   → Different files = mark [P] for parallel
   → Same file = sequential (no [P])
   → Tests before implementation (TDD)
5. Tasks numbered T001-T072
6. Dependencies: migrations → models → storage → services → api → tests
7. Backend tasks (55), Frontend tasks (12), Integration tasks (5)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Paths based on plan.md structure: `backend/src/`, `frontend/src/`

## Path Conventions
- **Backend**: `backend/src/models/`, `backend/src/storage/`, `backend/src/services/`, `backend/src/api/`, `backend/tests/`
- **Frontend**: `frontend/src/routes/`, `frontend/src/lib/components/`, `frontend/src/lib/api/`
- **Migrations**: `backend/migrations/`

---

## Phase 3.1: Setup & Infrastructure (T001-T010)

### Backend Setup
- [x] T001 Create Rust workspace structure: `backend/src/{models,storage,services,api}`, `backend/tests/{contract,integration,unit}`
- [x] T002 Initialize Cargo.toml with dependencies: axum 0.7, sqlx 0.7 (sqlite, runtime-tokio), tokio 1.35, serde, chrono, thiserror
- [x] T003 [P] Configure cargo clippy and rustfmt in backend/rustfmt.toml and backend/.cargo/config.toml
- [ ] T004 Install sqlx-cli: `cargo install sqlx-cli --no-default-features --features sqlite`

### Frontend Setup
- [x] T005 Create SvelteKit project structure: `frontend/src/{routes,lib/components,lib/api}`
- [x] T006 Initialize package.json with dependencies: svelte 4.0, @sveltejs/kit, typescript 5.0, vite 5.0
- [x] T007 [P] Configure TypeScript in frontend/tsconfig.json and frontend/svelte.config.js

### Database Setup
- [x] T008 Create SQLite migration 001_initial_schema.sql in backend/migrations/: Create tables (departments, sections, users, business_tasks, document_types, generation_rules, documents, counters)
- [x] T009 Create SQLite migration 002_indexes.sql in backend/migrations/: Create indexes (document_number, document_type, department, section, business_task, deleted)
- [x] T010 Configure SQLite WAL mode in backend/src/storage/db.rs: `PRAGMA journal_mode=WAL`

---

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3

### Contract Tests - Document Path API (T011-T017)
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

- [x] T011 [P] Contract test create_document_auto signature in backend/tests/contract/test_document_path_create_auto.rs
- [x] T012 [P] Contract test create_document_manual signature in backend/tests/contract/test_document_path_create_manual.rs
- [x] T013 [P] Contract test get_document_by_id signature in backend/tests/contract/test_document_path_get_id.rs
- [x] T014 [P] Contract test get_document_by_number signature in backend/tests/contract/test_document_path_get_number.rs
- [x] T015 [P] Contract test update_document_path signature in backend/tests/contract/test_document_path_update.rs
- [x] T016 [P] Contract test delete_document signature in backend/tests/contract/test_document_path_delete.rs
- [x] T017 [P] Contract test search_documents signature in backend/tests/contract/test_document_path_search.rs

### Contract Tests - Generation API (T018-T020)
- [x] T018 [P] Contract test generate_document_number signature in backend/tests/contract/test_generation_generate.rs
- [x] T019 [P] Contract test get_next_counter signature in backend/tests/contract/test_generation_counter.rs
- [x] T020 [P] Contract test build_scope_key signature in backend/tests/contract/test_generation_scope.rs

### Contract Tests - Query API (T021-T026)
- [x] T021 [P] Contract test get_all_documents signature in backend/tests/contract/test_query_all.rs
- [x] T022 [P] Contract test get_documents_by_type signature in backend/tests/contract/test_query_by_type.rs
- [x] T023 [P] Contract test get_documents_by_department signature in backend/tests/contract/test_query_by_dept.rs
- [x] T024 [P] Contract test get_documents_by_section signature in backend/tests/contract/test_query_by_section.rs
- [x] T025 [P] Contract test get_documents_by_task signature in backend/tests/contract/test_query_by_task.rs
- [x] T026 [P] Contract test DocumentQuery builder pattern in backend/tests/contract/test_query_builder.rs

### Integration Tests - Quickstart Scenarios (T027-T036)
- [x] T027 [P] Integration test Step 1: DB initialization in backend/tests/integration/test_quickstart_step1_init.rs
- [x] T028 [P] Integration test Step 2: Organization setup in backend/tests/integration/test_quickstart_step2_org.rs
- [x] T029 [P] Integration test Step 3: User creation in backend/tests/integration/test_quickstart_step3_user.rs
- [x] T030 [P] Integration test Step 4: DocumentType definition in backend/tests/integration/test_quickstart_step4_type.rs
- [x] T031 [P] Integration test Step 5: Auto-generate AGI2509001 in backend/tests/integration/test_quickstart_step5_autogen.rs
- [x] T032 [P] Integration test Step 6: Multi-byte りん議I-25009 in backend/tests/integration/test_quickstart_step6_multibyte.rs
- [x] T033 [P] Integration test Step 7: Query by type in backend/tests/integration/test_quickstart_step7_query.rs
- [x] T034 [P] Integration test Step 8: Logical deletion in backend/tests/integration/test_quickstart_step8_delete.rs
- [x] T035 [P] Integration test Step 9: Concurrent reads (10 threads) in backend/tests/integration/test_quickstart_step9_concurrent.rs
- [x] T036 [P] Integration test Step 10: Performance (<10ms avg) in backend/tests/integration/test_quickstart_step10_performance.rs

---

## Phase 3.3: Core Implementation (ONLY after tests are failing)

### Models - Newtypes & Core Types (T037-T043)
- [ ] T037 [P] DeptCode, SectionCode, UserId, TaskId, DocumentId, TypeCode newtypes in backend/src/models/newtypes.rs
- [ ] T038 [P] Permissions struct in backend/src/models/permissions.rs
- [ ] T039 [P] Department model in backend/src/models/department.rs
- [ ] T040 [P] Section model in backend/src/models/section.rs
- [ ] T041 [P] User model in backend/src/models/user.rs
- [ ] T042 [P] BusinessTask model in backend/src/models/business_task.rs
- [ ] T043 [P] DocumentType model in backend/src/models/document_type.rs

### Models - Generation Rules & Document Path (T044-T045)
- [ ] T044 PathGenerationRule model with RuleComponent enum in backend/src/models/generation_rule.rs
- [ ] T045 DocumentPath model in backend/src/models/document_path.rs

### Storage Layer - sqlx Queries (T046-T053)
- [ ] T046 [P] Department CRUD with sqlx::query! in backend/src/storage/department.rs
- [ ] T047 [P] Section CRUD with sqlx::query! in backend/src/storage/section.rs
- [ ] T048 [P] User CRUD with sqlx::query! in backend/src/storage/user.rs
- [ ] T049 [P] BusinessTask CRUD with sqlx::query! in backend/src/storage/business_task.rs
- [ ] T050 [P] DocumentType CRUD with sqlx::query! in backend/src/storage/document_type.rs
- [ ] T051 DocumentPath CRUD with sqlx::query! in backend/src/storage/document_path.rs
- [ ] T052 Counter management (get_next_counter, increment) in backend/src/storage/counter.rs
- [ ] T053 Query operations (by_type, by_dept, by_section, by_task) in backend/src/storage/query.rs

### Services - Business Logic (T054-T057)
- [ ] T054 Document number generation service (apply RuleComponent, format output) in backend/src/services/generation_service.rs
- [ ] T055 Document creation service (validate, generate number, save path) in backend/src/services/document_service.rs
- [ ] T056 Organization service (dept/section/user management) in backend/src/services/organization_service.rs
- [ ] T057 Query service (build queries, filter, include_deleted) in backend/src/services/query_service.rs

### API Layer - axum Handlers (T058-T063)
- [ ] T058 POST /api/documents (auto-generated) handler in backend/src/api/documents/create_auto.rs
- [ ] T059 POST /api/documents/manual handler in backend/src/api/documents/create_manual.rs
- [ ] T060 GET /api/documents/:id handler in backend/src/api/documents/get_by_id.rs
- [ ] T061 GET /api/documents/number/:number handler in backend/src/api/documents/get_by_number.rs
- [ ] T062 PUT /api/documents/:id/path handler in backend/src/api/documents/update_path.rs
- [ ] T063 DELETE /api/documents/:id handler in backend/src/api/documents/delete.rs

### Error Handling & Validation (T064-T065)
- [ ] T064 Error enum with thiserror in backend/src/error.rs
- [ ] T065 Input validation (absolute path check, type code format) in backend/src/validation.rs

---

## Phase 3.4: Frontend Implementation (T066-T072)

### Svelte Components (T066-T069)
- [ ] T066 [P] DocumentList component in frontend/src/lib/components/DocumentList.svelte
- [ ] T067 [P] DocumentForm component (create/edit) in frontend/src/lib/components/DocumentForm.svelte
- [ ] T068 [P] SearchBar component in frontend/src/lib/components/SearchBar.svelte
- [ ] T069 [P] DocumentDetails component in frontend/src/lib/components/DocumentDetails.svelte

### SvelteKit Routes (T070-T072)
- [ ] T070 Dashboard route in frontend/src/routes/+page.svelte
- [ ] T071 Documents list route in frontend/src/routes/documents/+page.svelte
- [ ] T072 Document detail route in frontend/src/routes/documents/[id]/+page.svelte

---

## Dependencies

### Critical Path (Sequential)
```
T008-T010 (migrations) 
  → T037-T045 (models)
    → T046-T053 (storage)
      → T054-T057 (services)
        → T058-T063 (API)
          → T027-T036 (integration tests pass)
```

### Test Dependencies
- Contract tests (T011-T026) MUST fail before any implementation (T037-T065)
- Integration tests (T027-T036) MUST be written before implementation
- Integration tests pass only after implementation complete

### Frontend Dependencies
- Backend API (T058-T063) must be available before frontend integration
- Components (T066-T069) can be developed in parallel
- Routes (T070-T072) depend on components

### Parallel Groups
**Group A - Contract Tests** (can run simultaneously after T010):
- T011-T017 (document_path API)
- T018-T020 (generation API)
- T021-T026 (query API)

**Group B - Integration Tests** (can run simultaneously after T010):
- T027-T036 (all quickstart scenarios)

**Group C - Models** (can run simultaneously after T010):
- T037, T038, T039, T040, T041, T042, T043 (all independent models)

**Group D - Storage Layer** (can run simultaneously after T037-T045):
- T046, T047, T048, T049, T050 (independent entity storage)

**Group E - Frontend Components** (can run simultaneously after T058-T063):
- T066, T067, T068, T069 (all independent components)

---

## Parallel Execution Examples

### Example 1: Launch all contract tests together
```bash
# After T010 (migrations complete), launch in parallel:
Task: "Contract test create_document_auto signature in backend/tests/contract/test_document_path_create_auto.rs"
Task: "Contract test create_document_manual signature in backend/tests/contract/test_document_path_create_manual.rs"
Task: "Contract test get_document_by_id signature in backend/tests/contract/test_document_path_get_id.rs"
# ... (T011-T026 all together)
```

### Example 2: Launch all model implementations together
```bash
# After tests written and migrations run, launch in parallel:
Task: "DeptCode, SectionCode, UserId, TaskId, DocumentId, TypeCode newtypes in backend/src/models/newtypes.rs"
Task: "Permissions struct in backend/src/models/permissions.rs"
Task: "Department model in backend/src/models/department.rs"
# ... (T037-T043 all together)
```

### Example 3: Launch frontend components together
```bash
# After API endpoints available, launch in parallel:
Task: "DocumentList component in frontend/src/lib/components/DocumentList.svelte"
Task: "DocumentForm component in frontend/src/lib/components/DocumentForm.svelte"
Task: "SearchBar component in frontend/src/lib/components/SearchBar.svelte"
Task: "DocumentDetails component in frontend/src/lib/components/DocumentDetails.svelte"
```

---

## Notes

### TDD Enforcement
- **CRITICAL**: All contract tests (T011-T026) MUST fail before implementation
- Integration tests (T027-T036) MUST be written before services/API
- Verify tests fail: `cargo test` should show failures
- Only proceed to implementation after confirming test failures

### File Organization
- [P] tasks = different files, no dependencies
- Same-file edits = sequential (e.g., T051-T053 all touch storage layer, but different files)
- Commit after each task completion

### Performance Validation
- T036 validates <10ms generation requirement
- Run performance tests after T055 (generation service) completion
- Target: <10ms for document creation, <100ms for queries

### Multi-byte Support
- T032 validates UTF-8 multi-byte document types (りん議)
- Rust String handles UTF-8 natively (no special encoding needed)
- Test with Japanese characters throughout integration tests

### Concurrency Testing
- T035 validates 10+ concurrent reads
- SQLite WAL mode (T010) enables concurrent reads
- Exclusive write locks enforced by sqlx transactions

---

## Task Generation Rules
*Applied during main() execution*

1. **From Contracts** (4 files):
   - document_path_api.md → 7 contract tests (T011-T017) + 7 API handlers (T058-T063)
   - generation_api.md → 3 contract tests (T018-T020) + 1 service (T054)
   - query_api.md → 6 contract tests (T021-T026) + 2 storage (T053, T057)
   - organization_api.md → organization service (T056)

2. **From Data Model** (7 entities):
   - Each entity → model task (T037-T045) + storage task (T046-T052)
   - Relationships → service layer tasks (T054-T057)

3. **From Quickstart** (10 scenarios):
   - Each step → integration test (T027-T036)
   - Validation scenarios map to acceptance criteria

4. **Ordering**:
   - Setup (T001-T010) → Tests (T011-T036) → Models (T037-T045) → Storage (T046-T053) → Services (T054-T057) → API (T058-T065) → Frontend (T066-T072)
   - Dependencies block parallel execution within category

---

## Validation Checklist
*GATE: Checked before marking complete*

- [x] All contracts have corresponding tests (T011-T026 cover all API functions)
- [x] All entities have model tasks (T037-T045 cover 7 entities)
- [x] All tests come before implementation (T011-T036 before T037-T065)
- [x] Parallel tasks truly independent (different files, verified)
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task
- [x] TDD workflow enforced (contract tests → integration tests → implementation)
- [x] Quickstart scenarios covered (T027-T036 map to 10 steps)
- [x] Performance requirements included (T036 validates <10ms)
- [x] Multi-byte support tested (T032 validates UTF-8)
- [x] Concurrency tested (T035 validates 10+ concurrent reads)

---

**Task Count**: 72 tasks total
- Setup: 10 tasks (T001-T010)
- Contract Tests: 16 tasks (T011-T026)
- Integration Tests: 10 tasks (T027-T036)
- Models: 9 tasks (T037-T045)
- Storage: 8 tasks (T046-T053)
- Services: 4 tasks (T054-T057)
- API: 8 tasks (T058-T065)
- Frontend: 7 tasks (T066-T072)

**Estimated Timeline**: 
- Phase 3.1 (Setup): 1-2 days
- Phase 3.2 (Tests): 3-4 days
- Phase 3.3 (Backend): 5-7 days
- Phase 3.4 (Frontend): 2-3 days
- **Total**: 11-16 days (assuming 1 developer)

**Parallel Potential**: 
- With 3 developers: ~7-10 days (parallel groups A-E)
- With 5+ developers: ~5-7 days (maximum parallelization)

---

**Status**: Tasks ready for execution. Start with T001.

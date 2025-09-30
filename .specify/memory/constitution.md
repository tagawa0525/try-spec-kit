<!--
SYNC IMPACT REPORT
==================
Version Change: 1.0.0 → 1.1.0
Date: 2025-09-30

Modified Principles:
- EXPANDED: II. Test-Driven Development - Added Rust-specific testing requirements (cargo test, integration tests)
- NEW: VI. Rust Idioms & Safety - Added Rust-specific development standards

Added Sections:
- Principle VI: Rust Idioms & Safety (Rust-specific requirements)

Removed Sections:
- None

Templates Requiring Updates:
✅ .specify/templates/plan-template.md - Updated (constitution version reference v2.1.1 → v1.1.0)
✅ .specify/templates/spec-template.md - Aligned (language-agnostic)
✅ .specify/templates/tasks-template.md - Aligned (supports cargo commands)
✅ .specify/templates/agent-file-template.md - Aligned (will extract Rust commands)
✅ .github/prompts/*.prompt.md - Aligned (language-agnostic workflow)

Follow-up TODOs:
- None
-->

# Spec Kit Constitution

## Core Principles

### I. Specification-First Development

Every feature begins with a complete, unambiguous specification before any technical planning or implementation.

**Rules**:
- MUST create `spec.md` before `plan.md` or code
- Specifications MUST focus on WHAT and WHY, never HOW
- All ambiguities MUST be marked with `[NEEDS CLARIFICATION: specific question]`
- Specifications MUST be written for business stakeholders, not developers
- Technical implementation details (languages, frameworks, APIs) are FORBIDDEN in specs

**Rationale**: Clear requirements prevent rework and ensure stakeholder alignment before costly development begins.

### II. Test-Driven Development (NON-NEGOTIABLE)

All implementation follows strict red-green-refactor TDD cycles using Rust testing tools.

**Rules**:
- Tests MUST be written before implementation code
- Tests MUST fail initially (prove they detect absence of feature)
- Implementation MUST make failing tests pass
- Contract tests and integration tests MUST be completed before any implementation
- No implementation without failing tests
- MUST use `cargo test` for unit tests (in same file or `tests/` directory)
- Integration tests MUST live in `tests/` directory with separate files per test suite
- MUST use `#[test]` and `#[cfg(test)]` attributes appropriately
- Test modules MUST follow Rust conventions (`mod tests` for unit tests)

**Rationale**: TDD ensures correctness, prevents regression, and validates requirements are testable. Rust's built-in testing framework enforces this discipline naturally.

### III. Template-Driven Automation

All artifacts follow executable templates with defined workflows and validation gates.

**Rules**:
- MUST use templates from `.specify/templates/` for all documents
- Templates contain executable `Execution Flow` sections that MUST be followed
- Placeholders (e.g., `[FEATURE_NAME]`) MUST be replaced with concrete values
- Templates include validation checklists that MUST pass
- Scripts in `.specify/scripts/` MUST be used for automation (e.g., feature creation, plan setup)

**Rationale**: Templates ensure consistency, reduce manual errors, and encode institutional knowledge.

### IV. Structured Documentation

Documentation follows a hierarchical structure with clear separation of concerns.

**Rules**:
- Feature documentation MUST live in `specs/[###-feature-name]/`
- Each feature MUST produce: `spec.md`, `plan.md`, `tasks.md`
- Phase-specific artifacts MUST be generated: `research.md`, `data-model.md`, `contracts/`, `quickstart.md`
- Agent-specific guidance files MUST stay under 150 lines for token efficiency
- Templates and prompts MUST be versioned and tracked

**Rationale**: Predictable structure enables automation, improves discoverability, and supports AI agent workflows.

### V. Clarity Over Assumptions

When information is missing or ambiguous, mark it explicitly rather than guessing.

**Rules**:
- Use `[NEEDS CLARIFICATION: specific question]` for all unknowns
- Technical Context fields default to `NEEDS CLARIFICATION` if not explicitly known
- Phase 0 Research MUST resolve all clarifications before proceeding
- Validation checklists MUST fail if clarifications remain unresolved
- Errors MUST be raised for blocking unknowns (e.g., "Cannot determine user scenarios")

**Rationale**: Explicit unknowns prevent silent failures and misunderstood requirements. Better to ask than to implement wrong.

### VI. Rust Idioms & Safety

All Rust code follows idiomatic patterns and leverages the type system for safety.

**Rules**:
- MUST compile without warnings (`cargo build` clean, use `#[allow(...)]` sparingly with justification)
- MUST pass `cargo clippy` with default lints (document exceptions in code)
- MUST use `Result<T, E>` for recoverable errors, never panic in library code
- CLI applications MAY panic for unrecoverable errors, but MUST provide clear error messages
- MUST leverage ownership, borrowing, and lifetimes correctly (no unnecessary clones)
- MUST use `cargo fmt` for consistent formatting
- Public APIs MUST have rustdoc comments (`///`) with examples
- MUST prefer `std` types; external crates require justification in Complexity Tracking
- MUST use semantic versioning for crate versions (align with constitution versioning)

**Rationale**: Rust's compiler enforces memory safety and thread safety. Following idiomatic patterns maximizes these guarantees and produces maintainable code. Clippy catches common mistakes and anti-patterns.

## Workflow Gates

**Constitutional Checkpoints** (enforced at specific phases):

1. **Specification Gate** (`spec.md`):
   - No `[NEEDS CLARIFICATION]` markers remain (or documented deferred)
   - Requirements are testable and unambiguous
   - No implementation details present
   - Written for non-technical stakeholders

2. **Planning Gate** (`plan.md`):
   - Constitution Check section completed
   - All Technical Context fields resolved
   - Phase 0 Research resolves all unknowns
   - Post-Design Constitution Check passes

3. **Tasks Gate** (`tasks.md`):
   - Tests numbered before corresponding implementation
   - Parallel tasks (`[P]`) verified as independent
   - All contracts have test tasks
   - All entities have model tasks

4. **Implementation Gate**:
   - Tests fail before implementation
   - Tests pass after implementation
   - No code committed without passing tests
   - `cargo build` completes without warnings
   - `cargo clippy` passes with default lints
   - `cargo test` passes all tests (unit + integration)
   - Quickstart validation executes successfully

## Governance

**Authority**: This constitution supersedes all other development practices and conventions.

**Amendment Process**:
- Constitution changes MUST be documented with version bump rationale
- Version follows semantic versioning:
  - **MAJOR**: Backward-incompatible governance changes, principle removals/redefinitions
  - **MINOR**: New principles, materially expanded sections
  - **PATCH**: Clarifications, wording improvements, typo fixes
- Amendments MUST include Sync Impact Report identifying affected templates
- All dependent templates and documentation MUST be updated before finalizing amendment
- Changes MUST include ISO-formatted dates (YYYY-MM-DD)

**Compliance**:
- Template `Execution Flow` sections enforce constitutional requirements
- Validation checklists MUST be completed and passing
- Gate failures MUST block progression to next phase
- Complexity violations MUST be documented and justified in Complexity Tracking tables

**Version Control**:
- Constitution version referenced in all templates (e.g., `Based on Constitution v1.0.0`)
- Templates updated when constitution changes
- Agent guidance files regenerated to reflect new principles

**Living Document**:
- This constitution evolves with the project
- Feedback from template execution informs amendments
- Retrospectives identify gaps or outdated principles

**Version**: 1.1.0 | **Ratified**: 2025-09-30 | **Last Amended**: 2025-09-30
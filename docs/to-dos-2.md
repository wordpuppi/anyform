# axum-sea-forms Implementation Tracker v2

> **Status**: Active
> **Started**: 2024-12-25
> **Last Updated**: 2024-12-25
> **PRD**: [axum-sea-forms-prd-0.3.2.md](/Users/rick/p/wordpuppi/docs/prd/libs/asf/axum-sea-forms-prd-0.3.2.md)

---

## Phase 1: Foundation (v0.1.0) — COMPLETE

### Entities & Migrations
- [x] SeaORM entities: Form, Step, Field, FieldOption, Submission, Result
- [x] Migrations for SQLite, PostgreSQL, MySQL (6 tables: asf_forms, asf_steps, asf_fields, asf_field_options, asf_submissions, asf_results)

### Schema Types
- [x] ValueType enum (21 field types)
- [x] ValidationRules struct
- [x] UiOptions struct
- [x] FormSettings struct
- [x] FieldValue enum

### Validation Engine
- [x] `validate_submission()` - single-step validation
- [x] Email, URL, tel, date/time format validation
- [x] Required field validation
- [x] String length (min/max)
- [x] Numeric range (min/max)
- [x] Pattern/regex validation
- [x] Array selection validation (min/max selections)

### Renderers
- [x] JsonRenderer - form schema to JSON
- [x] HtmlRenderer - form to HTML with HtmlOptions
- [x] TeraRenderer - context builder for Tera templates

### Extractors
- [x] FormSubmission extractor
- [x] ValidatedSubmission extractor

### Router
- [x] FormsRouter with builder pattern
- [x] Public routes: GET/POST /forms/{slug}
- [x] Feature-gated handlers

---

## Phase 2: CRUD & Testing (v0.2.0) — COMPLETE

### Form Builder Service
- [x] `FormBuilder::create(db, input)` - create form with nested steps/fields/options
- [x] `FormBuilder::update(db, id, input)` - update existing form
- [x] `FormBuilder::soft_delete(db, id)` - mark deleted_at
- [x] `FormBuilder::hard_delete(db, id)` - permanent deletion
- [x] `FormBuilder::restore(db, id)` - restore soft-deleted
- [x] `FormBuilder::find_by_slug(db, slug)`
- [x] `FormBuilder::find_by_id(db, id)`
- [x] `FormBuilder::list(db)`

### Builder Input Types
- [x] CreateFormInput with fluent API
- [x] CreateStepInput with fluent API
- [x] CreateFieldInput with fluent API
- [x] CreateOptionInput

### Database Seeding
- [x] `seed_all(db)` - create all example forms
- [x] `seed_contact_form(db)` - text, email, tel, textarea, radio
- [x] `seed_feedback_form(db)` - rating, NPS, multi_select
- [x] `seed_quiz_form(db)` - scoring with correct_answer/points
- [x] `clear_seeded_forms(db)`

### Admin API Endpoints
- [x] GET /admin/forms - list all forms
- [x] POST /admin/forms - create form
- [x] GET /admin/forms/{id} - get form by ID
- [x] PUT /admin/forms/{id} - update form
- [x] DELETE /admin/forms/{id} - soft delete form
- [x] GET /admin/forms/{id}/submissions - list submissions
- [x] GET /admin/forms/{id}/submissions/{sub_id} - get submission
- [x] DELETE /admin/forms/{id}/submissions/{sub_id} - delete submission

### CLI Commands
- [x] `asf migrate [--up|--down|--status]`
- [x] `asf form list`
- [x] `asf form show <slug>`
- [x] `asf form create --file <path>`
- [x] `asf form update <slug> --file <path>`
- [x] `asf form delete <slug>`
- [x] `asf form export <slug> [--format json]`
- [x] `asf form render <slug>`
- [x] `asf submissions list --form <slug>`
- [x] `asf submissions show <id>`
- [x] `asf submissions delete <id>`
- [x] `asf submissions export --form <slug> [--format csv|json]`
- [x] `asf seed [--contact-only|--feedback-only|--quiz-only|--clear]`

---

## Phase 2.1: Forms as Code (v0.2.1) — COMPLETE

### CLI Sync
- [x] `asf form sync --folder <path>` - sync JSON files to database
- [x] Reads all `*.json` files from folder
- [x] Match forms by slug (upsert)
- [x] Create new / update existing
- [x] Additive sync (doesn't delete unmatched DB forms)
- [x] Reports created/updated/error counts

### API Sync Endpoint
- [x] POST /admin/forms/sync - accepts array of form definitions
- [x] Returns created/updated counts and errors

---

## Phase 2.2: API Polish & Distribution (v0.2.2) — COMPLETE

### ApiResponse Envelope
- [x] `ApiResponse<T>` struct with data, success, status, error, pagination, request_id, meta
- [x] `ApiError` struct with code, message, details
- [x] `PaginationInfo` struct (offset + cursor support)
- [x] Builder methods: `ok()`, `created()`, `not_found()`, `bad_request()`, `validation_failed()`
- [x] `with_request_id()`, `with_pagination()`, `with_meta()` chainable methods
- [x] IntoResponse impl for Axum

### RequestId Extractor
- [x] Extract from X-Request-ID header or auto-generate UUID
- [x] Unit tests for extractor

### asf serve Command
- [x] `asf serve [--host 0.0.0.0] [--port 3000]`
- [x] Starts API server with form routes

---

## Phase 3: Multi-Step & Conditions (v0.3.x) — IN PROGRESS

### Phase 3.0: Core — COMPLETE

#### axum-sea-forms-core Crate
- [x] Workspace crate for shared types
- [x] `ConditionOp` enum (eq, neq, gt, gte, lt, lte, contains, in, empty, not_empty)
- [x] `ConditionRule` enum (Simple, And, Or)
- [x] `ConditionRule::evaluate(data)` method
- [x] Comprehensive tests for condition evaluation

#### Server-Side Multi-Step
- [x] Step ordering and navigation
- [x] `validate_multi_step_submission()` - validates with step/field conditions
- [x] Conditional step display (evalexpr + ConditionRule)
- [x] Conditional field display
- [x] Errors grouped by step ID

---

### Phase 3.1: Embedded SQLite (v0.3.1) — PENDING

#### FormsRouter Embedded Support
- [ ] `FormsRouter::embedded()` - use SQLite at `./data/asf.db`
- [ ] `FormsRouter::embedded_at(path)` - custom SQLite path
- [ ] Auto-create database file and parent directories
- [ ] Auto-run migrations on startup

#### FormsRouterBuilder Methods
- [ ] `.embedded()` method
- [ ] `.db_path(path)` method
- [ ] `.auto_migrate(bool)` method

#### CLI Flags
- [ ] `--embedded` flag for all commands
- [ ] `--db-path <PATH>` flag (default: ./data/asf.db)
- [ ] DATABASE_URL still takes precedence if set

#### Feature Flag
- [ ] `sqlite-embedded` feature (enabled by default)
- [ ] Add `directories` crate dependency

#### Docker
- [ ] Update Docker image to use embedded SQLite by default
- [ ] /data volume mount for persistence

#### Tests & Docs
- [ ] Tests for embedded database flow
- [ ] Documentation updates

---

### Phase 3.2: WASM Client (v0.3.2) — PENDING

#### Crate Setup
- [ ] Create `axum-sea-forms-client/Cargo.toml`
- [ ] Add dependencies: wasm-bindgen, web-sys, js-sys, serde, serde-wasm-bindgen
- [ ] Add dependency on axum-sea-forms-core

#### Form Schema Types
- [ ] FormJson struct (mirrors server JSON output)
- [ ] StepJson struct
- [ ] FieldJson struct
- [ ] FieldOptionJson struct

#### FormClient Struct
- [ ] `FormClient::new()` constructor
- [ ] `FormClient::fetch(url)` - fetch form schema
- [ ] `FormClient::submit(url, data)` - submit form data
- [ ] JavaScript bindings via wasm-bindgen

#### FormState Struct
- [ ] `values: HashMap<String, FieldValue>`
- [ ] `errors: HashMap<String, Vec<String>>`
- [ ] `touched: HashSet<String>`
- [ ] `current_step: Option<Uuid>`
- [ ] `set_value(field, value)`
- [ ] `get_value(field)`
- [ ] `mark_touched(field)`

#### Condition Evaluation (Client-Side)
- [ ] Re-export ConditionRule from axum-sea-forms-core
- [ ] `visible_steps()` - filter steps by conditions
- [ ] `visible_fields(step_id)` - filter fields by conditions
- [ ] Tests for client-side evaluation

#### Client-Side Validation
- [ ] Port validation logic to WASM-compatible code
- [ ] `validate_field(field_name)` - validate single field
- [ ] `validate_step(step_id)` - validate all fields in step
- [ ] `validate_all()` - validate entire form
- [ ] `is_valid()` - check if form has no errors

#### Multi-Step Navigation
- [ ] `current_step()` - get current step
- [ ] `next_step()` - advance to next visible step
- [ ] `prev_step()` - go back to previous visible step
- [ ] `can_go_next()` / `can_go_prev()` - navigation availability
- [ ] `progress()` - returns (current, total) for progress indicator

#### API Client
- [ ] `fetch_form(slug)` via web-sys fetch API
- [ ] `submit_form(slug, data)` via web-sys fetch API
- [ ] Handle ApiResponse envelope
- [ ] Error handling for network/validation errors

#### JavaScript Bindings
- [ ] Export FormClient to JS via wasm-bindgen
- [ ] Export FormState methods
- [ ] TypeScript type definitions (.d.ts)

#### Examples & Documentation
- [ ] Vanilla JavaScript integration example
- [ ] Leptos integration notes
- [ ] Yew integration notes
- [ ] README with usage instructions

---

## Phase 4: Survey & Quiz Polish (v0.4.0) — PENDING

### Scoring Refinements
- [ ] Weighted scoring support
- [ ] Partial credit for multi-select
- [ ] Score normalization (percentage)

### Result Buckets
- [ ] Result bucket management API
- [ ] Auto-select result based on score range
- [ ] Custom result messages

### Analytics Helpers
- [ ] NPS calculation helper
- [ ] Rating average helper
- [ ] Response distribution helper

---

## Phase 5: Integration Tests — PENDING

### Test Infrastructure
- [ ] Create `axum-sea-forms/tests/` directory
- [ ] `tests/common/mod.rs` - shared utilities
- [ ] `tests/common/fixtures.rs` - test data builders
- [ ] `tests/common/db.rs` - in-memory SQLite setup

### Handler Integration Tests
**File:** `tests/handler_tests.rs`
- [ ] GET /forms/{slug} - form HTML
- [ ] GET /forms/{slug}/json - form JSON
- [ ] POST /forms/{slug} - valid submission
- [ ] POST /forms/{slug} - validation errors
- [ ] 404 for non-existent form
- [ ] Admin CRUD operations

### Workflow Tests
**File:** `tests/workflow_tests.rs`
- [ ] Complete form submission workflow
- [ ] Multi-step form workflow
- [ ] Validation error and retry
- [ ] Form CRUD lifecycle

### CLI Tests
**File:** `axum-sea-forms-cli/tests/cli_tests.rs`
- [ ] Using assert_cmd + insta snapshots
- [ ] All form commands
- [ ] All submission commands
- [ ] Error handling

---

## Phase 6: TUI Form Builder (v0.5.0) — PENDING

### Dependencies
- [ ] Add ratatui 0.29
- [ ] Add crossterm 0.28

### TUI Core
- [ ] `src/tui/mod.rs` - module entry
- [ ] `src/tui/app.rs` - app state and main loop
- [ ] `src/tui/ui.rs` - rendering functions

### TUI Components
- [ ] Text input component
- [ ] Select list component
- [ ] Modal component

### TUI Screens
- [ ] Main menu screen
- [ ] Form editor screen
- [ ] Step editor screen
- [ ] Field editor screen
- [ ] Options editor screen
- [ ] Validation rules editor screen

### CLI Integration
- [ ] `asf form create --interactive`
- [ ] `asf form edit <slug>`
- [ ] `asf tui` - main menu

---

## Summary

| Phase | Version | Status | Items |
|-------|---------|--------|-------|
| 1 Foundation | 0.1.0 | COMPLETE | Entities, validation, renderers, extractors, router |
| 2 CRUD & Testing | 0.2.0 | COMPLETE | FormBuilder, seeding, admin API, CLI |
| 2.1 Forms as Code | 0.2.1 | COMPLETE | Sync command, sync API |
| 2.2 API Polish | 0.2.2 | COMPLETE | ApiResponse, RequestId, serve command |
| 3.0 Multi-Step Core | 0.3.0 | COMPLETE | axum-sea-forms-core, conditions, multi-step validation |
| 3.1 Embedded SQLite | 0.3.1 | PENDING | Zero-config database |
| 3.2 WASM Client | 0.3.2 | PENDING | Browser-based form handling |
| 4 Survey/Quiz | 0.4.0 | PENDING | Scoring, results, analytics |
| 5 Integration Tests | - | PENDING | Handler, workflow, CLI tests |
| 6 TUI Builder | 0.5.0 | PENDING | Ratatui interactive builder |

---

## Changelog

### 2024-12-25 (v2)
- Reset tracker to reflect actual implementation state
- Marked Phases 1, 2, 2.1, 2.2, 3.0 as COMPLETE
- Added detailed Phase 3.2 (WASM Client) breakdown
- Reorganized phases for clarity

### 2024-12-25 (v1)
- Initial implementation tracker created

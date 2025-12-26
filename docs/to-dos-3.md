# anyform Implementation Tracker v3

> **Status**: Active
> **Started**: 2024-12-26
> **Last Updated**: 2024-12-26
> **PRD**: [axum-sea-forms-prd-0.4.0.md](/Users/rick/p/wordpuppi/docs/prd/libs/asf/axum-sea-forms-prd-0.4.0.md)

---

## Previous Phases (COMPLETE)

### Phase 1: Foundation (v0.1.0) — COMPLETE
- [x] SeaORM entities: Form, Step, Field, FieldOption, Submission, Result
- [x] Migrations for SQLite, PostgreSQL, MySQL
- [x] Schema types (ValueType, ValidationRules, UiOptions, FormSettings, FieldValue)
- [x] Validation engine (single-step)
- [x] Renderers (JSON, HTML, Tera)
- [x] Extractors and Router

### Phase 2: CRUD & Testing (v0.2.0) — COMPLETE
- [x] FormBuilder service (create, update, soft_delete, hard_delete, restore)
- [x] Builder input types with fluent API
- [x] Database seeding (contact, feedback, quiz forms)
- [x] Admin API endpoints
- [x] CLI commands (migrate, form, submissions, seed)

### Phase 2.1: Forms as Code (v0.2.1) — COMPLETE
- [x] `asf form sync --folder <path>` command
- [x] POST /admin/forms/sync endpoint

### Phase 2.2: API Polish (v0.2.2) — COMPLETE
- [x] ApiResponse envelope
- [x] RequestId extractor
- [x] `asf serve` command

### Phase 3.0: Multi-Step Core (v0.3.0) — COMPLETE
- [x] axum-sea-forms-core crate
- [x] ConditionRule and ConditionOp enums
- [x] Server-side multi-step validation with conditions
- [x] Step/field visibility evaluation

---

## Phase 4.0: Rename & Restructure (v0.4.0) — PENDING

### Crate Consolidation (1 main crate + 1 WASM crate)
- [ ] Merge `axum-sea-forms` + `axum-sea-forms-core` + `axum-sea-forms-cli` → `anyform`
- [ ] Create unified `anyform/src/lib.rs` (library exports)
- [ ] Create unified `anyform/src/main.rs` (CLI binary)
- [ ] Move condition.rs from core into main crate
- [ ] Update workspace Cargo.toml (only 2 members: anyform, anyform-client)
- [ ] Update all `use` statements

### Code Renames
- [ ] `FormsRouter` → `AnyFormRouter`
- [ ] Update struct/enum prefixes where applicable
- [ ] Update error types naming
- [ ] Update feature flag documentation

### Table Prefix Migration
- [ ] Create migration: `asf_` → `af_` tables
- [ ] Add `--rename-tables` flag to `anyform migrate`
- [ ] Test migration on SQLite
- [ ] Test migration on PostgreSQL
- [ ] Test migration on MySQL
- [ ] Document manual migration SQL

### API Route Changes
- [ ] Prefix all routes with `/api/`
- [ ] Update handler tests
- [ ] Update CLI to use new routes
- [ ] Document route changes in migration guide

### Documentation Updates
- [ ] Update README.md
- [ ] Update all doc comments
- [ ] Update CHANGELOG.md
- [ ] Update examples

---

## Phase 4.1: Distribution — PENDING

### GitHub Actions
- [ ] Create `.github/workflows/release.yml`
- [ ] Cross-compile for Linux amd64
- [ ] Cross-compile for Linux arm64
- [ ] Cross-compile for macOS amd64 (Intel)
- [ ] Cross-compile for macOS arm64 (Apple Silicon)
- [ ] Cross-compile for Windows amd64
- [ ] Upload release artifacts
- [ ] Generate checksums

### Install Scripts
- [ ] Create `install.sh` for Linux/macOS
- [ ] Host at anyform.dev (or GitHub Pages)
- [ ] Test on fresh Linux VM
- [ ] Test on fresh macOS

### Package Managers
- [ ] Create Homebrew formula
- [ ] Submit to Homebrew/homebrew-core (or tap)
- [ ] Create Scoop manifest
- [ ] Test Scoop installation

### Docker
- [ ] Create `Dockerfile`
- [ ] Create `docker-compose.yml` example
- [ ] Push to Docker Hub (anyform/anyform)
- [ ] Push to GitHub Container Registry
- [ ] Document volume mounts
- [ ] Test with PostgreSQL compose

### Landing Page (Optional)
- [ ] Create anyform.dev domain
- [ ] Host documentation
- [ ] Installation instructions
- [ ] Quick start guide

---

## Phase 4.2: Zero-Config SQLite — PENDING

### `anyform init` Command
- [ ] Implement `init` subcommand
- [ ] Create parent directories if needed
- [ ] Create SQLite database file
- [ ] Run migrations automatically
- [ ] `--path <PATH>` flag (default: ./anyform.db)
- [ ] `--database <URL>` flag for external DB
- [ ] `--force` flag to overwrite
- [ ] `--seed` flag to add example forms
- [ ] Unit tests for init command

### `anyform serve` Enhancements
- [ ] Auto-create database if not exists
- [ ] Auto-run pending migrations
- [ ] `--no-admin` flag to disable admin routes
- [ ] `--cors <ORIGIN>` flag for CORS
- [ ] Graceful shutdown handling
- [ ] Health check endpoint (`/health`)

### Database Auto-Detection
- [ ] Check `DATABASE_URL` env var first
- [ ] Fall back to `--database` flag
- [ ] Fall back to `./anyform.db`
- [ ] Print database location on startup
- [ ] Warn if database doesn't exist (suggest `init`)

### Tests
- [ ] Test init creates database
- [ ] Test init runs migrations
- [ ] Test serve auto-migrates
- [ ] Test serve with embedded SQLite
- [ ] Test serve with external Postgres
- [ ] Integration test: init → serve → submit form

---

## Phase 4.3: WASM Client (anyform-client) — PENDING

### Crate Setup
- [ ] Create `anyform-client/Cargo.toml`
- [ ] Add wasm-bindgen, wasm-bindgen-futures dependencies
- [ ] Add web-sys with fetch features
- [ ] Add js-sys, serde, serde_json, serde-wasm-bindgen
- [ ] Add uuid with `js` feature
- [ ] Add regex for validation
- [ ] Create `src/lib.rs` with wasm-bindgen exports

### Schema Types (duplicated for WASM - no server deps)
- [ ] Define `FormJson` struct
- [ ] Define `StepJson` struct
- [ ] Define `FieldJson` struct
- [ ] Define `FieldOptionJson` struct
- [ ] Define `ConditionRule` and `ConditionOp` (copy from main crate)
- [ ] Define `ValidationRules` struct
- [ ] Implement Serialize/Deserialize

### FormState Struct
- [ ] Create `src/form_state.rs`
- [ ] `values: HashMap<String, serde_json::Value>`
- [ ] `errors: HashMap<String, Vec<String>>`
- [ ] `touched: HashSet<String>`
- [ ] `current_step_id: Option<Uuid>`
- [ ] `set_value(field, value)` method
- [ ] `get_value(field)` method
- [ ] `get_values()` method (all values)
- [ ] `mark_touched(field)` method
- [ ] `is_touched(field)` method
- [ ] Unit tests for state management

### Condition Evaluation
- [ ] Create `src/condition.rs`
- [ ] Implement `visible_steps()` method
- [ ] Implement `visible_fields(step_id)` method
- [ ] Implement `is_step_visible(step_id)` method
- [ ] Implement `is_field_visible(field_name)` method
- [ ] Unit tests for condition evaluation

### Client-Side Validation
- [ ] Create `src/validation.rs`
- [ ] Port `validate_required()` from server
- [ ] Port `validate_email()` from server
- [ ] Port `validate_url()` from server
- [ ] Port `validate_tel()` from server
- [ ] Port `validate_min_length()` from server
- [ ] Port `validate_max_length()` from server
- [ ] Port `validate_pattern()` from server
- [ ] Port `validate_min_value()` from server
- [ ] Port `validate_max_value()` from server
- [ ] Port `validate_min_selections()` from server
- [ ] Port `validate_max_selections()` from server
- [ ] Implement `validate_field(field)` method
- [ ] Implement `validate_step(step_id)` method
- [ ] Implement `validate_all()` method
- [ ] Implement `is_valid()` method
- [ ] Implement `get_errors(field)` method
- [ ] Unit tests for validation

### Multi-Step Navigation
- [ ] Implement `current_step()` method
- [ ] Implement `go_to_step(step_id)` method
- [ ] Implement `next_step()` method (skip hidden)
- [ ] Implement `prev_step()` method (skip hidden)
- [ ] Implement `can_go_next()` method
- [ ] Implement `can_go_prev()` method
- [ ] Implement `progress()` method → [current, total]
- [ ] Unit tests for navigation

### API Client
- [ ] Create `src/api.rs`
- [ ] Implement fetch wrapper using web-sys
- [ ] Implement `fetch_form(slug)` method
- [ ] Implement `submit_form(slug, data)` method
- [ ] Handle ApiResponse envelope parsing
- [ ] Error handling (network, validation, server)
- [ ] Unit tests with mock responses

### FormClient Struct
- [ ] Create `src/form_client.rs`
- [ ] `FormClient::new(base_url)` constructor
- [ ] `fetch_form(slug) -> FormState` method
- [ ] `submit_form(slug, data)` method
- [ ] wasm-bindgen exports

### JavaScript Bindings
- [ ] Export FormClient to JS
- [ ] Export FormState methods to JS
- [ ] Handle JsValue conversions
- [ ] Test from JavaScript

### WASM Build & Test
- [ ] Install wasm-pack: `cargo install wasm-pack`
- [ ] Build: `wasm-pack build --target web --release`
- [ ] Verify pkg/ output (*.js, *.wasm, *.d.ts, package.json)
- [ ] Test WASM output size (target: <500KB)
- [ ] Test in browser environment
- [ ] Add wasm-bindgen-test tests

### Hydration Mode (auto-magic forms)
- [ ] Create `src/hydrate.rs` - entry points
- [ ] Implement `hydrate_all()` - find and hydrate all forms on page
- [ ] Implement `hydrate(slug)` - hydrate specific form by slug
- [ ] Create `src/dom.rs` - DOM query helpers
- [ ] Implement `query(selector)` - find single element
- [ ] Implement `query_all(selector)` - find all elements
- [ ] Implement `set_visible(element, bool)` - update visibility attribute
- [ ] Create `src/form_controller.rs` - AfForm struct
- [ ] Parse `data-af-form` attribute for form slug
- [ ] Parse `data-af-step` for step discovery
- [ ] Parse `data-af-field` for field discovery
- [ ] Parse `data-af-condition` JSON for conditions
- [ ] Parse `data-af-validation` JSON for validation rules
- [ ] Bind input/change events to fields
- [ ] Bind click events to `.af-prev`, `.af-next` buttons
- [ ] Bind submit event to form
- [ ] Update `data-af-visible` on condition changes
- [ ] Show/hide `.af-submit` on last step
- [ ] Display validation errors in `.af-error-message`
- [ ] Add/remove `.af-error` class on validation
- [ ] Export `hydrate` and `hydrate_all` via wasm-bindgen
- [ ] Unit tests for form controller
- [ ] Integration test: hydrate multi-step form

### HTML Renderer Updates
- [ ] Update CSS class prefix: `asf-` → `af-`
- [ ] Update data attribute prefix: `data-asf-` → `data-af-`
- [ ] Update inline CSS variable names
- [ ] Update WASM loader script path
- [ ] Add `data-af-validation` attribute output
- [ ] Test rendered HTML with WASM hydration

---

## Phase 4.4: npm Package (anyform-js) — PENDING

### Package Setup
- [ ] Create `anyform-js/package.json`
- [ ] Set name: `anyform-js`
- [ ] Set version: `0.4.0`
- [ ] Add build scripts
- [ ] Add type declarations path

### TypeScript Bindings
- [ ] Create `src/index.ts`
- [ ] Export FormClient class
- [ ] Export FormState class
- [ ] Export type definitions
- [ ] Create `anyform.d.ts` type definitions

### Build Pipeline
- [ ] Build WASM with wasm-pack
- [ ] Copy WASM to `wasm/` directory
- [ ] Bundle TypeScript
- [ ] Generate source maps
- [ ] Test import in Node.js
- [ ] Test import in browser

### Documentation
- [ ] README with installation
- [ ] Usage examples
- [ ] API reference
- [ ] TypeScript example
- [ ] React example snippet
- [ ] Vue example snippet

### Publishing
- [ ] Create npm account/org
- [ ] Test publish with `--dry-run`
- [ ] Publish to npm

---

## Phase 5: Survey & Quiz Polish (v0.5.0) — PENDING

### Scoring Refinements
- [ ] Weighted scoring support (field-level weights)
- [ ] Partial credit for multi-select
- [ ] Score normalization (percentage)
- [ ] Custom scoring functions

### Result Buckets
- [ ] Result bucket management API
- [ ] Auto-select result based on score range
- [ ] Custom result messages
- [ ] Result bucket CRUD endpoints

### Analytics Helpers
- [ ] NPS calculation helper
- [ ] Rating average helper
- [ ] Response distribution helper
- [ ] Export analytics as JSON

---

## Phase 6: Integration Tests — PENDING

### Test Infrastructure
- [ ] Create `anyform/tests/` directory
- [ ] Create `tests/common/mod.rs` utilities
- [ ] Create `tests/common/fixtures.rs` test data
- [ ] Create `tests/common/db.rs` in-memory SQLite

### Handler Tests
**File:** `tests/handler_tests.rs`
- [ ] GET /api/forms/{slug} - returns form JSON
- [ ] GET /api/forms/{slug}.html - returns HTML
- [ ] POST /api/forms/{slug} - valid submission
- [ ] POST /api/forms/{slug} - validation errors
- [ ] 404 for non-existent form
- [ ] Admin CRUD operations

### Workflow Tests
**File:** `tests/workflow_tests.rs`
- [ ] Complete form submission workflow
- [ ] Multi-step form with conditions
- [ ] Validation error and retry
- [ ] Form CRUD lifecycle

### CLI Tests
**File:** `anyform/tests/cli_tests.rs`
- [ ] Using assert_cmd + insta snapshots
- [ ] `anyform init` creates database
- [ ] `anyform form list` output
- [ ] `anyform form create` creates form
- [ ] `anyform serve` starts server
- [ ] Error handling tests

---

## Phase 7: TUI Builder (v0.6.0) — PENDING

### Dependencies
- [ ] Add ratatui 0.29
- [ ] Add crossterm 0.28

### TUI Core
- [ ] Create `src/tui/mod.rs`
- [ ] Create `src/tui/app.rs` (state + main loop)
- [ ] Create `src/tui/ui.rs` (rendering)

### TUI Components
- [ ] Text input component
- [ ] Select list component
- [ ] Modal component
- [ ] Scrollable list component

### TUI Screens
- [ ] Main menu screen
- [ ] Form editor screen
- [ ] Step editor screen
- [ ] Field editor screen
- [ ] Options editor screen
- [ ] Validation rules editor

### CLI Integration
- [ ] `anyform form create --interactive`
- [ ] `anyform form edit <slug>`
- [ ] `anyform tui` (main menu)

---

## Summary

| Phase | Version | Status | Focus |
|-------|---------|--------|-------|
| 1-3.0 | 0.1.0-0.3.0 | COMPLETE | Core foundation |
| 4.0 Rename | 0.4.0 | PENDING | Merge 3 crates → 1 (`anyform`) |
| 4.1 Distribution | 0.4.0 | PENDING | Binaries, Docker, brew, npm |
| 4.2 Zero-Config | 0.4.0 | PENDING | Embedded SQLite |
| 4.3 WASM Client | 0.4.0 | PENDING | `anyform-client` (standalone WASM) |
| 4.4 npm Package | 0.4.0 | PENDING | `anyform-js` |
| 5 Survey/Quiz | 0.5.0 | PENDING | Scoring, results |
| 6 Integration Tests | — | PENDING | Handler, workflow, CLI tests |
| 7 TUI Builder | 0.6.0 | PENDING | Interactive form creation |

### Release Artifacts (from 2 Rust crates)

| Artifact | Source | Distribution |
|----------|--------|--------------|
| `anyform` binary | `anyform` crate | GitHub Releases, brew, curl, scoop |
| `anyform` library | `anyform` crate | crates.io |
| `anyform-client` | `anyform-client` crate | crates.io (WASM) |
| `anyform-js` | built from `anyform-client` | npm |
| Docker image | `anyform` binary | Docker Hub |

---

## Changelog

### 2024-12-26 (v3.1)
- Simplified to 2-crate structure (anyform + anyform-client)
- anyform-client is standalone (no core dependency, types duplicated)
- Added wasm-pack build details
- Added release artifacts table

### 2024-12-26 (v3)
- Created new tracker for anyform 0.4.0 product launch
- Reorganized phases: 4.0 (rename), 4.1-4.4 (features), 5+ (future)
- Added detailed breakdown for WASM client implementation
- Added distribution and packaging tasks
- Changed table prefix from `asf_` to `af_`

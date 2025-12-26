# axum-sea-forms v0.2.0 Implementation Tracker

> **Status**: Active
> **Started**: 2024-12-25
> **PRD**: [axum-sea-forms-prd-0.2.0.md](/Users/rick/p/wordpuppi/docs/prd/libs/asf/axum-sea-forms-prd-0.2.0.md)

---

## Phase 2: Test Infrastructure

### 2.1 Test Setup
- [ ] Create `axum-sea-forms/tests/common/mod.rs` - Shared test utilities
- [ ] Create `axum-sea-forms/tests/common/fixtures.rs` - Test data builders
- [ ] Create `axum-sea-forms/tests/common/db.rs` - In-memory SQLite setup

### 2.2 Dependencies
- [ ] Add to `axum-sea-forms/Cargo.toml`:
  ```toml
  [dev-dependencies]
  insta = { version = "1.34", features = ["yaml", "json"] }
  tempfile = "3"
  tower = { version = "0.5", features = ["util"] }
  http-body-util = "0.1"
  ```

---

## Phase 1.3: Form Builder Service

- [ ] Create `axum-sea-forms/src/services/mod.rs` - Services module
- [ ] Create `axum-sea-forms/src/services/form_builder.rs` - Form creation service
- [ ] Implement `CreateFormInput` struct
- [ ] Implement `CreateStepInput` struct
- [ ] Implement `CreateFieldInput` struct
- [ ] Implement `CreateOptionInput` struct
- [ ] Implement `FormBuilder::new()` with fluent API
- [ ] Export from `axum-sea-forms/src/lib.rs`

---

## Phase 1.1: Form Create (Fix CLI Stub + API)

### Entity Methods
- [ ] Add `Form::create(db, input) -> Result<Model>` in `entities/form.rs`
- [ ] Add transaction support for nested creation (form + steps + fields + options)

### API Endpoints
- [ ] Add `POST /admin/forms` handler in `handlers/mod.rs`
- [ ] Register route in `router.rs`

### CLI Commands
- [ ] Implement `asf form create --file <path>` in `commands/form.rs` (currently a stub)
- [ ] Parse JSON file into `CreateFormInput`
- [ ] Call `FormBuilder` service

---

## Phase 1.4: Database Seeding

### Seed Module
- [ ] Create `axum-sea-forms/src/seed.rs`
- [ ] Implement `seed_example_forms(db) -> Result<()>`

### Example Forms to Create
- [ ] **Contact Form** - Basic fields
  - [ ] name (text, required)
  - [ ] email (email, required)
  - [ ] phone (tel, optional)
  - [ ] message (textarea, required)
  - [ ] preferred_contact (radio: email/phone/either)

- [ ] **Feedback Survey** - Selection fields
  - [ ] satisfaction (rating 1-5)
  - [ ] recommend (nps 0-10)
  - [ ] features_used (multi_select)
  - [ ] improvements (textarea)

- [ ] **Quiz Example** - Scoring fields
  - [ ] question1 (radio with correct_answer + points)
  - [ ] question2 (select with correct_answer + points)
  - [ ] results mapping (beginner/intermediate/expert)

### CLI Commands
- [ ] Create `axum-sea-forms-cli/src/commands/seed.rs`
- [ ] Add `asf seed` command to main.rs
- [ ] Add `asf seed --contact-only` option
- [ ] Add `asf seed --clear` option

---

## Phase 3.1: Validation Tests (~40 tests)

**File:** `axum-sea-forms/tests/validation_tests.rs`

- [ ] Email validation edge cases
- [ ] URL validation (http/https only)
- [ ] Phone validation (various formats)
- [ ] Date/DateTime/Time format validation
- [ ] Required field validation
- [ ] String length validation (min/max)
- [ ] Numeric range validation (min/max)
- [ ] Pattern/regex validation
- [ ] Array selection validation (min/max selections)
- [ ] Full submission validation with multiple errors
- [ ] Add insta snapshots for validation error messages

---

## Phase 3.2: Renderer Tests (~50 tests)

### JSON Renderer Tests (~15 tests)
**File:** `axum-sea-forms/tests/render_json_tests.rs`
- [ ] Form structure serialization
- [ ] Field options serialization
- [ ] Validation rules in output
- [ ] Multi-step form serialization
- [ ] Insta snapshots for JSON output

### HTML Renderer Tests (~25 tests)
**File:** `axum-sea-forms/tests/render_html_tests.rs`
- [ ] Form element attributes (action, method, enctype)
- [ ] Input types for each ValueType
- [ ] Select/radio/checkbox rendering
- [ ] Textarea with rows
- [ ] Hidden fields
- [ ] Display-only fields (heading, paragraph)
- [ ] Error message display
- [ ] Value pre-population
- [ ] HTML escaping (XSS prevention)
- [ ] Insta snapshots for HTML output

### Tera Renderer Tests (~10 tests)
**File:** `axum-sea-forms/tests/render_tera_tests.rs`
- [ ] Context structure
- [ ] Multipart detection
- [ ] Values and errors insertion

---

## Phase 7: Ratatui TUI (~25 tests)

### 7.1 Dependencies
- [ ] Add to `axum-sea-forms-cli/Cargo.toml`:
  ```toml
  [dependencies]
  ratatui = "0.29"
  crossterm = "0.28"

  [dev-dependencies]
  ratatui = { version = "0.29", features = ["unstable-backend-writer"] }
  ```

### 7.2 Core TUI Structure
- [ ] Create `axum-sea-forms-cli/src/tui/mod.rs`
- [ ] Create `axum-sea-forms-cli/src/tui/app.rs` - App state and main loop
- [ ] Create `axum-sea-forms-cli/src/tui/ui.rs` - Rendering functions

### 7.3 Reusable Components
- [ ] Create `axum-sea-forms-cli/src/tui/components/mod.rs`
- [ ] Create `axum-sea-forms-cli/src/tui/components/text_input.rs`
- [ ] Create `axum-sea-forms-cli/src/tui/components/select_list.rs`
- [ ] Create `axum-sea-forms-cli/src/tui/components/modal.rs`

### 7.4 Screen Modules
- [ ] Create `axum-sea-forms-cli/src/tui/screens/mod.rs`
- [ ] Create `axum-sea-forms-cli/src/tui/screens/main_menu.rs`
- [ ] Create `axum-sea-forms-cli/src/tui/screens/form_editor.rs`
- [ ] Create `axum-sea-forms-cli/src/tui/screens/step_editor.rs`
- [ ] Create `axum-sea-forms-cli/src/tui/screens/field_editor.rs`
- [ ] Create `axum-sea-forms-cli/src/tui/screens/options_editor.rs`
- [ ] Create `axum-sea-forms-cli/src/tui/screens/validation_editor.rs`

### 7.5 CLI Integration
- [ ] Add `asf form create --interactive` command
- [ ] Add `asf form edit <slug>` command
- [ ] Add `asf tui` command for main menu

### 7.6 TUI Tests
**File:** `axum-sea-forms-cli/tests/tui_tests.rs`
- [ ] Component tests (text_input, select_list, etc.)
- [ ] Navigation tests (keyboard shortcuts)
- [ ] Workflow tests (create form flow, edit form flow)
- [ ] Edge case tests (empty form, duplicate names, scrolling)

---

## Phase 1.1 (continued): Form Update/Delete

### Entity Methods
- [ ] Add `Form::update(db, id, input) -> Result<Model>` in `entities/form.rs`
- [ ] Add `Form::soft_delete(db, id) -> Result<()>` in `entities/form.rs`

### API Endpoints
- [ ] Add `PUT /admin/forms/{id}` handler
- [ ] Add `DELETE /admin/forms/{id}` handler
- [ ] Register routes in `router.rs`

### CLI Commands
- [ ] Add `asf form update <slug> --file <path>` command
- [ ] Add `asf form delete <slug>` command

---

## Phase 1.2: Submission CRUD

### Entity Methods
- [ ] Add `Submission::soft_delete(db, id) -> Result<()>` in `entities/submission.rs`

### API Endpoints
- [ ] Add `GET /admin/forms/{id}/submissions/{sub_id}` handler
- [ ] Add `DELETE /admin/forms/{id}/submissions/{sub_id}` handler

### CLI Commands
- [ ] Add `asf submissions show <id>` command
- [ ] Add `asf submissions delete <id>` command

---

## Phase 4: Integration Tests (~40 tests)

### Handler Integration Tests
**File:** `axum-sea-forms/tests/handler_tests.rs`

GET handlers:
- [ ] `GET /forms/{slug}` - form HTML
- [ ] `GET /forms/{slug}/json` - form JSON
- [ ] 404 for non-existent form
- [ ] 410 for deleted form

POST handlers:
- [ ] `POST /forms/{slug}` - valid submission
- [ ] `POST /forms/{slug}` - validation errors
- [ ] `POST /forms/{slug}/submit` - redirect on success
- [ ] `POST /forms/{slug}/submit` - re-render on error

Admin handlers:
- [ ] `GET /admin/forms` - list forms
- [ ] `GET /admin/forms/{id}` - get form
- [ ] `POST /admin/forms` - create form
- [ ] `PUT /admin/forms/{id}` - update form
- [ ] `DELETE /admin/forms/{id}` - delete form
- [ ] `GET /admin/forms/{id}/submissions` - list submissions

### Workflow Tests
**File:** `axum-sea-forms/tests/workflow_tests.rs`
- [ ] Complete form submission workflow
- [ ] Validation error and retry workflow
- [ ] Form CRUD lifecycle
- [ ] Submission listing and export

---

## Phase 5: CLI Tests (~20 tests)

**File:** `axum-sea-forms-cli/tests/cli_tests.rs`

Using `assert_cmd` and insta snapshot testing:
- [ ] `asf migrate` commands
- [ ] `asf form list` (empty and populated)
- [ ] `asf form show <slug>`
- [ ] `asf form export <slug> --json`
- [ ] `asf form render <slug>`
- [ ] `asf form create --file <path>`
- [ ] `asf form delete <slug>`
- [ ] `asf submissions list <form-slug>`
- [ ] `asf submissions export <form-slug> --format csv`
- [ ] Error handling for invalid inputs

---

## Phase 6: Error Handling Tests (~15 tests)

**File:** `axum-sea-forms/tests/error_tests.rs`

- [ ] FormError variant creation
- [ ] Status code mapping
- [ ] Error code strings
- [ ] JSON response format
- [ ] ValidationErrors collection and retrieval

---

## (DEFERRED) Phase 8: WordPuppi CLI Integration

> **Status**: Deferred until axum-sea-forms is stable
> **Tracked in**: wordpuppi-prd-0.6.2.md Phase 1.4

### Make axum-sea-forms-cli a library + binary crate
- [ ] Create `axum-sea-forms-cli/src/lib.rs` with public exports
- [ ] Export `FormAction` enum publicly
- [ ] Export `SubmissionAction` enum publicly
- [ ] Export handler functions publicly
- [ ] Add `[lib]` section to Cargo.toml

### wpp-cli Integration
- [ ] Add axum-sea-forms-cli dependency to wpp-cli
- [ ] Add `Forms` subcommand enum
- [ ] Add `Submissions` subcommand enum
- [ ] Wire up command handlers
- [ ] Share database connection

### Commands to add to wpp-cli
- [ ] `wordpuppi forms list`
- [ ] `wordpuppi forms show <slug>`
- [ ] `wordpuppi forms create --interactive`
- [ ] `wordpuppi forms create --file <path>`
- [ ] `wordpuppi forms edit <slug>`
- [ ] `wordpuppi forms delete <slug>`
- [ ] `wordpuppi forms render <slug>`
- [ ] `wordpuppi forms export <slug> --json`
- [ ] `wordpuppi submissions list <form-slug>`
- [ ] `wordpuppi submissions export <form-slug> --format csv`

---

## Summary

| Phase | Files | Tests | Status |
|-------|-------|-------|--------|
| 2 Test Infrastructure | 3 | - | Pending |
| 1.3 Form Builder | 2 | - | Pending |
| 1.1 Form Create | 4 | 5 | Pending |
| 1.4 Database Seeding | 2 | - | Pending |
| 3.1 Validation Tests | 1 | 40 | Pending |
| 3.2 Renderer Tests | 3 | 50 | Pending |
| 7 Ratatui TUI | 12 | 25 | Pending |
| 1.1 Form Update/Delete | 2 | 5 | Pending |
| 1.2 Submission CRUD | 2 | 5 | Pending |
| 4 Integration Tests | 2 | 40 | Pending |
| 5 CLI Tests | 1 | 20 | Pending |
| 6 Error Tests | 1 | 15 | Pending |
| **asf Subtotal** | **35** | **~205** | |
| (Deferred) 8 wpp-cli | 3 | 5 | Deferred |
| **Grand Total** | **38** | **~210** | |

---

## Changelog

### 2024-12-25
- Created initial implementation tracker from plan

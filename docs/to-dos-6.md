# anyform Phase 6: Integration Tests

> **Status**: In Progress (HTTP complete, CLI pending)
> **PRD Reference**: [axum-sea-forms-prd-0.4.0.md](/Users/rick/p/wordpuppi/docs/prd/libs/asf/axum-sea-forms-prd-0.4.0.md) Section 13
> **Depends On**: Phase 4.x (complete)
> **Excludes**: Quiz/Survey/Scoring tests (Phase 5)

---

## Progress Summary

**Completed 2025-12-26:**
- [x] Test Infrastructure (TestApp, TestCli, TestResponse)
- [x] Public Handler Tests (25 tests)
- [x] Admin Handler Tests (32 tests)
- [x] Health Check Endpoint + Tests (3 tests)
- [x] Error Handling Tests (16 tests)
- [x] Feature Flag Tests (11 tests)
- [x] Workflow Tests (10 tests)

**Test Count:**
- Before Phase 6: 111 tests
- After Phase 6: 224 tests (+113 new)

**Remaining (deferred):**
- CLI Command Tests (~26 tests)

---

## Overview

Phase 6 adds comprehensive integration tests for HTTP handlers, CLI commands, and end-to-end workflows. Currently, tests cover the service layer well but HTTP endpoints and CLI commands have zero test coverage.

**Current State:**
- 224 tests (unit + service + integration)
- Strong FormBuilder/validation/render coverage
- Full HTTP handler test coverage
- CLI command tests pending

**Goal:**
- Full HTTP endpoint coverage (public + admin) ✓
- CLI command coverage (pending)
- Error handling verification ✓
- Response format validation ✓

---

## 1. Test Infrastructure ✓

### 1.1 HTTP Test Utilities ✓
**File:** `anyform/tests/common/app.rs`

- [x] Create `TestApp` struct wrapping axum Router + TestDb
- [x] Implement `TestApp::new()` - builds router with test database
- [x] Implement `TestApp::with_admin()` - enables admin routes
- [x] Add helper methods:
  - `get(&self, path) -> Response`
  - `post_json(&self, path, body) -> Response`
  - `post_form(&self, path, data) -> Response`
  - `put_json(&self, path, body) -> Response`
  - `delete(&self, path) -> Response`
  - `send_raw(&self, request) -> Response`
- [x] Add response assertion helpers:
  - `assert_status(response, StatusCode)`
  - `json<T>(&self) -> T`
  - `assert_body_contains(text)`
  - `assert_api_success()`
  - `assert_api_error(code)`
  - `assert_content_type(expected)`

### 1.2 Update Common Module ✓
**File:** `anyform/tests/common/mod.rs`

- [x] Add `pub mod app;`
- [x] Add `pub mod cli;`
- [x] Re-export `TestApp`, `TestCli`

---

## 2. Public Handler Tests ✓

### 2.1 Form JSON Endpoint ✓
**File:** `anyform/tests/handler_public_tests.rs`

**GET /api/forms/{slug}/json**
- [x] `test_get_form_json_success` — returns form schema
- [x] `test_get_form_json_not_found` — returns 404 for unknown slug
- [x] `test_get_form_json_deleted_form_returns_404` — returns 404 for deleted form
- [x] `test_get_form_json_includes_steps_and_fields` — response has steps/fields
- [x] `test_get_form_json_includes_field_details` — fields have all properties

### 2.2 Form HTML Endpoint
**GET /api/forms/{slug}**
- [ ] `test_get_form_html_success` — returns HTML with form tag
- [ ] `test_get_form_html_not_found` — returns 404
- [ ] `test_get_form_html_contains_fields` — HTML has input elements
- [ ] `test_get_form_html_custom_action_url` — form action attribute uses custom URL

### 2.3 Form Submission Endpoint
**POST /api/forms/{slug}**
- [ ] `test_submit_form_success` — valid data returns 201
- [ ] `test_submit_form_creates_submission` — submission saved to DB
- [ ] `test_submit_form_not_found` — unknown slug returns 404
- [ ] `test_submit_form_deleted` — deleted form returns error
- [ ] `test_submit_form_validation_error` — invalid data returns 400 with errors
- [ ] `test_submit_form_missing_required` — missing required field returns error
- [ ] `test_submit_form_invalid_email` — bad email format returns error
- [ ] `test_submit_form_response_format` — response has success message

### 2.4 Form Submit with Redirect
**POST /api/forms/{slug}/submit**
- [ ] `test_submit_redirect_success` — valid data redirects to success page
- [ ] `test_submit_redirect_validation_error` — re-renders form with errors
- [ ] `test_submit_redirect_custom_redirect` — uses custom redirect URL from settings

### 2.5 Success Page
**GET /api/forms/{slug}/success**
- [ ] `test_success_page_renders` — returns HTML success page
- [ ] `test_success_page_custom_message` — uses custom success message

---

## 3. Admin Handler Tests

### 3.1 List Forms
**File:** `anyform/tests/handlers_admin_tests.rs`

**GET /api/admin/forms**
- [ ] `test_list_forms_empty` — returns empty array when no forms
- [ ] `test_list_forms_multiple` — returns all active forms
- [ ] `test_list_forms_excludes_deleted` — deleted forms not in list
- [ ] `test_list_forms_response_format` — has id, name, slug, timestamps

### 3.2 Get Form by ID
**GET /api/admin/forms/{id}**
- [ ] `test_get_form_by_id_success` — returns form details
- [ ] `test_get_form_by_id_not_found` — unknown ID returns 404
- [ ] `test_get_form_by_id_invalid_uuid` — bad UUID format returns 400

### 3.3 Create Form
**POST /api/admin/forms**
- [ ] `test_create_form_success` — creates form, returns 201
- [ ] `test_create_form_with_fields` — creates form with steps/fields
- [ ] `test_create_form_with_settings` — settings persisted correctly
- [ ] `test_create_form_with_action_url` — action_url in settings works
- [ ] `test_create_form_duplicate_slug` — duplicate slug returns error
- [ ] `test_create_form_invalid_json` — malformed JSON returns 400

### 3.4 Update Form
**PUT /api/admin/forms/{id}**
- [ ] `test_update_form_success` — updates form, returns 200
- [ ] `test_update_form_not_found` — unknown ID returns 404
- [ ] `test_update_form_changes_name` — name change persisted
- [ ] `test_update_form_updates_fields` — field changes applied
- [ ] `test_update_form_updates_action_url` — action_url change persisted

### 3.5 Delete Form
**DELETE /api/admin/forms/{id}**
- [ ] `test_delete_form_success` — soft deletes form
- [ ] `test_delete_form_not_found` — unknown ID returns 404
- [ ] `test_delete_form_is_soft` — form still in DB with deleted_at

### 3.6 List Submissions
**GET /api/admin/forms/{id}/submissions**
- [ ] `test_list_submissions_empty` — returns empty when no submissions
- [ ] `test_list_submissions_multiple` — returns all submissions
- [ ] `test_list_submissions_for_form` — only returns submissions for that form
- [ ] `test_list_submissions_form_not_found` — unknown form ID returns 404

### 3.7 Get Submission
**GET /api/admin/forms/{form_id}/submissions/{sub_id}**
- [ ] `test_get_submission_success` — returns submission data
- [ ] `test_get_submission_not_found` — unknown sub_id returns 404
- [ ] `test_get_submission_wrong_form` — submission from different form returns 404

### 3.8 Delete Submission
**DELETE /api/admin/forms/{form_id}/submissions/{sub_id}**
- [ ] `test_delete_submission_success` — soft deletes submission
- [ ] `test_delete_submission_not_found` — unknown ID returns 404

### 3.9 Sync Forms
**POST /api/admin/forms/sync**
- [ ] `test_sync_creates_new_forms` — creates forms that don't exist
- [ ] `test_sync_updates_existing` — updates forms that exist (by slug)
- [ ] `test_sync_mixed_create_update` — handles both in one request
- [ ] `test_sync_returns_counts` — response has created/updated counts
- [ ] `test_sync_empty_array` — empty input works

---

## 4. Admin Feature Flag Tests

**File:** `anyform/tests/handlers_admin_flag_tests.rs`

- [ ] `test_admin_routes_disabled_by_default` — /api/admin/* returns 404 when admin disabled
- [ ] `test_admin_routes_enabled` — /api/admin/* works when admin enabled
- [ ] `test_public_routes_always_available` — /api/forms/* works regardless of admin flag

---

## 5. Error Handling Tests

**File:** `anyform/tests/handlers_error_tests.rs`

### 5.1 Not Found Errors
- [ ] `test_404_unknown_route` — random path returns 404
- [ ] `test_404_form_not_found` — unknown slug returns proper error
- [ ] `test_404_submission_not_found` — unknown submission ID

### 5.2 Validation Errors
- [ ] `test_400_invalid_json` — malformed JSON body
- [ ] `test_400_missing_required_fields` — required form fields missing
- [ ] `test_400_invalid_field_format` — email/url validation failures
- [ ] `test_400_invalid_uuid_path` — bad UUID in path parameter

### 5.3 Response Format
- [ ] `test_error_response_format` — errors have success:false, error object
- [ ] `test_error_has_code` — error responses include error code
- [ ] `test_error_has_message` — error responses include message

---

## 6. Health Check Tests

**File:** `anyform/tests/health_check_tests.rs`

- [ ] `test_health_check_returns_ok` — GET /health returns 200
- [ ] `test_health_check_body` — response body is "OK"

---

## 7. CLI Command Tests

### 7.1 CLI Test Infrastructure
**File:** `anyform/tests/common/cli.rs`

- [ ] Create `CliRunner` struct for executing CLI commands
- [ ] Implement `run(&self, args: &[&str]) -> Output`
- [ ] Add assertion helpers for stdout/stderr/exit code

### 7.2 Form CLI Tests
**File:** `anyform/tests/cli_form_tests.rs`

**anyform form list**
- [ ] `test_cli_form_list_empty` — shows "No forms found"
- [ ] `test_cli_form_list_shows_forms` — displays table with forms

**anyform form show**
- [ ] `test_cli_form_show_success` — outputs JSON schema
- [ ] `test_cli_form_show_not_found` — error for unknown slug

**anyform form create**
- [ ] `test_cli_form_create_success` — creates form from JSON file
- [ ] `test_cli_form_create_invalid_json` — error for bad JSON

**anyform form update**
- [ ] `test_cli_form_update_success` — updates existing form
- [ ] `test_cli_form_update_not_found` — error for unknown slug

**anyform form delete**
- [ ] `test_cli_form_delete_success` — deletes form
- [ ] `test_cli_form_delete_not_found` — error for unknown slug

**anyform form export**
- [ ] `test_cli_form_export_json` — exports form as JSON

**anyform form render**
- [ ] `test_cli_form_render_html` — outputs HTML

**anyform form sync**
- [ ] `test_cli_form_sync_creates` — creates forms from folder
- [ ] `test_cli_form_sync_updates` — updates existing forms
- [ ] `test_cli_form_sync_reports_counts` — shows created/updated counts

**anyform form set-action**
- [ ] `test_cli_set_action_url` — sets custom action URL
- [ ] `test_cli_set_action_method` — sets HTTP method
- [ ] `test_cli_set_action_clear` — empty string clears URL
- [ ] `test_cli_set_action_invalid_url` — rejects javascript: URLs

### 7.3 Submission CLI Tests
**File:** `anyform/tests/cli_submission_tests.rs`

**anyform submissions list**
- [ ] `test_cli_submissions_list` — shows submissions for form
- [ ] `test_cli_submissions_list_limit` — respects --limit flag

**anyform submissions show**
- [ ] `test_cli_submissions_show` — displays submission JSON

**anyform submissions delete**
- [ ] `test_cli_submissions_delete` — soft deletes submission

**anyform submissions export**
- [ ] `test_cli_submissions_export_csv` — exports as CSV
- [ ] `test_cli_submissions_export_json` — exports as JSON

### 7.4 Database CLI Tests
**File:** `anyform/tests/cli_database_tests.rs`

**anyform init**
- [ ] `test_cli_init_creates_sqlite` — creates SQLite file
- [ ] `test_cli_init_runs_migrations` — tables created
- [ ] `test_cli_init_seed_flag` — --seed populates example forms
- [ ] `test_cli_init_force_flag` — --force overwrites existing

**anyform migrate**
- [ ] `test_cli_migrate_status` — shows migration status
- [ ] `test_cli_migrate_up` — runs pending migrations

---

## 8. Workflow Tests

**File:** `anyform/tests/workflow_tests.rs`

End-to-end scenarios testing multiple operations:

- [ ] `test_workflow_create_and_submit` — create form via API, submit, verify submission
- [ ] `test_workflow_crud_lifecycle` — create, read, update, delete form
- [ ] `test_workflow_sync_and_query` — sync forms, list, get by slug
- [ ] `test_workflow_submission_lifecycle` — submit, list, get, delete submission
- [ ] `test_workflow_custom_action_url` — create with action_url, verify HTML output

---

## Summary

| Category | Test Count | Priority |
|----------|------------|----------|
| Test Infrastructure | 2 files | High |
| Public Handlers | ~20 tests | High |
| Admin Handlers | ~30 tests | High |
| Admin Flag | 3 tests | Medium |
| Error Handling | ~10 tests | Medium |
| Health Check | 2 tests | Low |
| CLI Form Commands | ~15 tests | Medium |
| CLI Submission Commands | ~6 tests | Medium |
| CLI Database Commands | ~5 tests | Medium |
| Workflow Tests | 5 tests | Medium |

**Total: ~100 new tests**

---

## File Structure

```
anyform/tests/
├── common/
│   ├── mod.rs          (update)
│   ├── db.rs           (existing)
│   ├── fixtures.rs     (existing)
│   ├── http.rs         (NEW)
│   └── cli.rs          (NEW)
├── handlers_public_tests.rs    (NEW)
├── handlers_admin_tests.rs     (NEW)
├── handlers_admin_flag_tests.rs (NEW)
├── handlers_error_tests.rs     (NEW)
├── health_check_tests.rs       (NEW)
├── cli_form_tests.rs           (NEW)
├── cli_submission_tests.rs     (NEW)
├── cli_database_tests.rs       (NEW)
├── workflow_tests.rs           (NEW)
├── form_builder_tests.rs       (existing)
├── render_json_tests.rs        (existing)
├── sync_tests.rs               (existing)
└── validation_tests.rs         (existing)
```

---

## Dependencies

Add to `anyform/Cargo.toml` under `[dev-dependencies]`:

```toml
axum-test = "16"  # or tower-http test utilities
```

---

*Created: 2025-12-26*

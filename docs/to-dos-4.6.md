# anyform Phase 4.6: Custom Submission URLs

> **Status**: Complete
> **PRD Reference**: [axum-sea-forms-prd-0.4.2.md](/Users/rick/p/wordpuppi/docs/prd/libs/asf/axum-sea-forms-prd-0.4.2.md)
> **Depends On**: Phase 4.0-4.4 (core functionality)

---

## Overview

Allow forms to submit to external URLs instead of the built-in anyform handler.

**Implementation Notes:**
- Uses existing `FormSettings.action_url` and `FormSettings.method` fields
- No database migration required
- HTML renderer already supported these settings

---

## Completed Tasks

### 1. FormSettings Builder Methods
**File:** `anyform/src/schema/form_settings.rs`

- [x] Add `action_url(url)` builder method
- [x] Add `method(method)` builder method

### 2. FormJson Schema Update
**File:** `anyform/src/render/json.rs`

- [x] Add `action_url` field to `FormJson` (top-level convenience)
- [x] Add `action_method` field to `FormJson` (top-level convenience)
- [x] Populate from settings in `JsonRenderer::render()`

### 3. CLI Command
**Files:** `anyform/src/commands/mod.rs`, `anyform/src/commands/form.rs`

- [x] Add `SetAction` subcommand with `--url` and `--method` flags
- [x] Implement `set_action()` function
- [x] Add URL validation (http/https only, block javascript: and data:)
- [x] Support clearing URL with empty string

### 4. WASM Client
**Files:** `anyform-client/src/schema.rs`, `anyform-client/src/form_state.rs`, `anyform-client/src/hydrate.rs`

- [x] Add `action_url` and `action_method` to client `FormJson`
- [x] Add `action_url` and `method` to client `FormSettings`
- [x] Add `action_url()` method to `FormState` (returns custom or default)
- [x] Add `action_method()` method to `FormState` (returns method or "POST")
- [x] Update `hydrate.rs` to include new fields

---

## Usage Examples

### CLI
```bash
# Set custom action URL
anyform form set-action contact --url "https://api.example.com/leads"

# Clear (revert to anyform)
anyform form set-action contact --url ""
```

### JSON
```json
{
  "name": "Contact",
  "slug": "contact",
  "settings": {
    "action_url": "https://api.example.com/contact",
    "method": "POST"
  }
}
```

### Rust
```rust
FormSettings::new()
    .action_url("https://api.example.com/leads")
    .method("POST")
```

### WASM/JavaScript
```javascript
const url = form.action_url();    // Custom or "/api/forms/contact"
const method = form.action_method();  // "POST"
```

---

*Completed: 2025-12-26*

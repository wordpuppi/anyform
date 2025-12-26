# Phase 3.5: WASM Client Crate - Detailed Implementation Plan

> **Status:** Phases 3.1-3.4 COMPLETE, Phase 3.5 IN PROGRESS
> **Target:** Browser-side hydration for multi-step forms with conditions

---

## Overview

Create `axum-sea-forms-client` - a WASM crate that hydrates server-rendered HTML forms to add:
- Step navigation (prev/next buttons)
- Real-time condition evaluation (show/hide fields)
- Client-side validation before step navigation
- Error display integration

**Architecture:** Server renders complete HTML → WASM hydrates for interactivity → Single final submission

---

## Prerequisites (Already Implemented)

### HTML Data Attributes (from Phase 3.3)
```html
<form data-asf-form="contact">
  <div class="asf-step" data-asf-step="0" data-asf-visible="true">
    <div class="asf-field" data-asf-field="email" data-asf-validation='{"required":true}'>
    <div class="asf-field" data-asf-field="company" data-asf-condition='{"field":"has_company","op":"eq","value":true}'>
  </div>
  <div class="asf-navigation">
    <button class="asf-prev" disabled>Back</button>
    <button class="asf-next">Next</button>
    <button class="asf-submit" style="display:none">Submit</button>
  </div>
</form>
```

### ConditionRule (from Phase 3.1 - axum-sea-forms-core)
```rust
pub enum ConditionRule {
    Simple { field: String, op: ConditionOp, value: Option<Value> },
    And { and: Vec<ConditionRule> },
    Or { or: Vec<ConditionRule> },
}
impl ConditionRule {
    pub fn evaluate(&self, data: &HashMap<String, Value>) -> bool
}
```

---

## File Structure

```
axum-sea-forms-client/
├── Cargo.toml
├── src/
│   ├── lib.rs           # WASM entry point: init(), hydrate()
│   ├── form.rs          # AsfForm struct - main form controller
│   ├── step.rs          # Step navigation state machine
│   ├── field.rs         # Field value extraction & updates
│   ├── condition.rs     # Re-export from core + DOM integration
│   ├── validation.rs    # Client-side validation (mirrors server)
│   └── dom.rs           # DOM query helpers, event binding
└── tests/
    ├── condition_tests.rs
    ├── validation_tests.rs
    └── form_tests.rs
```

---

## Implementation Steps

### Step 1: Create Crate Structure

**Cargo.toml:**
```toml
[package]
name = "axum-sea-forms-client"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
axum-sea-forms-core = { path = "../axum-sea-forms-core" }
js-sys = "0.3"
wasm-bindgen-futures = "0.4"

[dependencies.web-sys]
version = "0.3"
features = [
    "Window", "Document", "Element", "HtmlElement",
    "HtmlInputElement", "HtmlSelectElement", "HtmlTextAreaElement",
    "HtmlFormElement", "HtmlButtonElement",
    "NodeList", "Event", "InputEvent", "SubmitEvent",
    "DomTokenList", "CssStyleDeclaration",
    "console"
]

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

**Update workspace Cargo.toml:**
```toml
members = ["axum-sea-forms", "axum-sea-forms-cli", "axum-sea-forms-core", "axum-sea-forms-client", "migration"]
```

---

### Step 2: Entry Point (lib.rs)

```rust
use wasm_bindgen::prelude::*;

mod form;
mod step;
mod field;
mod condition;
mod validation;
mod dom;

/// Called once when WASM loads - finds and hydrates all forms
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error messages
    console_error_panic_hook::set_once();

    // Auto-hydrate all forms on page
    hydrate_all();
}

/// Hydrate all forms with data-asf-form attribute
#[wasm_bindgen]
pub fn hydrate_all() {
    let forms = dom::query_all("[data-asf-form]");
    for form_el in forms {
        if let Err(e) = form::AsfForm::hydrate(form_el) {
            web_sys::console::error_1(&format!("Failed to hydrate form: {:?}", e).into());
        }
    }
}

/// Hydrate a specific form by slug
#[wasm_bindgen]
pub fn hydrate(slug: &str) -> Result<(), JsValue> {
    let selector = format!("[data-asf-form=\"{}\"]", slug);
    let form_el = dom::query(&selector)?;
    form::AsfForm::hydrate(form_el)?;
    Ok(())
}
```

---

### Step 3: Form Controller (form.rs)

```rust
pub struct AsfForm {
    element: HtmlFormElement,
    slug: String,
    steps: Vec<AsfStep>,
    fields: HashMap<String, AsfField>,
    current_step: usize,
}

impl AsfForm {
    pub fn hydrate(element: Element) -> Result<Self, JsValue> {
        // 1. Parse form slug from data-asf-form
        // 2. Find all steps (.asf-step)
        // 3. Find all fields (.asf-field)
        // 4. Parse conditions from data-asf-condition
        // 5. Parse validation from data-asf-validation
        // 6. Attach event listeners
        // 7. Evaluate initial conditions
        // 8. Show first visible step
    }

    fn bind_events(&self) {
        // - prev/next button clicks
        // - field input/change events
        // - form submit
    }

    fn on_field_change(&mut self, field_name: &str) {
        // 1. Update field value in state
        // 2. Re-evaluate all conditions
        // 3. Update visibility (data-asf-visible)
        // 4. Clear/update validation errors
    }

    fn go_to_step(&mut self, index: usize) -> bool {
        // 1. Validate current step (if moving forward)
        // 2. Hide current step
        // 3. Show target step
        // 4. Update button states
        // 5. Return success/failure
    }

    fn collect_data(&self) -> HashMap<String, serde_json::Value> {
        // Collect all field values for condition evaluation
    }
}
```

---

### Step 4: Step Navigation (step.rs)

```rust
pub struct AsfStep {
    index: usize,
    element: Element,
    condition: Option<ConditionRule>,
    fields: Vec<String>, // field names in this step
}

impl AsfStep {
    pub fn from_element(el: Element, index: usize) -> Result<Self, JsValue> {
        // Parse data-asf-condition if present
    }

    pub fn is_visible(&self, data: &HashMap<String, Value>) -> bool {
        match &self.condition {
            Some(rule) => rule.evaluate(data),
            None => true,
        }
    }

    pub fn show(&self) {
        self.element.set_attribute("data-asf-visible", "true");
    }

    pub fn hide(&self) {
        self.element.set_attribute("data-asf-visible", "false");
    }
}

pub struct StepNavigator {
    steps: Vec<AsfStep>,
    current: usize,
}

impl StepNavigator {
    pub fn next_visible(&self, data: &HashMap<String, Value>) -> Option<usize>
    pub fn prev_visible(&self, data: &HashMap<String, Value>) -> Option<usize>
    pub fn is_last_visible(&self, data: &HashMap<String, Value>) -> bool
    pub fn first_visible(&self, data: &HashMap<String, Value>) -> usize
}
```

---

### Step 5: Field Handling (field.rs)

```rust
pub struct AsfField {
    name: String,
    element: Element,
    input: FieldInput,
    condition: Option<ConditionRule>,
    validation: ValidationRules,
}

pub enum FieldInput {
    Text(HtmlInputElement),
    Textarea(HtmlTextAreaElement),
    Select(HtmlSelectElement),
    Checkbox(HtmlInputElement),
    Radio(Vec<HtmlInputElement>),
}

impl AsfField {
    pub fn get_value(&self) -> serde_json::Value {
        match &self.input {
            FieldInput::Text(el) => Value::String(el.value()),
            FieldInput::Checkbox(el) => Value::Bool(el.checked()),
            FieldInput::Select(el) => Value::String(el.value()),
            // ...
        }
    }

    pub fn is_visible(&self, data: &HashMap<String, Value>) -> bool {
        match &self.condition {
            Some(rule) => rule.evaluate(data),
            None => true,
        }
    }

    pub fn show_error(&self, message: &str) {
        self.element.class_list().add_1("asf-error");
        // Find or create .asf-error-message span
    }

    pub fn clear_error(&self) {
        self.element.class_list().remove_1("asf-error");
    }
}
```

---

### Step 6: Client Validation (validation.rs)

```rust
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationRules {
    pub required: Option<bool>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub pattern: Option<String>,
    pub min_selections: Option<usize>,
    pub max_selections: Option<usize>,
}

impl ValidationRules {
    pub fn validate(&self, value: &serde_json::Value, label: &str) -> Vec<String> {
        let mut errors = Vec::new();

        // Required check
        if self.required == Some(true) && is_empty(value) {
            errors.push(format!("{} is required", label));
            return errors;
        }

        // String length
        if let Some(s) = value.as_str() {
            if let Some(min) = self.min_length {
                if s.len() < min {
                    errors.push(format!("{} must be at least {} characters", label, min));
                }
            }
            // ... max_length, pattern
        }

        // Numeric range
        if let Some(n) = as_number(value) {
            // ... min, max
        }

        errors
    }
}

pub fn validate_step(fields: &[&AsfField], data: &HashMap<String, Value>) -> HashMap<String, Vec<String>> {
    let mut errors = HashMap::new();
    for field in fields {
        if !field.is_visible(data) {
            continue; // Skip hidden fields
        }
        let value = data.get(&field.name).unwrap_or(&Value::Null);
        let field_errors = field.validation.validate(value, &field.name);
        if !field_errors.is_empty() {
            errors.insert(field.name.clone(), field_errors);
        }
    }
    errors
}
```

---

### Step 7: DOM Helpers (dom.rs)

```rust
use web_sys::{window, Document, Element, NodeList};

pub fn document() -> Document {
    window().unwrap().document().unwrap()
}

pub fn query(selector: &str) -> Result<Element, JsValue> {
    document()
        .query_selector(selector)?
        .ok_or_else(|| JsValue::from_str(&format!("Element not found: {}", selector)))
}

pub fn query_all(selector: &str) -> Vec<Element> {
    let list = document().query_selector_all(selector).unwrap();
    (0..list.length())
        .filter_map(|i| list.item(i))
        .collect()
}

pub fn set_visible(el: &Element, visible: bool) {
    el.set_attribute("data-asf-visible", if visible { "true" } else { "false" }).ok();
}

pub fn add_event_listener<F>(el: &Element, event: &str, callback: F)
where
    F: FnMut(web_sys::Event) + 'static
{
    let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    el.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref()).ok();
    closure.forget(); // Prevent cleanup
}
```

---

## Test Plan

### Unit Tests (Rust - no DOM)

**tests/condition_tests.rs** - Already in core, can add client-specific tests
```rust
#[test]
fn test_parse_condition_from_json_string() {
    let json = r#"{"field":"country","op":"eq","value":"US"}"#;
    let rule: ConditionRule = serde_json::from_str(json).unwrap();
    // ...
}
```

**tests/validation_tests.rs**
```rust
#[test]
fn test_required_validation() {
    let rules = ValidationRules { required: Some(true), ..Default::default() };
    let errors = rules.validate(&Value::Null, "Email");
    assert_eq!(errors, vec!["Email is required"]);
}

#[test]
fn test_min_length_validation() {
    let rules = ValidationRules { min_length: Some(3), ..Default::default() };
    let errors = rules.validate(&Value::String("ab".into()), "Name");
    assert_eq!(errors, vec!["Name must be at least 3 characters"]);
}

#[test]
fn test_pattern_validation() {
    let rules = ValidationRules { pattern: Some(r"^\d+$".into()), ..Default::default() };
    let errors = rules.validate(&Value::String("abc".into()), "Code");
    assert!(!errors.is_empty());
}

#[test]
fn test_hidden_field_skipped() {
    // Field with false condition should not be validated
}
```

### WASM Integration Tests (wasm-bindgen-test)

**tests/wasm_tests.rs**
```rust
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_hydrate_simple_form() {
    // Create mock HTML
    let html = r#"
        <form data-asf-form="test">
            <div class="asf-step" data-asf-step="0" data-asf-visible="true">
                <div class="asf-field" data-asf-field="name"></div>
            </div>
        </form>
    "#;
    document().body().unwrap().set_inner_html(html);

    // Hydrate
    hydrate("test").expect("Should hydrate");

    // Verify form was processed (e.g., events attached)
}

#[wasm_bindgen_test]
fn test_step_navigation() {
    // Setup 2-step form
    // Click next
    // Verify step 0 hidden, step 1 visible
}

#[wasm_bindgen_test]
fn test_condition_hides_field() {
    // Setup field with condition
    // Verify initially hidden
    // Change dependent field
    // Verify now visible
}

#[wasm_bindgen_test]
fn test_validation_blocks_next() {
    // Setup step with required field
    // Click next without filling
    // Verify still on step 0
    // Verify error shown
}
```

### Manual E2E Test Cases

1. **Basic Navigation**
   - Load multi-step form
   - Click Next → moves to step 2
   - Click Back → returns to step 1
   - On last step → Submit button visible, Next hidden

2. **Conditional Fields**
   - Check "Has Company" checkbox
   - Company Name field appears
   - Uncheck → field hides
   - Company Name value cleared when hidden

3. **Conditional Steps**
   - Select "Enterprise" plan
   - "Enterprise Details" step becomes reachable
   - Select "Personal" plan
   - Step is skipped in navigation

4. **Validation**
   - Leave required field empty
   - Click Next → error shown, stays on step
   - Fill field → error clears
   - Click Next → proceeds

5. **Server Error Display**
   - Submit form with server-side validation error
   - Errors displayed per step
   - Navigate to step with errors

---

## Build & Output

```bash
# Build WASM
cd axum-sea-forms-client
wasm-pack build --target web --out-dir ../dist/wasm

# Output files:
# dist/wasm/axum_sea_forms_client.js   (~5KB)
# dist/wasm/axum_sea_forms_client_bg.wasm (~40KB)
```

---

## Integration Checklist

- [ ] Crate compiles to WASM
- [ ] `hydrate_all()` finds forms on page load
- [ ] Steps show/hide correctly
- [ ] Conditions evaluate on field change
- [ ] Validation blocks navigation
- [ ] Error messages display
- [ ] Submit button shows on last step
- [ ] Works with single-step forms (no errors)
- [ ] Target size <50KB gzipped

---

## Files to Create/Modify

| File | Action | Description |
|------|--------|-------------|
| `Cargo.toml` (workspace) | MODIFY | Add `axum-sea-forms-client` to members |
| `axum-sea-forms-client/Cargo.toml` | CREATE | WASM crate config |
| `axum-sea-forms-client/src/lib.rs` | CREATE | Entry point |
| `axum-sea-forms-client/src/form.rs` | CREATE | Form controller |
| `axum-sea-forms-client/src/step.rs` | CREATE | Step navigation |
| `axum-sea-forms-client/src/field.rs` | CREATE | Field handling |
| `axum-sea-forms-client/src/validation.rs` | CREATE | Client validation |
| `axum-sea-forms-client/src/dom.rs` | CREATE | DOM utilities |
| `axum-sea-forms-client/tests/*.rs` | CREATE | Test files |

---

## Next Phases (After 3.5)

### Phase 3.6: CLI WASM Command
- Add `asf wasm generate --output ./static/asf/` command
- Creates: asf-client.js, asf-client_bg.wasm

### Phase 3.7: WASM API Endpoints
- `GET /admin/wasm/client.js` → application/javascript
- `GET /admin/wasm/client.wasm` → application/wasm

### Phase 3.8: Build & Distribution
- GitHub Actions WASM build step
- Release artifacts with WASM files

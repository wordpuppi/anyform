//! Form state management for anyform-client.
//!
//! FormState tracks all form values, validation errors, touched fields,
//! and current step position for multi-step forms.

use crate::schema::{ConditionRule, FieldJson, FormJson, StepJson};
use crate::validation::validate_field;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

/// Client-side form state manager.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct FormState {
    schema: FormJson,
    values: HashMap<String, serde_json::Value>,
    errors: HashMap<String, Vec<String>>,
    touched: HashSet<String>,
    current_step_index: usize,
}

#[wasm_bindgen]
impl FormState {
    /// Creates a new FormState from a form schema.
    #[wasm_bindgen(constructor)]
    pub fn new(schema_js: JsValue) -> Result<FormState, JsValue> {
        let schema: FormJson = serde_wasm_bindgen::from_value(schema_js)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse schema: {}", e)))?;

        let mut values = HashMap::new();

        // Initialize with default values
        for step in &schema.steps {
            for field in &step.fields {
                if let Some(default) = &field.default_value {
                    values.insert(field.name.clone(), default.clone());
                }
            }
        }

        Ok(FormState {
            schema,
            values,
            errors: HashMap::new(),
            touched: HashSet::new(),
            current_step_index: 0,
        })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Value management
    // ─────────────────────────────────────────────────────────────────────────

    /// Sets a field value.
    pub fn set_value(&mut self, field: &str, value: JsValue) {
        let json_value: serde_json::Value = serde_wasm_bindgen::from_value(value)
            .unwrap_or(serde_json::Value::Null);
        self.values.insert(field.to_string(), json_value);

        // Re-validate if field was touched
        if self.touched.contains(field) {
            self.validate_field_internal(field);
        }
    }

    /// Gets a field value.
    pub fn get_value(&self, field: &str) -> JsValue {
        match self.values.get(field) {
            Some(value) => serde_wasm_bindgen::to_value(value).unwrap_or(JsValue::NULL),
            None => JsValue::NULL,
        }
    }

    /// Gets all values as a JS object.
    pub fn get_values(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.values).unwrap_or(JsValue::NULL)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Touch tracking
    // ─────────────────────────────────────────────────────────────────────────

    /// Marks a field as touched (user has interacted with it).
    pub fn mark_touched(&mut self, field: &str) {
        self.touched.insert(field.to_string());
        // Validate on touch
        self.validate_field_internal(field);
    }

    /// Checks if a field has been touched.
    pub fn is_touched(&self, field: &str) -> bool {
        self.touched.contains(field)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Validation
    // ─────────────────────────────────────────────────────────────────────────

    /// Validates a specific field and returns errors.
    pub fn validate_field(&mut self, field_name: &str) -> Vec<String> {
        self.validate_field_internal(field_name);
        self.errors.get(field_name).cloned().unwrap_or_default()
    }

    /// Validates all fields in a step.
    pub fn validate_step(&mut self, step_id: &str) -> JsValue {
        let step_uuid = Uuid::parse_str(step_id).ok();
        let mut step_errors: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(step) = self.schema.steps.iter().find(|s| Some(s.id) == step_uuid) {
            for field in &step.fields {
                // Skip hidden fields
                if !self.is_field_visible_internal(&field.name) {
                    continue;
                }

                let value = self.values.get(&field.name).unwrap_or(&serde_json::Value::Null);
                let errors = validate_field(field, value);
                if !errors.is_empty() {
                    step_errors.insert(field.name.clone(), errors.clone());
                    self.errors.insert(field.name.clone(), errors);
                } else {
                    self.errors.remove(&field.name);
                }
            }
        }

        serde_wasm_bindgen::to_value(&step_errors).unwrap_or(JsValue::NULL)
    }

    /// Validates all visible fields in the form.
    pub fn validate_all(&mut self) -> JsValue {
        for step in &self.schema.steps.clone() {
            // Skip hidden steps
            if !self.is_step_visible_internal(&step.id.to_string()) {
                continue;
            }

            for field in &step.fields {
                // Skip hidden fields
                if !self.is_field_visible_internal(&field.name) {
                    continue;
                }

                self.validate_field_internal(&field.name);
            }
        }

        serde_wasm_bindgen::to_value(&self.errors).unwrap_or(JsValue::NULL)
    }

    /// Returns true if the form has no validation errors.
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty() || self.errors.values().all(|e| e.is_empty())
    }

    /// Gets errors for a specific field.
    pub fn get_errors(&self, field: &str) -> Vec<String> {
        self.errors.get(field).cloned().unwrap_or_default()
    }

    /// Gets all errors as a JS object.
    pub fn get_all_errors(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.errors).unwrap_or(JsValue::NULL)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Visibility (condition evaluation)
    // ─────────────────────────────────────────────────────────────────────────

    /// Returns all visible steps.
    pub fn visible_steps(&self) -> JsValue {
        let visible: Vec<&StepJson> = self
            .schema
            .steps
            .iter()
            .filter(|s| self.is_step_visible_internal(&s.id.to_string()))
            .collect();
        serde_wasm_bindgen::to_value(&visible).unwrap_or(JsValue::NULL)
    }

    /// Returns visible fields, optionally filtered by step.
    pub fn visible_fields(&self, step_id: Option<String>) -> JsValue {
        let mut fields: Vec<&FieldJson> = Vec::new();

        let steps: Vec<&StepJson> = if let Some(id) = step_id {
            let uuid = Uuid::parse_str(&id).ok();
            self.schema.steps.iter().filter(|s| Some(s.id) == uuid).collect()
        } else {
            self.schema.steps.iter().collect()
        };

        for step in steps {
            if !self.is_step_visible_internal(&step.id.to_string()) {
                continue;
            }
            for field in &step.fields {
                if self.is_field_visible_internal(&field.name) {
                    fields.push(field);
                }
            }
        }

        serde_wasm_bindgen::to_value(&fields).unwrap_or(JsValue::NULL)
    }

    /// Checks if a step is visible.
    pub fn is_step_visible(&self, step_id: &str) -> bool {
        self.is_step_visible_internal(step_id)
    }

    /// Checks if a field is visible.
    pub fn is_field_visible(&self, field_name: &str) -> bool {
        self.is_field_visible_internal(field_name)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Multi-step navigation
    // ─────────────────────────────────────────────────────────────────────────

    /// Returns the current step.
    pub fn current_step(&self) -> JsValue {
        let visible_steps = self.get_visible_step_indices();
        if let Some(&index) = visible_steps.get(self.current_step_index) {
            if let Some(step) = self.schema.steps.get(index) {
                return serde_wasm_bindgen::to_value(step).unwrap_or(JsValue::NULL);
            }
        }
        JsValue::NULL
    }

    /// Returns the current step index (0-based, among visible steps).
    pub fn current_step_index(&self) -> usize {
        self.current_step_index
    }

    /// Moves to the next visible step. Returns true if successful.
    pub fn next_step(&mut self) -> bool {
        let visible_steps = self.get_visible_step_indices();
        if self.current_step_index + 1 < visible_steps.len() {
            self.current_step_index += 1;
            true
        } else {
            false
        }
    }

    /// Moves to the previous visible step. Returns true if successful.
    pub fn prev_step(&mut self) -> bool {
        if self.current_step_index > 0 {
            self.current_step_index -= 1;
            true
        } else {
            false
        }
    }

    /// Navigates to a specific step by ID. Returns true if successful.
    pub fn go_to_step(&mut self, step_id: &str) -> bool {
        let uuid = match Uuid::parse_str(step_id) {
            Ok(u) => u,
            Err(_) => return false,
        };

        let visible_steps = self.get_visible_step_indices();
        for (visible_index, &actual_index) in visible_steps.iter().enumerate() {
            if let Some(step) = self.schema.steps.get(actual_index) {
                if step.id == uuid {
                    self.current_step_index = visible_index;
                    return true;
                }
            }
        }
        false
    }

    /// Returns true if we can advance to the next step.
    pub fn can_go_next(&self) -> bool {
        let visible_steps = self.get_visible_step_indices();
        self.current_step_index + 1 < visible_steps.len()
    }

    /// Returns true if we can go back to the previous step.
    pub fn can_go_prev(&self) -> bool {
        self.current_step_index > 0
    }

    /// Returns progress as [current, total] (1-indexed for display).
    pub fn progress(&self) -> Vec<u32> {
        let visible_steps = self.get_visible_step_indices();
        vec![
            (self.current_step_index + 1) as u32,
            visible_steps.len() as u32,
        ]
    }

    /// Returns true if currently on the last step.
    pub fn is_last_step(&self) -> bool {
        let visible_steps = self.get_visible_step_indices();
        self.current_step_index + 1 >= visible_steps.len()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Schema access
    // ─────────────────────────────────────────────────────────────────────────

    /// Returns the form slug.
    pub fn slug(&self) -> String {
        self.schema.slug.clone()
    }

    /// Returns the form name.
    pub fn name(&self) -> String {
        self.schema.name.clone()
    }

    /// Returns the form schema as JS.
    pub fn schema(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.schema).unwrap_or(JsValue::NULL)
    }

    /// Returns the action URL for form submission.
    ///
    /// If a custom action URL is set, returns that URL.
    /// Otherwise, returns the default anyform submission endpoint.
    pub fn action_url(&self) -> String {
        self.schema
            .action_url
            .clone()
            .or_else(|| self.schema.settings.action_url.clone())
            .unwrap_or_else(|| format!("/api/forms/{}", self.schema.slug))
    }

    /// Returns the HTTP method for form submission.
    ///
    /// Returns "POST" if not explicitly configured.
    pub fn action_method(&self) -> String {
        self.schema
            .action_method
            .clone()
            .or_else(|| self.schema.settings.method.clone())
            .unwrap_or_else(|| "POST".to_string())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal methods (not exposed to JS)
// ─────────────────────────────────────────────────────────────────────────────

impl FormState {
    fn validate_field_internal(&mut self, field_name: &str) {
        // Find the field in the schema
        for step in &self.schema.steps {
            for field in &step.fields {
                if field.name == field_name {
                    let value = self.values.get(field_name).unwrap_or(&serde_json::Value::Null);
                    let errors = validate_field(field, value);
                    if errors.is_empty() {
                        self.errors.remove(field_name);
                    } else {
                        self.errors.insert(field_name.to_string(), errors);
                    }
                    return;
                }
            }
        }
    }

    fn is_step_visible_internal(&self, step_id: &str) -> bool {
        let uuid = match Uuid::parse_str(step_id) {
            Ok(u) => u,
            Err(_) => return false,
        };

        for step in &self.schema.steps {
            if step.id == uuid {
                return evaluate_condition(&step.condition, &self.values);
            }
        }
        false
    }

    fn is_field_visible_internal(&self, field_name: &str) -> bool {
        for step in &self.schema.steps {
            // Skip if step is hidden
            if !evaluate_condition(&step.condition, &self.values) {
                continue;
            }

            for field in &step.fields {
                if field.name == field_name {
                    return evaluate_condition(&field.condition, &self.values);
                }
            }
        }
        false
    }

    fn get_visible_step_indices(&self) -> Vec<usize> {
        self.schema
            .steps
            .iter()
            .enumerate()
            .filter(|(_, step)| evaluate_condition(&step.condition, &self.values))
            .map(|(i, _)| i)
            .collect()
    }
}

/// Evaluates a condition against form values.
fn evaluate_condition(
    condition: &Option<ConditionRule>,
    values: &HashMap<String, serde_json::Value>,
) -> bool {
    match condition {
        None => true, // No condition = always visible
        Some(rule) => rule.evaluate(values),
    }
}

// Re-export for use in form_client
impl FormState {
    /// Creates a FormState from a Rust FormJson struct.
    pub fn from_schema(schema: FormJson) -> Self {
        let mut values = HashMap::new();

        // Initialize with default values
        for step in &schema.steps {
            for field in &step.fields {
                if let Some(default) = &field.default_value {
                    values.insert(field.name.clone(), default.clone());
                }
            }
        }

        FormState {
            schema,
            values,
            errors: HashMap::new(),
            touched: HashSet::new(),
            current_step_index: 0,
        }
    }

    /// Gets all values as a Rust HashMap (for submission).
    pub fn values_map(&self) -> &HashMap<String, serde_json::Value> {
        &self.values
    }
}

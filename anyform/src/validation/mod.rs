//! Form validation engine.

use regex::Regex;
use std::collections::HashMap;

use crate::entities::field::Model as Field;
use crate::entities::step::Model as Step;
use crate::error::{StepValidationErrors, ValidationErrors};
use crate::schema::{FieldValue, ValidationRules, ValueType};

/// Validates a submission against a form's fields.
///
/// Returns a `ValidationErrors` containing any validation failures.
pub fn validate_submission(
    fields: &[Field],
    data: &HashMap<String, FieldValue>,
) -> ValidationErrors {
    let mut errors = ValidationErrors::new();

    for field in fields {
        // Skip display-only fields
        if field.is_display_only() {
            continue;
        }

        let value = data.get(&field.id.to_string()).or_else(|| data.get(&field.name));
        let field_errors = validate_field(field, value);

        for error in field_errors {
            errors.add(&field.name, error);
        }
    }

    errors
}

/// Validates a multi-step form submission with condition evaluation.
///
/// This function:
/// - Evaluates step conditions and skips hidden steps
/// - Evaluates field conditions and skips hidden fields
/// - Groups validation errors by step ID
///
/// # Arguments
///
/// * `steps` - Steps with their associated fields
/// * `data` - Submitted field values keyed by field name
///
/// # Returns
///
/// A `StepValidationErrors` containing any validation failures grouped by step.
pub fn validate_multi_step_submission(
    steps: &[(Step, Vec<Field>)],
    data: &HashMap<String, FieldValue>,
) -> StepValidationErrors {
    let mut errors = StepValidationErrors::new();

    // Convert FieldValue data to serde_json::Value for condition evaluation
    let json_data: HashMap<String, serde_json::Value> = data
        .iter()
        .map(|(k, v)| (k.clone(), v.into()))
        .collect();

    for (step, fields) in steps {
        // Check step condition - skip if step is hidden
        if let Some(condition) = step.condition_rule() {
            if !condition.evaluate(&json_data) {
                // Step is hidden, skip validation
                continue;
            }
        }

        let step_id = step.id.to_string();

        for field in fields {
            // Skip display-only fields
            if field.is_display_only() {
                continue;
            }

            // Check field condition - skip if field is hidden
            if let Some(condition) = field.condition() {
                if !condition.evaluate(&json_data) {
                    // Field is hidden, skip validation
                    continue;
                }
            }

            let value = data.get(&field.id.to_string()).or_else(|| data.get(&field.name));
            let field_errors = validate_field(field, value);

            for error in field_errors {
                errors.add(&step_id, &field.name, error);
            }
        }
    }

    errors
}

/// Validates a single step's fields with condition evaluation.
///
/// # Arguments
///
/// * `step` - The step being validated
/// * `fields` - Fields in this step
/// * `data` - Submitted field values
///
/// # Returns
///
/// A `ValidationErrors` for this step only.
pub fn validate_step(
    step: &Step,
    fields: &[Field],
    data: &HashMap<String, FieldValue>,
) -> ValidationErrors {
    let mut errors = ValidationErrors::new();

    // Convert FieldValue data to serde_json::Value for condition evaluation
    let json_data: HashMap<String, serde_json::Value> = data
        .iter()
        .map(|(k, v)| (k.clone(), v.into()))
        .collect();

    // Check step condition - return empty if step is hidden
    if let Some(condition) = step.condition_rule() {
        if !condition.evaluate(&json_data) {
            return errors;
        }
    }

    for field in fields {
        // Skip display-only fields
        if field.is_display_only() {
            continue;
        }

        // Check field condition - skip if field is hidden
        if let Some(condition) = field.condition() {
            if !condition.evaluate(&json_data) {
                continue;
            }
        }

        let value = data.get(&field.id.to_string()).or_else(|| data.get(&field.name));
        let field_errors = validate_field(field, value);

        for error in field_errors {
            errors.add(&field.name, error);
        }
    }

    errors
}

/// Checks if a field is visible based on its condition.
///
/// Returns `true` if the field has no condition or if the condition evaluates to true.
#[must_use]
pub fn is_field_visible(field: &Field, data: &HashMap<String, FieldValue>) -> bool {
    let Some(condition) = field.condition() else {
        return true;
    };

    let json_data: HashMap<String, serde_json::Value> = data
        .iter()
        .map(|(k, v)| (k.clone(), v.into()))
        .collect();

    condition.evaluate(&json_data)
}

/// Checks if a step is visible based on its condition.
///
/// Returns `true` if the step has no condition or if the condition evaluates to true.
#[must_use]
pub fn is_step_visible(step: &Step, data: &HashMap<String, FieldValue>) -> bool {
    let Some(condition) = step.condition_rule() else {
        return true;
    };

    let json_data: HashMap<String, serde_json::Value> = data
        .iter()
        .map(|(k, v)| (k.clone(), v.into()))
        .collect();

    condition.evaluate(&json_data)
}

/// Validates a single field value.
pub fn validate_field(field: &Field, value: Option<&FieldValue>) -> Vec<String> {
    let mut errors = Vec::new();
    let rules = field.validation();
    let value_type = field.value_type();

    // Check required
    if field.required {
        let is_empty = match value {
            None => true,
            Some(v) => v.is_empty(),
        };

        if is_empty {
            errors.push(format!("{} is required", field.label));
            return errors; // No point checking other rules if empty and required
        }
    }

    // If no value, skip other validations
    let Some(value) = value else {
        return errors;
    };

    if value.is_empty() {
        return errors;
    }

    // Type-specific validation
    if let Some(vt) = value_type {
        errors.extend(validate_by_type(vt, value, &field.label));
    }

    // Rule-based validation
    errors.extend(validate_by_rules(&rules, value, &field.label));

    errors
}

/// Validates a value based on its type.
fn validate_by_type(value_type: ValueType, value: &FieldValue, label: &str) -> Vec<String> {
    let mut errors = Vec::new();

    match value_type {
        ValueType::Email => {
            if let Some(s) = value.as_str() {
                if !is_valid_email(s) {
                    errors.push(format!("{label} must be a valid email address"));
                }
            }
        }
        ValueType::Url => {
            if let Some(s) = value.as_str() {
                if !is_valid_url(s) {
                    errors.push(format!("{label} must be a valid URL"));
                }
            }
        }
        ValueType::Number | ValueType::Rating | ValueType::Scale | ValueType::Nps => {
            if value.as_number().is_none() {
                errors.push(format!("{label} must be a number"));
            }
        }
        ValueType::Tel => {
            if let Some(s) = value.as_str() {
                if !is_valid_phone(s) {
                    errors.push(format!("{label} must be a valid phone number"));
                }
            }
        }
        ValueType::Date => {
            if let Some(s) = value.as_str() {
                if !is_valid_date(s) {
                    errors.push(format!("{label} must be a valid date (YYYY-MM-DD)"));
                }
            }
        }
        ValueType::DateTime => {
            if let Some(s) = value.as_str() {
                if !is_valid_datetime(s) {
                    errors.push(format!("{label} must be a valid date and time"));
                }
            }
        }
        ValueType::Time => {
            if let Some(s) = value.as_str() {
                if !is_valid_time(s) {
                    errors.push(format!("{label} must be a valid time (HH:MM)"));
                }
            }
        }
        _ => {}
    }

    errors
}

/// Validates a value based on validation rules.
fn validate_by_rules(rules: &ValidationRules, value: &FieldValue, label: &str) -> Vec<String> {
    let mut errors = Vec::new();

    // String length validation
    if let Some(s) = value.as_str() {
        if let Some(min) = rules.min_length {
            if s.len() < min {
                errors.push(format!("{label} must be at least {min} characters"));
            }
        }
        if let Some(max) = rules.max_length {
            if s.len() > max {
                errors.push(format!("{label} must be at most {max} characters"));
            }
        }

        // Pattern validation
        if let Some(pattern) = &rules.pattern {
            if let Ok(re) = Regex::new(pattern) {
                if !re.is_match(s) {
                    let message = rules
                        .pattern_message
                        .as_deref()
                        .unwrap_or("Invalid format");
                    errors.push(format!("{label}: {message}"));
                }
            }
        }
    }

    // Numeric validation
    if let Some(n) = value.as_number() {
        if let Some(min) = rules.min {
            if n < min {
                errors.push(format!("{label} must be at least {min}"));
            }
        }
        if let Some(max) = rules.max {
            if n > max {
                errors.push(format!("{label} must be at most {max}"));
            }
        }
    }

    // Array selection validation
    if let Some(arr) = value.as_array() {
        if let Some(min) = rules.min_selections {
            if arr.len() < min {
                errors.push(format!("{label} requires at least {min} selections"));
            }
        }
        if let Some(max) = rules.max_selections {
            if arr.len() > max {
                errors.push(format!("{label} allows at most {max} selections"));
            }
        }
    }

    errors
}

/// Checks if a string is a valid email address.
fn is_valid_email(s: &str) -> bool {
    // Basic email validation - contains @ and has text on both sides
    let re = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    re.is_match(s)
}

/// Checks if a string is a valid URL.
fn is_valid_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

/// Checks if a string is a valid phone number.
fn is_valid_phone(s: &str) -> bool {
    // Allow digits, spaces, dashes, parentheses, and plus sign
    let re = Regex::new(r"^[\d\s\-\(\)\+]+$").unwrap();
    re.is_match(s) && s.chars().filter(|c| c.is_ascii_digit()).count() >= 7
}

/// Checks if a string is a valid date (YYYY-MM-DD).
fn is_valid_date(s: &str) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    re.is_match(s)
}

/// Checks if a string is a valid datetime.
fn is_valid_datetime(s: &str) -> bool {
    // Accept ISO 8601 or datetime-local format
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}(:\d{2})?").unwrap();
    re.is_match(s)
}

/// Checks if a string is a valid time (HH:MM or HH:MM:SS).
fn is_valid_time(s: &str) -> bool {
    let re = Regex::new(r"^\d{2}:\d{2}(:\d{2})?$").unwrap();
    re.is_match(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn make_field(name: &str, field_type: &str, required: bool) -> Field {
        Field {
            id: Uuid::new_v4(),
            step_id: Uuid::new_v4(),
            name: name.to_string(),
            label: name.to_string(),
            field_type: field_type.to_string(),
            order: 0,
            required,
            placeholder: None,
            help_text: None,
            default_value: None,
            validation_rules: None,
            ui_options: None,
            correct_answer: None,
            points: None,
            weight: None,
            created_at: Utc::now().into(),
        }
    }

    fn make_step(name: &str, condition: Option<serde_json::Value>) -> Step {
        Step {
            id: Uuid::new_v4(),
            form_id: Uuid::new_v4(),
            name: name.to_string(),
            description: None,
            order: 0,
            condition,
            created_at: Utc::now().into(),
        }
    }

    fn make_conditional_field(name: &str, required: bool, condition: serde_json::Value) -> Field {
        let mut field = make_field(name, "text", required);
        field.ui_options = Some(serde_json::json!({ "condition": condition }));
        field
    }

    #[test]
    fn test_multi_step_basic_validation() {
        let step = make_step("Basic Info", None);
        let fields = vec![
            make_field("name", "text", true),
            make_field("email", "email", true),
        ];
        let steps = vec![(step, fields)];

        // Empty data should fail required validation
        let data = HashMap::new();
        let errors = validate_multi_step_submission(&steps, &data);
        assert!(!errors.is_empty());
        assert_eq!(errors.error_count(), 2);

        // Valid data should pass
        let mut data = HashMap::new();
        data.insert("name".to_string(), FieldValue::Text("John".to_string()));
        data.insert(
            "email".to_string(),
            FieldValue::Text("john@example.com".to_string()),
        );
        let errors = validate_multi_step_submission(&steps, &data);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_multi_step_conditional_field() {
        let step = make_step("Contact", None);
        let fields = vec![
            make_field("has_company", "checkbox", false),
            // Company name is only required when has_company is true
            make_conditional_field(
                "company_name",
                true,
                serde_json::json!({
                    "field": "has_company",
                    "op": "eq",
                    "value": true
                }),
            ),
        ];
        let steps = vec![(step, fields)];

        // No company checkbox checked - company_name should not be validated
        let data = HashMap::new();
        let errors = validate_multi_step_submission(&steps, &data);
        assert!(errors.is_empty());

        // Company checkbox checked but no company name - should fail
        let mut data = HashMap::new();
        data.insert("has_company".to_string(), FieldValue::Bool(true));
        let errors = validate_multi_step_submission(&steps, &data);
        assert!(!errors.is_empty());
        assert_eq!(errors.error_count(), 1);

        // Company checkbox checked with company name - should pass
        let mut data = HashMap::new();
        data.insert("has_company".to_string(), FieldValue::Bool(true));
        data.insert(
            "company_name".to_string(),
            FieldValue::Text("Acme Inc".to_string()),
        );
        let errors = validate_multi_step_submission(&steps, &data);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_multi_step_conditional_step() {
        // Step 2 only shows when role is "enterprise"
        let step1 = make_step("Account Type", None);
        let step2 = make_step(
            "Enterprise Details",
            Some(serde_json::json!({
                "field": "role",
                "op": "eq",
                "value": "enterprise"
            })),
        );

        let fields1 = vec![make_field("role", "select", true)];
        let fields2 = vec![make_field("company_size", "number", true)];

        let steps = vec![(step1, fields1), (step2, fields2)];

        // Not enterprise - step 2 should be skipped
        let mut data = HashMap::new();
        data.insert("role".to_string(), FieldValue::Text("personal".to_string()));
        let errors = validate_multi_step_submission(&steps, &data);
        assert!(errors.is_empty());

        // Enterprise but no company size - should fail
        let mut data = HashMap::new();
        data.insert(
            "role".to_string(),
            FieldValue::Text("enterprise".to_string()),
        );
        let errors = validate_multi_step_submission(&steps, &data);
        assert!(!errors.is_empty());
        assert_eq!(errors.error_count(), 1);

        // Enterprise with company size - should pass
        let mut data = HashMap::new();
        data.insert(
            "role".to_string(),
            FieldValue::Text("enterprise".to_string()),
        );
        data.insert("company_size".to_string(), FieldValue::Number(100.0));
        let errors = validate_multi_step_submission(&steps, &data);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_step_validation_errors_grouped_by_step() {
        let step1 = make_step("Step 1", None);
        let step1_id = step1.id.to_string();
        let step2 = make_step("Step 2", None);
        let step2_id = step2.id.to_string();

        let fields1 = vec![make_field("name", "text", true)];
        let fields2 = vec![make_field("email", "email", true)];

        let steps = vec![(step1, fields1), (step2, fields2)];

        // Empty data - should have errors in both steps
        let data = HashMap::new();
        let errors = validate_multi_step_submission(&steps, &data);

        assert!(!errors.is_empty());
        assert_eq!(errors.steps.len(), 2);
        assert!(errors.get_step(&step1_id).is_some());
        assert!(errors.get_step(&step2_id).is_some());
        assert!(errors.get_field(&step1_id, "name").is_some());
        assert!(errors.get_field(&step2_id, "email").is_some());
    }

    #[test]
    fn test_step_validation_errors_flatten() {
        let mut errors = StepValidationErrors::new();
        errors.add("step-1", "name", "Name is required");
        errors.add("step-1", "email", "Email is required");
        errors.add("step-2", "phone", "Phone is required");

        let flat = errors.flatten();
        assert_eq!(flat.len(), 3);
        assert!(flat.get("name").is_some());
        assert!(flat.get("email").is_some());
        assert!(flat.get("phone").is_some());
    }

    #[test]
    fn test_is_field_visible() {
        let field_no_condition = make_field("name", "text", true);
        let field_with_condition = make_conditional_field(
            "company",
            true,
            serde_json::json!({
                "field": "is_business",
                "op": "eq",
                "value": true
            }),
        );

        // Field without condition is always visible
        let data = HashMap::new();
        assert!(is_field_visible(&field_no_condition, &data));

        // Field with condition - not visible when condition is false
        let data = HashMap::new();
        assert!(!is_field_visible(&field_with_condition, &data));

        // Field with condition - visible when condition is true
        let mut data = HashMap::new();
        data.insert("is_business".to_string(), FieldValue::Bool(true));
        assert!(is_field_visible(&field_with_condition, &data));
    }

    #[test]
    fn test_is_step_visible() {
        let step_no_condition = make_step("Basic", None);
        let step_with_condition = make_step(
            "Enterprise",
            Some(serde_json::json!({
                "field": "plan",
                "op": "eq",
                "value": "enterprise"
            })),
        );

        // Step without condition is always visible
        let data = HashMap::new();
        assert!(is_step_visible(&step_no_condition, &data));

        // Step with condition - not visible when condition is false
        let mut data = HashMap::new();
        data.insert("plan".to_string(), FieldValue::Text("free".to_string()));
        assert!(!is_step_visible(&step_with_condition, &data));

        // Step with condition - visible when condition is true
        let mut data = HashMap::new();
        data.insert(
            "plan".to_string(),
            FieldValue::Text("enterprise".to_string()),
        );
        assert!(is_step_visible(&step_with_condition, &data));
    }

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name@sub.domain.com"));
        assert!(!is_valid_email("invalid"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("test@"));
    }

    #[test]
    fn test_url_validation() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://localhost:3000"));
        assert!(!is_valid_url("ftp://example.com"));
        assert!(!is_valid_url("example.com"));
    }

    #[test]
    fn test_phone_validation() {
        assert!(is_valid_phone("555-123-4567"));
        assert!(is_valid_phone("+1 (555) 123-4567"));
        assert!(is_valid_phone("5551234567"));
        assert!(!is_valid_phone("123")); // Too short
        assert!(!is_valid_phone("abc-def-ghij"));
    }

    #[test]
    fn test_date_validation() {
        assert!(is_valid_date("2024-12-25"));
        assert!(!is_valid_date("12-25-2024"));
        assert!(!is_valid_date("2024/12/25"));
    }

    #[test]
    fn test_time_validation() {
        assert!(is_valid_time("14:30"));
        assert!(is_valid_time("14:30:00"));
        assert!(!is_valid_time("2:30 PM"));
    }
}

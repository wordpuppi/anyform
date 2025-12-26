//! Client-side validation for anyform.
//!
//! This module mirrors the server-side validation logic to provide
//! instant feedback in the browser without round-trips to the server.

use crate::schema::{FieldJson, ValidationRules, ValueType};
use regex::Regex;

/// Validates a field value against its rules.
pub fn validate_field(
    field: &FieldJson,
    value: &serde_json::Value,
) -> Vec<String> {
    let mut errors = Vec::new();
    let rules = &field.validation;

    // Required validation
    if rules.required && is_empty(value) {
        errors.push(format!("{} is required", field.label));
        return errors; // Skip other validations if empty and required
    }

    // Skip other validations if value is empty
    if is_empty(value) {
        return errors;
    }

    // Type-specific validations
    match field.field_type {
        ValueType::Email => {
            if let Err(e) = validate_email(value) {
                errors.push(e);
            }
        }
        ValueType::Url => {
            if let Err(e) = validate_url(value) {
                errors.push(e);
            }
        }
        ValueType::Tel => {
            if let Err(e) = validate_tel(value) {
                errors.push(e);
            }
        }
        _ => {}
    }

    // Rule-based validations
    validate_rules(rules, value, &field.label, &mut errors);

    errors
}

/// Checks if a value is considered empty.
fn is_empty(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null => true,
        serde_json::Value::String(s) => s.trim().is_empty(),
        serde_json::Value::Array(a) => a.is_empty(),
        serde_json::Value::Object(o) => o.is_empty(),
        _ => false,
    }
}

/// Validates an email address.
fn validate_email(value: &serde_json::Value) -> Result<(), String> {
    let serde_json::Value::String(email) = value else {
        return Ok(());
    };

    // Simple email regex pattern
    let pattern = r"^[^\s@]+@[^\s@]+\.[^\s@]+$";
    let re = Regex::new(pattern).unwrap();

    if !re.is_match(email) {
        return Err("Invalid email format".to_string());
    }

    Ok(())
}

/// Validates a URL.
fn validate_url(value: &serde_json::Value) -> Result<(), String> {
    let serde_json::Value::String(url) = value else {
        return Ok(());
    };

    // Simple URL pattern
    let pattern = r"^https?://[^\s]+$";
    let re = Regex::new(pattern).unwrap();

    if !re.is_match(url) {
        return Err("Invalid URL format".to_string());
    }

    Ok(())
}

/// Validates a telephone number.
fn validate_tel(value: &serde_json::Value) -> Result<(), String> {
    let serde_json::Value::String(tel) = value else {
        return Ok(());
    };

    // Allow digits, spaces, dashes, parentheses, and + sign
    let pattern = r"^[\d\s\-\(\)\+]+$";
    let re = Regex::new(pattern).unwrap();

    if !re.is_match(tel) {
        return Err("Invalid phone number format".to_string());
    }

    Ok(())
}

/// Validates a value against validation rules.
fn validate_rules(
    rules: &ValidationRules,
    value: &serde_json::Value,
    label: &str,
    errors: &mut Vec<String>,
) {
    // Min length
    if let Some(min) = rules.min_length {
        if let serde_json::Value::String(s) = value {
            if s.len() < min {
                errors.push(format!("{} must be at least {} characters", label, min));
            }
        }
    }

    // Max length
    if let Some(max) = rules.max_length {
        if let serde_json::Value::String(s) = value {
            if s.len() > max {
                errors.push(format!("{} must be at most {} characters", label, max));
            }
        }
    }

    // Pattern
    if let Some(pattern) = &rules.pattern {
        if let serde_json::Value::String(s) = value {
            if let Ok(re) = Regex::new(pattern) {
                if !re.is_match(s) {
                    let message = rules
                        .pattern_message
                        .clone()
                        .unwrap_or_else(|| format!("{} does not match the required format", label));
                    errors.push(message);
                }
            }
        }
    }

    // Min value (numeric)
    if let Some(min) = rules.min_value {
        if let Some(num) = get_numeric_value(value) {
            if num < min {
                errors.push(format!("{} must be at least {}", label, min));
            }
        }
    }

    // Max value (numeric)
    if let Some(max) = rules.max_value {
        if let Some(num) = get_numeric_value(value) {
            if num > max {
                errors.push(format!("{} must be at most {}", label, max));
            }
        }
    }

    // Min selections (arrays)
    if let Some(min) = rules.min_selections {
        if let serde_json::Value::Array(arr) = value {
            if arr.len() < min {
                errors.push(format!("Select at least {} options", min));
            }
        }
    }

    // Max selections (arrays)
    if let Some(max) = rules.max_selections {
        if let serde_json::Value::Array(arr) = value {
            if arr.len() > max {
                errors.push(format!("Select at most {} options", max));
            }
        }
    }
}

/// Extracts a numeric value from a JSON value.
fn get_numeric_value(value: &serde_json::Value) -> Option<f64> {
    match value {
        serde_json::Value::Number(n) => n.as_f64(),
        serde_json::Value::String(s) => s.parse::<f64>().ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    fn make_field(name: &str, field_type: ValueType, validation: ValidationRules) -> FieldJson {
        FieldJson {
            id: Uuid::new_v4(),
            name: name.to_string(),
            label: name.to_string(),
            field_type,
            placeholder: None,
            help_text: None,
            default_value: None,
            validation,
            condition: None,
            options: vec![],
            order: 0,
        }
    }

    #[test]
    fn test_required_validation() {
        let field = make_field(
            "email",
            ValueType::Text,
            ValidationRules {
                required: true,
                ..Default::default()
            },
        );

        // Empty value fails
        assert!(!validate_field(&field, &json!(null)).is_empty());
        assert!(!validate_field(&field, &json!("")).is_empty());
        assert!(!validate_field(&field, &json!("  ")).is_empty());

        // Non-empty value passes
        assert!(validate_field(&field, &json!("test")).is_empty());
    }

    #[test]
    fn test_email_validation() {
        let field = make_field("email", ValueType::Email, ValidationRules::default());

        assert!(validate_field(&field, &json!("test@example.com")).is_empty());
        assert!(!validate_field(&field, &json!("invalid")).is_empty());
        assert!(!validate_field(&field, &json!("test@")).is_empty());
    }

    #[test]
    fn test_min_length_validation() {
        let field = make_field(
            "name",
            ValueType::Text,
            ValidationRules {
                min_length: Some(3),
                ..Default::default()
            },
        );

        assert!(!validate_field(&field, &json!("ab")).is_empty());
        assert!(validate_field(&field, &json!("abc")).is_empty());
        assert!(validate_field(&field, &json!("abcd")).is_empty());
    }

    #[test]
    fn test_pattern_validation() {
        let field = make_field(
            "code",
            ValueType::Text,
            ValidationRules {
                pattern: Some(r"^\d{4}$".to_string()),
                pattern_message: Some("Must be 4 digits".to_string()),
                ..Default::default()
            },
        );

        assert!(validate_field(&field, &json!("1234")).is_empty());
        assert!(!validate_field(&field, &json!("123")).is_empty());
        assert!(!validate_field(&field, &json!("abcd")).is_empty());
    }

    #[test]
    fn test_numeric_range_validation() {
        let field = make_field(
            "age",
            ValueType::Number,
            ValidationRules {
                min_value: Some(18.0),
                max_value: Some(120.0),
                ..Default::default()
            },
        );

        assert!(!validate_field(&field, &json!(17)).is_empty());
        assert!(validate_field(&field, &json!(18)).is_empty());
        assert!(validate_field(&field, &json!(50)).is_empty());
        assert!(validate_field(&field, &json!(120)).is_empty());
        assert!(!validate_field(&field, &json!(121)).is_empty());
    }
}

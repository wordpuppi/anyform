//! Condition rules for dynamic step/field visibility.
//!
//! Conditions use a simple JSON format that's easy to construct and validate:
//!
//! ```json
//! {"field": "country", "op": "eq", "value": "US"}
//! ```
//!
//! Compound conditions with AND/OR:
//!
//! ```json
//! {"and": [
//!   {"field": "country", "op": "eq", "value": "US"},
//!   {"field": "age", "op": "gte", "value": 18}
//! ]}
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Condition operators for comparing field values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOp {
    /// Equals (==)
    Eq,
    /// Not equals (!=)
    Neq,
    /// Greater than (>)
    Gt,
    /// Greater than or equal (>=)
    Gte,
    /// Less than (<)
    Lt,
    /// Less than or equal (<=)
    Lte,
    /// String contains substring
    Contains,
    /// Value is in array
    In,
    /// Field is empty (null, "", or missing)
    Empty,
    /// Field is not empty
    NotEmpty,
}

impl ConditionOp {
    /// Evaluate the operator against a field value and expected value.
    pub fn evaluate(
        &self,
        field_value: Option<&serde_json::Value>,
        expected: Option<&serde_json::Value>,
    ) -> bool {
        match self {
            ConditionOp::Empty => Self::is_empty(field_value),
            ConditionOp::NotEmpty => !Self::is_empty(field_value),
            _ => {
                let Some(field_val) = field_value else {
                    return false;
                };
                let Some(expected_val) = expected else {
                    return false;
                };
                self.compare(field_val, expected_val)
            }
        }
    }

    fn is_empty(value: Option<&serde_json::Value>) -> bool {
        match value {
            None => true,
            Some(serde_json::Value::Null) => true,
            Some(serde_json::Value::String(s)) => s.is_empty(),
            Some(serde_json::Value::Array(a)) => a.is_empty(),
            Some(serde_json::Value::Object(o)) => o.is_empty(),
            _ => false,
        }
    }

    fn compare(&self, field: &serde_json::Value, expected: &serde_json::Value) -> bool {
        match self {
            ConditionOp::Eq => Self::values_equal(field, expected),
            ConditionOp::Neq => !Self::values_equal(field, expected),
            ConditionOp::Gt => Self::compare_numeric(field, expected, |a, b| a > b),
            ConditionOp::Gte => Self::compare_numeric(field, expected, |a, b| a >= b),
            ConditionOp::Lt => Self::compare_numeric(field, expected, |a, b| a < b),
            ConditionOp::Lte => Self::compare_numeric(field, expected, |a, b| a <= b),
            ConditionOp::Contains => Self::string_contains(field, expected),
            ConditionOp::In => Self::value_in_array(field, expected),
            ConditionOp::Empty | ConditionOp::NotEmpty => unreachable!(),
        }
    }

    fn values_equal(a: &serde_json::Value, b: &serde_json::Value) -> bool {
        // Try numeric comparison first
        if let (Some(a_num), Some(b_num)) = (Self::as_f64(a), Self::as_f64(b)) {
            return (a_num - b_num).abs() < f64::EPSILON;
        }
        // Try string comparison
        if let (Some(a_str), Some(b_str)) = (Self::as_str(a), Self::as_str(b)) {
            return a_str == b_str;
        }
        // Try bool comparison
        if let (Some(a_bool), Some(b_bool)) = (a.as_bool(), b.as_bool()) {
            return a_bool == b_bool;
        }
        // Fall back to JSON equality
        a == b
    }

    fn compare_numeric<F>(field: &serde_json::Value, expected: &serde_json::Value, cmp: F) -> bool
    where
        F: Fn(f64, f64) -> bool,
    {
        let Some(field_num) = Self::as_f64(field) else {
            return false;
        };
        let Some(expected_num) = Self::as_f64(expected) else {
            return false;
        };
        cmp(field_num, expected_num)
    }

    fn string_contains(field: &serde_json::Value, expected: &serde_json::Value) -> bool {
        let Some(field_str) = Self::as_str(field) else {
            return false;
        };
        let Some(expected_str) = Self::as_str(expected) else {
            return false;
        };
        field_str.contains(expected_str)
    }

    fn value_in_array(field: &serde_json::Value, expected: &serde_json::Value) -> bool {
        let Some(arr) = expected.as_array() else {
            return false;
        };
        arr.iter().any(|item| Self::values_equal(field, item))
    }

    fn as_f64(value: &serde_json::Value) -> Option<f64> {
        match value {
            serde_json::Value::Number(n) => n.as_f64(),
            serde_json::Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    fn as_str(value: &serde_json::Value) -> Option<&str> {
        match value {
            serde_json::Value::String(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

/// A condition rule that determines visibility of steps or fields.
///
/// Supports simple comparisons, AND, and OR logic with arbitrary nesting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionRule {
    /// Simple comparison: `{"field": "x", "op": "eq", "value": "y"}`
    Simple {
        /// Field name to check
        field: String,
        /// Comparison operator
        op: ConditionOp,
        /// Expected value (optional for empty/not_empty operators)
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,
    },
    /// AND of multiple conditions: `{"and": [...]}`
    And {
        /// All conditions must be true
        and: Vec<ConditionRule>,
    },
    /// OR of multiple conditions: `{"or": [...]}`
    Or {
        /// At least one condition must be true
        or: Vec<ConditionRule>,
    },
}

impl ConditionRule {
    /// Create a simple equality condition.
    pub fn eq(field: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        Self::Simple {
            field: field.into(),
            op: ConditionOp::Eq,
            value: Some(value.into()),
        }
    }

    /// Create a simple not-equals condition.
    pub fn neq(field: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        Self::Simple {
            field: field.into(),
            op: ConditionOp::Neq,
            value: Some(value.into()),
        }
    }

    /// Create an "is empty" condition.
    pub fn empty(field: impl Into<String>) -> Self {
        Self::Simple {
            field: field.into(),
            op: ConditionOp::Empty,
            value: None,
        }
    }

    /// Create an "is not empty" condition.
    pub fn not_empty(field: impl Into<String>) -> Self {
        Self::Simple {
            field: field.into(),
            op: ConditionOp::NotEmpty,
            value: None,
        }
    }

    /// Create an AND condition from multiple rules.
    pub fn and(rules: Vec<ConditionRule>) -> Self {
        Self::And { and: rules }
    }

    /// Create an OR condition from multiple rules.
    pub fn or(rules: Vec<ConditionRule>) -> Self {
        Self::Or { or: rules }
    }

    /// Evaluate the condition against form data.
    ///
    /// # Arguments
    ///
    /// * `data` - A map of field names to their current values
    ///
    /// # Returns
    ///
    /// `true` if the condition is satisfied, `false` otherwise
    pub fn evaluate(&self, data: &HashMap<String, serde_json::Value>) -> bool {
        match self {
            ConditionRule::Simple { field, op, value } => {
                let field_value = data.get(field);
                op.evaluate(field_value, value.as_ref())
            }
            ConditionRule::And { and } => and.iter().all(|rule| rule.evaluate(data)),
            ConditionRule::Or { or } => or.iter().any(|rule| rule.evaluate(data)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_data(pairs: &[(&str, serde_json::Value)]) -> HashMap<String, serde_json::Value> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    #[test]
    fn test_eq_string() {
        let rule = ConditionRule::eq("country", "US");
        let data = make_data(&[("country", json!("US"))]);
        assert!(rule.evaluate(&data));

        let data = make_data(&[("country", json!("UK"))]);
        assert!(!rule.evaluate(&data));
    }

    #[test]
    fn test_eq_number() {
        let rule = ConditionRule::eq("age", 18);
        let data = make_data(&[("age", json!(18))]);
        assert!(rule.evaluate(&data));

        // String "18" should equal number 18
        let data = make_data(&[("age", json!("18"))]);
        assert!(rule.evaluate(&data));
    }

    #[test]
    fn test_neq() {
        let rule = ConditionRule::neq("status", "archived");
        let data = make_data(&[("status", json!("active"))]);
        assert!(rule.evaluate(&data));

        let data = make_data(&[("status", json!("archived"))]);
        assert!(!rule.evaluate(&data));
    }

    #[test]
    fn test_gt_gte_lt_lte() {
        let data = make_data(&[("score", json!(75))]);

        let gt = ConditionRule::Simple {
            field: "score".into(),
            op: ConditionOp::Gt,
            value: Some(json!(50)),
        };
        assert!(gt.evaluate(&data));

        let gte = ConditionRule::Simple {
            field: "score".into(),
            op: ConditionOp::Gte,
            value: Some(json!(75)),
        };
        assert!(gte.evaluate(&data));

        let lt = ConditionRule::Simple {
            field: "score".into(),
            op: ConditionOp::Lt,
            value: Some(json!(100)),
        };
        assert!(lt.evaluate(&data));

        let lte = ConditionRule::Simple {
            field: "score".into(),
            op: ConditionOp::Lte,
            value: Some(json!(75)),
        };
        assert!(lte.evaluate(&data));
    }

    #[test]
    fn test_contains() {
        let rule = ConditionRule::Simple {
            field: "email".into(),
            op: ConditionOp::Contains,
            value: Some(json!("@company.com")),
        };

        let data = make_data(&[("email", json!("user@company.com"))]);
        assert!(rule.evaluate(&data));

        let data = make_data(&[("email", json!("user@gmail.com"))]);
        assert!(!rule.evaluate(&data));
    }

    #[test]
    fn test_in() {
        let rule = ConditionRule::Simple {
            field: "role".into(),
            op: ConditionOp::In,
            value: Some(json!(["admin", "editor", "moderator"])),
        };

        let data = make_data(&[("role", json!("admin"))]);
        assert!(rule.evaluate(&data));

        let data = make_data(&[("role", json!("viewer"))]);
        assert!(!rule.evaluate(&data));
    }

    #[test]
    fn test_empty_not_empty() {
        let empty_rule = ConditionRule::empty("notes");
        let not_empty_rule = ConditionRule::not_empty("notes");

        // Missing field
        let data = make_data(&[]);
        assert!(empty_rule.evaluate(&data));
        assert!(!not_empty_rule.evaluate(&data));

        // Empty string
        let data = make_data(&[("notes", json!(""))]);
        assert!(empty_rule.evaluate(&data));
        assert!(!not_empty_rule.evaluate(&data));

        // Null value
        let data = make_data(&[("notes", json!(null))]);
        assert!(empty_rule.evaluate(&data));
        assert!(!not_empty_rule.evaluate(&data));

        // Non-empty string
        let data = make_data(&[("notes", json!("Hello"))]);
        assert!(!empty_rule.evaluate(&data));
        assert!(not_empty_rule.evaluate(&data));
    }

    #[test]
    fn test_and() {
        let rule = ConditionRule::and(vec![
            ConditionRule::eq("country", "US"),
            ConditionRule::Simple {
                field: "age".into(),
                op: ConditionOp::Gte,
                value: Some(json!(18)),
            },
        ]);

        // Both true
        let data = make_data(&[("country", json!("US")), ("age", json!(21))]);
        assert!(rule.evaluate(&data));

        // First false
        let data = make_data(&[("country", json!("UK")), ("age", json!(21))]);
        assert!(!rule.evaluate(&data));

        // Second false
        let data = make_data(&[("country", json!("US")), ("age", json!(16))]);
        assert!(!rule.evaluate(&data));
    }

    #[test]
    fn test_or() {
        let rule = ConditionRule::or(vec![
            ConditionRule::eq("plan", "enterprise"),
            ConditionRule::not_empty("referral_code"),
        ]);

        // First true
        let data = make_data(&[("plan", json!("enterprise"))]);
        assert!(rule.evaluate(&data));

        // Second true
        let data = make_data(&[("plan", json!("free")), ("referral_code", json!("ABC123"))]);
        assert!(rule.evaluate(&data));

        // Both false
        let data = make_data(&[("plan", json!("free")), ("referral_code", json!(""))]);
        assert!(!rule.evaluate(&data));
    }

    #[test]
    fn test_nested_conditions() {
        // (country == "US") AND ((age >= 21) OR (has_parental_consent == true))
        let rule = ConditionRule::and(vec![
            ConditionRule::eq("country", "US"),
            ConditionRule::or(vec![
                ConditionRule::Simple {
                    field: "age".into(),
                    op: ConditionOp::Gte,
                    value: Some(json!(21)),
                },
                ConditionRule::eq("has_parental_consent", true),
            ]),
        ]);

        // US, age 25 -> true
        let data = make_data(&[("country", json!("US")), ("age", json!(25))]);
        assert!(rule.evaluate(&data));

        // US, age 18 with consent -> true
        let data = make_data(&[
            ("country", json!("US")),
            ("age", json!(18)),
            ("has_parental_consent", json!(true)),
        ]);
        assert!(rule.evaluate(&data));

        // US, age 18 without consent -> false
        let data = make_data(&[
            ("country", json!("US")),
            ("age", json!(18)),
            ("has_parental_consent", json!(false)),
        ]);
        assert!(!rule.evaluate(&data));

        // UK, age 25 -> false (country doesn't match)
        let data = make_data(&[("country", json!("UK")), ("age", json!(25))]);
        assert!(!rule.evaluate(&data));
    }

    #[test]
    fn test_serde_simple() {
        let json_str = r#"{"field": "country", "op": "eq", "value": "US"}"#;
        let rule: ConditionRule = serde_json::from_str(json_str).unwrap();

        assert!(matches!(
            rule,
            ConditionRule::Simple {
                ref field,
                op: ConditionOp::Eq,
                ..
            } if field == "country"
        ));

        // Round-trip
        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: ConditionRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rule, deserialized);
    }

    #[test]
    fn test_serde_and() {
        let json_str = r#"{
            "and": [
                {"field": "country", "op": "eq", "value": "US"},
                {"field": "age", "op": "gte", "value": 18}
            ]
        }"#;
        let rule: ConditionRule = serde_json::from_str(json_str).unwrap();

        assert!(matches!(rule, ConditionRule::And { ref and } if and.len() == 2));
    }

    #[test]
    fn test_serde_empty_operator() {
        let json_str = r#"{"field": "notes", "op": "empty"}"#;
        let rule: ConditionRule = serde_json::from_str(json_str).unwrap();

        assert!(matches!(
            rule,
            ConditionRule::Simple {
                op: ConditionOp::Empty,
                value: None,
                ..
            }
        ));
    }
}

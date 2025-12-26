//! Field value types for form submissions.

use serde::{Deserialize, Serialize};

/// A value submitted for a form field.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    /// Text value.
    Text(String),

    /// Numeric value.
    Number(f64),

    /// Boolean value.
    Bool(bool),

    /// Array of values (for multi-select, checkboxes).
    Array(Vec<String>),

    /// Null/empty value.
    Null,
}

impl FieldValue {
    /// Returns the value as a string, if it is one.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the value as a string, converting if necessary.
    #[must_use]
    pub fn to_string_value(&self) -> String {
        match self {
            Self::Text(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Array(a) => a.join(", "),
            Self::Null => String::new(),
        }
    }

    /// Returns the value as a number, if it is one.
    #[must_use]
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            Self::Text(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Returns the value as a boolean, if it is one.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            Self::Text(s) => match s.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Some(true),
                "false" | "0" | "no" | "off" => Some(false),
                _ => None,
            },
            Self::Number(n) => Some(*n != 0.0),
            _ => None,
        }
    }

    /// Returns the value as an array, if it is one.
    #[must_use]
    pub fn as_array(&self) -> Option<&[String]> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Returns true if the value is null or empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Text(s) => s.is_empty(),
            Self::Array(a) => a.is_empty(),
            Self::Null => true,
            Self::Number(_) | Self::Bool(_) => false,
        }
    }

    /// Returns true if the value is null.
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

impl Default for FieldValue {
    fn default() -> Self {
        Self::Null
    }
}

impl From<String> for FieldValue {
    fn from(s: String) -> Self {
        if s.is_empty() {
            Self::Null
        } else {
            Self::Text(s)
        }
    }
}

impl From<&str> for FieldValue {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Self::Null
        } else {
            Self::Text(s.to_string())
        }
    }
}

impl From<f64> for FieldValue {
    fn from(n: f64) -> Self {
        Self::Number(n)
    }
}

impl From<i64> for FieldValue {
    fn from(n: i64) -> Self {
        Self::Number(n as f64)
    }
}

impl From<bool> for FieldValue {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<Vec<String>> for FieldValue {
    fn from(v: Vec<String>) -> Self {
        if v.is_empty() {
            Self::Null
        } else {
            Self::Array(v)
        }
    }
}

impl From<Option<String>> for FieldValue {
    fn from(opt: Option<String>) -> Self {
        match opt {
            Some(s) if !s.is_empty() => Self::Text(s),
            _ => Self::Null,
        }
    }
}

impl From<FieldValue> for serde_json::Value {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::Text(s) => serde_json::Value::String(s),
            FieldValue::Number(n) => serde_json::json!(n),
            FieldValue::Bool(b) => serde_json::Value::Bool(b),
            FieldValue::Array(a) => serde_json::Value::Array(
                a.into_iter().map(serde_json::Value::String).collect(),
            ),
            FieldValue::Null => serde_json::Value::Null,
        }
    }
}

impl From<&FieldValue> for serde_json::Value {
    fn from(value: &FieldValue) -> Self {
        match value {
            FieldValue::Text(s) => serde_json::Value::String(s.clone()),
            FieldValue::Number(n) => serde_json::json!(n),
            FieldValue::Bool(b) => serde_json::Value::Bool(*b),
            FieldValue::Array(a) => serde_json::Value::Array(
                a.iter().cloned().map(serde_json::Value::String).collect(),
            ),
            FieldValue::Null => serde_json::Value::Null,
        }
    }
}

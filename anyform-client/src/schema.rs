//! Schema types for anyform-client.
//!
//! These types are duplicated from the main anyform crate to avoid pulling in
//! heavy server-side dependencies (sea-orm, axum, etc.) into the WASM bundle.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Form JSON schema returned by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormJson {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub settings: FormSettings,
    pub steps: Vec<StepJson>,
}

/// Step in a multi-step form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepJson {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub order: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ConditionRule>,
    pub fields: Vec<FieldJson>,
}

/// Field in a form step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldJson {
    pub id: Uuid,
    pub name: String,
    pub label: String,
    pub field_type: ValueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(default)]
    pub validation: ValidationRules,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ConditionRule>,
    #[serde(default)]
    pub options: Vec<FieldOptionJson>,
    pub order: i32,
}

/// Option for select/radio/checkbox fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOptionJson {
    pub id: Uuid,
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub score: Option<i32>,
    pub order: i32,
}

/// Form settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormSettings {
    #[serde(default)]
    pub multi_step: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_button_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_redirect: Option<String>,
    #[serde(default)]
    pub show_progress: bool,
    #[serde(default)]
    pub allow_save_draft: bool,
}

/// Field value types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueType {
    Text,
    Textarea,
    Email,
    Url,
    Tel,
    Number,
    Date,
    Time,
    Datetime,
    Select,
    Radio,
    Checkbox,
    File,
    Hidden,
    Password,
    Color,
    Range,
    Rating,
    Scale,
}

/// Validation rules for a field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationRules {
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_selections: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selections: Option<usize>,
}

/// Condition rule for step/field visibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionRule {
    pub field: String,
    pub op: ConditionOp,
    pub value: serde_json::Value,
}

/// Condition operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOp {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    IsEmpty,
    IsNotEmpty,
}

impl ConditionRule {
    /// Evaluates this condition against the given form values.
    pub fn evaluate(&self, values: &std::collections::HashMap<String, serde_json::Value>) -> bool {
        let field_value = values.get(&self.field);

        match self.op {
            ConditionOp::IsEmpty => {
                field_value.is_none() || is_empty_value(field_value.unwrap())
            }
            ConditionOp::IsNotEmpty => {
                field_value.is_some() && !is_empty_value(field_value.unwrap())
            }
            _ => {
                let Some(field_value) = field_value else {
                    return false;
                };
                evaluate_comparison(&self.op, field_value, &self.value)
            }
        }
    }
}

fn is_empty_value(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null => true,
        serde_json::Value::String(s) => s.is_empty(),
        serde_json::Value::Array(a) => a.is_empty(),
        serde_json::Value::Object(o) => o.is_empty(),
        _ => false,
    }
}

fn evaluate_comparison(
    op: &ConditionOp,
    field_value: &serde_json::Value,
    condition_value: &serde_json::Value,
) -> bool {
    match op {
        ConditionOp::Eq => values_equal(field_value, condition_value),
        ConditionOp::Ne => !values_equal(field_value, condition_value),
        ConditionOp::Gt | ConditionOp::Gte | ConditionOp::Lt | ConditionOp::Lte => {
            compare_numeric(op, field_value, condition_value)
        }
        ConditionOp::Contains => string_contains(field_value, condition_value),
        ConditionOp::NotContains => !string_contains(field_value, condition_value),
        ConditionOp::StartsWith => string_starts_with(field_value, condition_value),
        ConditionOp::EndsWith => string_ends_with(field_value, condition_value),
        ConditionOp::IsEmpty | ConditionOp::IsNotEmpty => unreachable!(),
    }
}

fn values_equal(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    // Handle type coercion for common cases
    match (a, b) {
        (serde_json::Value::Number(n1), serde_json::Value::Number(n2)) => {
            n1.as_f64() == n2.as_f64()
        }
        (serde_json::Value::String(s), serde_json::Value::Number(n)) => {
            s.parse::<f64>().ok() == n.as_f64()
        }
        (serde_json::Value::Number(n), serde_json::Value::String(s)) => {
            n.as_f64() == s.parse::<f64>().ok()
        }
        (serde_json::Value::Bool(b1), serde_json::Value::Bool(b2)) => b1 == b2,
        (serde_json::Value::String(s), serde_json::Value::Bool(b)) => {
            matches!((s.as_str(), b), ("true", true) | ("false", false))
        }
        (serde_json::Value::Bool(b), serde_json::Value::String(s)) => {
            matches!((b, s.as_str()), (true, "true") | (false, "false"))
        }
        _ => a == b,
    }
}

fn compare_numeric(
    op: &ConditionOp,
    field_value: &serde_json::Value,
    condition_value: &serde_json::Value,
) -> bool {
    let field_num = match field_value {
        serde_json::Value::Number(n) => n.as_f64(),
        serde_json::Value::String(s) => s.parse::<f64>().ok(),
        _ => None,
    };
    let cond_num = match condition_value {
        serde_json::Value::Number(n) => n.as_f64(),
        serde_json::Value::String(s) => s.parse::<f64>().ok(),
        _ => None,
    };

    match (field_num, cond_num) {
        (Some(f), Some(c)) => match op {
            ConditionOp::Gt => f > c,
            ConditionOp::Gte => f >= c,
            ConditionOp::Lt => f < c,
            ConditionOp::Lte => f <= c,
            _ => false,
        },
        _ => false,
    }
}

fn string_contains(field_value: &serde_json::Value, condition_value: &serde_json::Value) -> bool {
    match (field_value, condition_value) {
        (serde_json::Value::String(s), serde_json::Value::String(needle)) => s.contains(needle.as_str()),
        (serde_json::Value::Array(arr), val) => arr.contains(val),
        _ => false,
    }
}

fn string_starts_with(field_value: &serde_json::Value, condition_value: &serde_json::Value) -> bool {
    match (field_value, condition_value) {
        (serde_json::Value::String(s), serde_json::Value::String(prefix)) => {
            s.starts_with(prefix.as_str())
        }
        _ => false,
    }
}

fn string_ends_with(field_value: &serde_json::Value, condition_value: &serde_json::Value) -> bool {
    match (field_value, condition_value) {
        (serde_json::Value::String(s), serde_json::Value::String(suffix)) => {
            s.ends_with(suffix.as_str())
        }
        _ => false,
    }
}

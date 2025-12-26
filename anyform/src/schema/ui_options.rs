//! UI options for form fields.

use crate::condition::ConditionRule;
use serde::{Deserialize, Serialize};

/// UI/display options for a form field.
///
/// These options control how the field is rendered in HTML.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiOptions {
    /// CSS class(es) for the field container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_class: Option<String>,

    /// CSS class(es) for the input element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_class: Option<String>,

    /// CSS class(es) for the label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_class: Option<String>,

    /// Width hint (e.g., "full", "half", "third", "25%").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,

    /// Number of rows for textarea.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<u32>,

    /// Number of columns for textarea.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cols: Option<u32>,

    /// Autocomplete attribute value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<String>,

    /// Whether to autofocus this field.
    #[serde(default)]
    pub autofocus: bool,

    /// Whether the field is disabled.
    #[serde(default)]
    pub disabled: bool,

    /// Whether the field is read-only.
    #[serde(default)]
    pub readonly: bool,

    /// Input mode (for mobile keyboards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputmode: Option<String>,

    /// Heading level for heading fields (1-6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading_level: Option<u8>,

    /// Maximum star rating value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rating: Option<u32>,

    /// Scale minimum value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_min: Option<i32>,

    /// Scale maximum value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_max: Option<i32>,

    /// Scale step value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_step: Option<i32>,

    /// Labels for scale endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_labels: Option<ScaleLabels>,

    /// Whether to show the character count.
    #[serde(default)]
    pub show_char_count: bool,

    /// Custom HTML attributes as key-value pairs.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub custom_attributes: std::collections::HashMap<String, String>,

    /// Condition rule for dynamic field visibility.
    ///
    /// When present, the field is only shown if the condition evaluates to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ConditionRule>,
}

/// Labels for scale endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleLabels {
    /// Label for the minimum value.
    pub min_label: Option<String>,
    /// Label for the maximum value.
    pub max_label: Option<String>,
    /// Label for the middle value.
    pub mid_label: Option<String>,
}

impl UiOptions {
    /// Creates new default UI options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the CSS class.
    #[must_use]
    pub fn css_class(mut self, class: impl Into<String>) -> Self {
        self.css_class = Some(class.into());
        self
    }

    /// Sets the width hint.
    #[must_use]
    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the number of rows for textarea.
    #[must_use]
    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = Some(rows);
        self
    }

    /// Sets autofocus.
    #[must_use]
    pub fn autofocus(mut self) -> Self {
        self.autofocus = true;
        self
    }

    /// Sets disabled state.
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Sets read-only state.
    #[must_use]
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }

    /// Adds a custom HTML attribute.
    #[must_use]
    pub fn attr(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_attributes.insert(name.into(), value.into());
        self
    }

    /// Sets a condition for dynamic field visibility.
    #[must_use]
    pub fn condition(mut self, condition: ConditionRule) -> Self {
        self.condition = Some(condition);
        self
    }
}

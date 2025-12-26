//! Validation rules for form fields.

use serde::{Deserialize, Serialize};

/// Validation rules for a form field.
///
/// These rules are stored as JSON in the database and applied
/// during submission validation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationRules {
    /// Minimum string length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,

    /// Maximum string length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,

    /// Minimum numeric value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,

    /// Maximum numeric value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,

    /// Step value for numeric inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,

    /// Regex pattern for validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    /// Custom error message for pattern validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_message: Option<String>,

    /// Minimum number of selections (for multi-select).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_selections: Option<usize>,

    /// Maximum number of selections (for multi-select).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selections: Option<usize>,

    /// Allowed file extensions (for file uploads).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_extensions: Option<Vec<String>>,

    /// Maximum file size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<usize>,

    /// Allowed MIME types (for file uploads).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mime_types: Option<Vec<String>>,

    /// Minimum date value (ISO 8601 format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_date: Option<String>,

    /// Maximum date value (ISO 8601 format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_date: Option<String>,

    /// Custom validation rules as JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<serde_json::Value>,
}

impl ValidationRules {
    /// Creates a new empty `ValidationRules`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the minimum string length.
    #[must_use]
    pub fn min_length(mut self, len: usize) -> Self {
        self.min_length = Some(len);
        self
    }

    /// Sets the maximum string length.
    #[must_use]
    pub fn max_length(mut self, len: usize) -> Self {
        self.max_length = Some(len);
        self
    }

    /// Sets the minimum numeric value.
    #[must_use]
    pub fn min(mut self, value: f64) -> Self {
        self.min = Some(value);
        self
    }

    /// Sets the maximum numeric value.
    #[must_use]
    pub fn max(mut self, value: f64) -> Self {
        self.max = Some(value);
        self
    }

    /// Sets the regex pattern.
    #[must_use]
    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    /// Sets the pattern error message.
    #[must_use]
    pub fn pattern_message(mut self, message: impl Into<String>) -> Self {
        self.pattern_message = Some(message.into());
        self
    }

    /// Returns true if any validation rules are set.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.min_length.is_none()
            && self.max_length.is_none()
            && self.min.is_none()
            && self.max.is_none()
            && self.step.is_none()
            && self.pattern.is_none()
            && self.min_selections.is_none()
            && self.max_selections.is_none()
            && self.allowed_extensions.is_none()
            && self.max_file_size.is_none()
            && self.allowed_mime_types.is_none()
            && self.min_date.is_none()
            && self.max_date.is_none()
            && self.custom.is_none()
    }
}

//! Error types for axum-sea-forms.

use axum::response::{IntoResponse, Response};
use http::StatusCode;
use std::collections::HashMap;

/// Core error type for axum-sea-forms.
#[derive(Debug, Clone, thiserror::Error)]
pub enum FormError {
    #[error("Form not found: {0}")]
    NotFound(String),

    #[error("Step not found: {0}")]
    StepNotFound(String),

    #[error("Field not found: {0}")]
    FieldNotFound(String),

    #[error("Validation failed")]
    ValidationFailed(ValidationErrors),

    #[error("Validation failed")]
    StepValidationFailed(StepValidationErrors),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Invalid field type: {0}")]
    InvalidFieldType(String),

    #[error("Condition evaluation failed: {0}")]
    ConditionError(String),

    #[error("File upload error: {0}")]
    FileUpload(String),

    #[error("Invalid form data: {0}")]
    InvalidData(String),

    #[error("Form is deleted")]
    FormDeleted,

    #[error("Submission not found: {0}")]
    SubmissionNotFound(String),
}

impl FormError {
    /// Returns the HTTP status code for this error.
    #[must_use]
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_)
            | Self::StepNotFound(_)
            | Self::FieldNotFound(_)
            | Self::SubmissionNotFound(_) => StatusCode::NOT_FOUND,
            Self::ValidationFailed(_)
            | Self::StepValidationFailed(_)
            | Self::InvalidFieldType(_)
            | Self::InvalidData(_) => StatusCode::BAD_REQUEST,
            Self::FormDeleted => StatusCode::GONE,
            Self::Database(_) | Self::ConditionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::FileUpload(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// Returns the error code string for this error.
    #[must_use]
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "FORM_NOT_FOUND",
            Self::StepNotFound(_) => "STEP_NOT_FOUND",
            Self::FieldNotFound(_) => "FIELD_NOT_FOUND",
            Self::ValidationFailed(_) | Self::StepValidationFailed(_) => "VALIDATION_FAILED",
            Self::Database(_) => "DATABASE_ERROR",
            Self::InvalidFieldType(_) => "INVALID_FIELD_TYPE",
            Self::ConditionError(_) => "CONDITION_ERROR",
            Self::FileUpload(_) => "FILE_UPLOAD_ERROR",
            Self::InvalidData(_) => "INVALID_DATA",
            Self::FormDeleted => "FORM_DELETED",
            Self::SubmissionNotFound(_) => "SUBMISSION_NOT_FOUND",
        }
    }
}

impl IntoResponse for FormError {
    fn into_response(self) -> Response {
        let api_response: crate::response::ApiResponse<()> = self.into();
        api_response.into_response()
    }
}

impl From<FormError> for crate::response::ApiResponse<()> {
    fn from(err: FormError) -> Self {
        match &err {
            FormError::ValidationFailed(errors) => {
                crate::response::ApiResponse::validation_failed(errors.clone())
            }
            FormError::StepValidationFailed(errors) => {
                crate::response::ApiResponse::step_validation_failed(errors.clone())
            }
            _ => crate::response::ApiResponse::error(
                err.error_code(),
                err.to_string(),
                err.status_code(),
            ),
        }
    }
}

impl From<sea_orm::DbErr> for FormError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<evalexpr::EvalexprError> for FormError {
    fn from(err: evalexpr::EvalexprError) -> Self {
        Self::ConditionError(err.to_string())
    }
}

/// Trait for converting FormError to your API response type.
///
/// Implement this trait on your own response type to integrate
/// axum-sea-forms errors with your API envelope pattern.
///
/// # Example
///
/// ```rust,ignore
/// use axum_sea_forms::{FormError, IntoApiError};
///
/// impl IntoApiError for ApiResponse<()> {
///     type Response = Self;
///
///     fn into_api_error(error: FormError) -> Self::Response {
///         ApiResponse {
///             data: None,
///             success: false,
///             status: error.status_code().as_u16(),
///             error: Some(error.to_string()),
///             ..Default::default()
///         }
///     }
/// }
/// ```
pub trait IntoApiError {
    type Response: IntoResponse;

    fn into_api_error(error: FormError) -> Self::Response;
}

/// Default implementation that returns the FormError directly.
impl IntoApiError for FormError {
    type Response = Self;

    fn into_api_error(error: FormError) -> Self::Response {
        error
    }
}

/// Validation errors mapped by field name.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ValidationErrors {
    /// Errors keyed by field name.
    pub errors: HashMap<String, Vec<String>>,
}

/// Validation errors grouped by step.
///
/// For multi-step forms, errors are organized by step ID, allowing clients
/// to display errors contextually within each step.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct StepValidationErrors {
    /// Errors keyed by step ID, then by field name.
    pub steps: HashMap<String, HashMap<String, Vec<String>>>,
}

impl StepValidationErrors {
    /// Creates new empty step validation errors.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if there are no validation errors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty() || self.steps.values().all(|fields| fields.is_empty())
    }

    /// Returns the total number of fields with errors across all steps.
    #[must_use]
    pub fn field_count(&self) -> usize {
        self.steps.values().map(HashMap::len).sum()
    }

    /// Returns the total number of error messages across all fields.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.steps
            .values()
            .flat_map(|fields| fields.values())
            .map(Vec::len)
            .sum()
    }

    /// Adds an error for a field within a step.
    pub fn add(
        &mut self,
        step_id: impl Into<String>,
        field: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.steps
            .entry(step_id.into())
            .or_default()
            .entry(field.into())
            .or_default()
            .push(message.into());
    }

    /// Gets errors for a specific step.
    #[must_use]
    pub fn get_step(&self, step_id: &str) -> Option<&HashMap<String, Vec<String>>> {
        self.steps.get(step_id)
    }

    /// Gets errors for a specific field within a step.
    #[must_use]
    pub fn get_field(&self, step_id: &str, field: &str) -> Option<&Vec<String>> {
        self.steps.get(step_id).and_then(|s| s.get(field))
    }

    /// Converts to flat `ValidationErrors`, losing step grouping.
    #[must_use]
    pub fn flatten(&self) -> ValidationErrors {
        let mut errors = ValidationErrors::new();
        for fields in self.steps.values() {
            for (field, messages) in fields {
                for message in messages {
                    errors.add(field.clone(), message.clone());
                }
            }
        }
        errors
    }
}

impl ValidationErrors {
    /// Creates a new empty `ValidationErrors`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if there are no validation errors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the number of fields with errors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Adds an error for a field.
    pub fn add(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.errors
            .entry(field.into())
            .or_default()
            .push(message.into());
    }

    /// Gets errors for a specific field.
    #[must_use]
    pub fn get(&self, field: &str) -> Option<&Vec<String>> {
        self.errors.get(field)
    }

    /// Merges another `ValidationErrors` into this one.
    pub fn merge(&mut self, other: Self) {
        for (field, messages) in other.errors {
            self.errors.entry(field).or_default().extend(messages);
        }
    }
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count = self.errors.values().map(Vec::len).sum::<usize>();
        write!(f, "{count} validation error(s)")
    }
}

impl std::fmt::Display for StepValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count = self.error_count();
        let step_count = self.steps.len();
        write!(f, "{count} validation error(s) in {step_count} step(s)")
    }
}

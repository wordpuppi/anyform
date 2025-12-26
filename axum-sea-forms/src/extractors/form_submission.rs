//! Form submission extractor.

use axum::extract::{FromRequest, Request};
use axum::http::header::CONTENT_TYPE;
use std::collections::HashMap;

use crate::error::FormError;
use crate::schema::FieldValue;

/// Extractor for form submission data.
///
/// Automatically detects content type and parses either:
/// - `application/x-www-form-urlencoded`
/// - `multipart/form-data`
/// - `application/json`
///
/// # Example
///
/// ```rust,ignore
/// use axum_sea_forms::FormSubmission;
///
/// async fn handle_submit(
///     FormSubmission(data): FormSubmission,
/// ) -> impl IntoResponse {
///     // data: HashMap<String, FieldValue>
///     // Process the submission...
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FormSubmission(pub HashMap<String, FieldValue>);

impl<S> FromRequest<S> for FormSubmission
where
    S: Send + Sync,
{
    type Rejection = FormError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let content_type = req
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if content_type.starts_with("application/json") {
            // Parse as JSON
            let bytes = axum::body::Bytes::from_request(req, state)
                .await
                .map_err(|e| FormError::InvalidData(e.to_string()))?;

            let data: HashMap<String, FieldValue> = serde_json::from_slice(&bytes)
                .map_err(|e| FormError::InvalidData(e.to_string()))?;

            Ok(Self(data))
        } else if content_type.starts_with("multipart/form-data") {
            // Parse as multipart
            let multipart = axum_extra::extract::Multipart::from_request(req, state)
                .await
                .map_err(|e| FormError::InvalidData(e.to_string()))?;

            let data = parse_multipart(multipart).await?;
            Ok(Self(data))
        } else {
            // Default to form-urlencoded
            let bytes = axum::body::Bytes::from_request(req, state)
                .await
                .map_err(|e| FormError::InvalidData(e.to_string()))?;

            let data = parse_urlencoded(&bytes)?;
            Ok(Self(data))
        }
    }
}

/// Parses URL-encoded form data.
fn parse_urlencoded(bytes: &[u8]) -> Result<HashMap<String, FieldValue>, FormError> {
    let mut data = HashMap::new();

    for (key, value) in form_urlencoded::parse(bytes) {
        let key = key.into_owned();
        let value = value.into_owned();

        // Handle array fields (e.g., "field[]" or "field[0]")
        if key.ends_with("[]") || key.contains('[') {
            let base_key = key.split('[').next().unwrap_or(&key).to_string();

            match data.get_mut(&base_key) {
                Some(FieldValue::Array(arr)) => {
                    arr.push(value);
                }
                _ => {
                    data.insert(base_key, FieldValue::Array(vec![value]));
                }
            }
        } else {
            data.insert(key, FieldValue::from(value));
        }
    }

    Ok(data)
}

/// Parses multipart form data.
async fn parse_multipart(
    mut multipart: axum_extra::extract::Multipart,
) -> Result<HashMap<String, FieldValue>, FormError> {
    let mut data = HashMap::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| FormError::InvalidData(e.to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();

        if name.is_empty() {
            continue;
        }

        // For now, treat file uploads as text (path/filename)
        // Full file handling will be added in Phase 2
        let value = field
            .text()
            .await
            .map_err(|e| FormError::InvalidData(e.to_string()))?;

        // Handle array fields
        if name.ends_with("[]") || name.contains('[') {
            let base_key = name.split('[').next().unwrap_or(&name).to_string();

            match data.get_mut(&base_key) {
                Some(FieldValue::Array(arr)) => {
                    arr.push(value);
                }
                _ => {
                    data.insert(base_key, FieldValue::Array(vec![value]));
                }
            }
        } else {
            data.insert(name, FieldValue::from(value));
        }
    }

    Ok(data)
}

impl FormSubmission {
    /// Gets a value by field name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&FieldValue> {
        self.0.get(name)
    }

    /// Gets a string value by field name.
    #[must_use]
    pub fn get_str(&self, name: &str) -> Option<&str> {
        self.0.get(name).and_then(FieldValue::as_str)
    }

    /// Returns true if the submission contains a field.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    /// Returns the number of fields in the submission.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the submission is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Consumes the extractor and returns the inner map.
    #[must_use]
    pub fn into_inner(self) -> HashMap<String, FieldValue> {
        self.0
    }
}

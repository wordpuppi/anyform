//! API response envelope types.
//!
//! Provides a consistent response format following Shopify/Stripe patterns.

use axum::response::{IntoResponse, Response};
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Display};
use uuid::Uuid;

use crate::error::{StepValidationErrors, ValidationErrors};

/// Standard API response envelope.
///
/// All API responses follow this format for consistency:
/// ```json
/// {
///   "data": { ... },
///   "success": true,
///   "status": 200,
///   "error": null,
///   "pagination": null,
///   "request_id": "uuid",
///   "meta": {}
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    /// Main data payload (null on errors).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// Whether the request succeeded.
    pub success: bool,

    /// HTTP status code.
    pub status: u16,

    /// Error information (null on success).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,

    /// Pagination info for list endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationInfo>,

    /// Request ID for tracking/debugging.
    pub request_id: String,

    /// Additional metadata.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub meta: HashMap<String, serde_json::Value>,
}

/// Error details in API responses.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiError {
    /// Error code (e.g., "FORM_NOT_FOUND").
    pub code: String,

    /// Human-readable error message.
    pub message: String,

    /// Additional error details (e.g., validation errors by field).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Pagination information for list responses.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationInfo {
    /// Current page number (1-indexed).
    pub current_page: u32,

    /// Number of items per page.
    pub per_page: u32,

    /// Total number of items across all pages.
    pub total_items: u32,

    /// Total number of pages.
    pub total_pages: u32,

    /// Cursor for next page (for cursor-based pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,

    /// Cursor for previous page (for cursor-based pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    /// Creates a successful response with data (200 OK).
    pub fn ok(data: T) -> Self {
        Self {
            data: Some(data),
            success: true,
            status: 200,
            error: None,
            pagination: None,
            request_id: Uuid::new_v4().to_string(),
            meta: HashMap::new(),
        }
    }

    /// Creates a successful response with data (201 Created).
    pub fn created(data: T) -> Self {
        Self {
            data: Some(data),
            success: true,
            status: 201,
            error: None,
            pagination: None,
            request_id: Uuid::new_v4().to_string(),
            meta: HashMap::new(),
        }
    }

    /// Sets the request ID (usually from X-Request-ID header).
    #[must_use]
    pub fn with_request_id(mut self, id: impl Into<String>) -> Self {
        self.request_id = id.into();
        self
    }

    /// Adds pagination information.
    #[must_use]
    pub fn with_pagination(mut self, pagination: PaginationInfo) -> Self {
        self.pagination = Some(pagination);
        self
    }

    /// Adds a metadata key-value pair.
    #[must_use]
    pub fn with_meta(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.meta.insert(key.into(), value);
        self
    }
}

impl ApiResponse<()> {
    /// Creates an empty successful response (200 OK).
    pub fn ok_empty() -> Self {
        Self {
            data: None,
            success: true,
            status: 200,
            error: None,
            pagination: None,
            request_id: Uuid::new_v4().to_string(),
            meta: HashMap::new(),
        }
    }

    /// Creates an error response.
    pub fn error(code: impl Into<String>, message: impl Into<String>, status: StatusCode) -> Self {
        Self {
            data: None,
            success: false,
            status: status.as_u16(),
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
                details: None,
            }),
            pagination: None,
            request_id: Uuid::new_v4().to_string(),
            meta: HashMap::new(),
        }
    }

    /// Creates an error response with details.
    pub fn error_with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        status: StatusCode,
        details: serde_json::Value,
    ) -> Self {
        Self {
            data: None,
            success: false,
            status: status.as_u16(),
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
                details: Some(details),
            }),
            pagination: None,
            request_id: Uuid::new_v4().to_string(),
            meta: HashMap::new(),
        }
    }

    /// Creates a 404 Not Found error response.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::error("NOT_FOUND", message, StatusCode::NOT_FOUND)
    }

    /// Creates a 400 Bad Request error response.
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::error("BAD_REQUEST", message, StatusCode::BAD_REQUEST)
    }

    /// Creates a 401 Unauthorized error response.
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::error("UNAUTHORIZED", message, StatusCode::UNAUTHORIZED)
    }

    /// Creates a 403 Forbidden error response.
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::error("FORBIDDEN", message, StatusCode::FORBIDDEN)
    }

    /// Creates a 500 Internal Server Error response.
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::error("INTERNAL_ERROR", message, StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// Creates a 422 Validation Failed error response.
    pub fn validation_failed(errors: ValidationErrors) -> Self {
        Self::error_with_details(
            "VALIDATION_FAILED",
            format!("{} validation error(s)", errors.len()),
            StatusCode::UNPROCESSABLE_ENTITY,
            serde_json::to_value(&errors.errors).unwrap_or_default(),
        )
    }

    /// Creates a 422 Validation Failed error response with step-grouped errors.
    ///
    /// The error details will have a `steps` key containing errors grouped by step ID.
    pub fn step_validation_failed(errors: StepValidationErrors) -> Self {
        Self::error_with_details(
            "VALIDATION_FAILED",
            format!("{} validation error(s)", errors.error_count()),
            StatusCode::UNPROCESSABLE_ENTITY,
            serde_json::json!({ "steps": errors.steps }),
        )
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::OK);
        (status, Json(&self)).into_response()
    }
}

impl<T: Serialize> Display for ApiResponse<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ApiResponse {{ success: {}, status: {}, request_id: {} }}",
            self.success, self.status, self.request_id
        )
    }
}

impl PaginationInfo {
    /// Creates pagination info for offset-based pagination.
    pub fn new(current_page: u32, per_page: u32, total_items: u32) -> Self {
        let total_pages = if total_items == 0 {
            1
        } else {
            (total_items + per_page - 1) / per_page
        };

        Self {
            current_page,
            per_page,
            total_items,
            total_pages,
            next_cursor: None,
            prev_cursor: None,
        }
    }

    /// Creates pagination info for cursor-based pagination.
    pub fn with_cursors(
        per_page: u32,
        total_items: u32,
        next_cursor: Option<String>,
        prev_cursor: Option<String>,
    ) -> Self {
        Self {
            current_page: 1,
            per_page,
            total_items,
            total_pages: 1,
            next_cursor,
            prev_cursor,
        }
    }
}

impl Display for PaginationInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Page {}/{} ({} items/page, {} total)",
            self.current_page, self.total_pages, self.per_page, self.total_items
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_response() {
        let response = ApiResponse::ok("hello");
        assert!(response.success);
        assert_eq!(response.status, 200);
        assert_eq!(response.data, Some("hello"));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_created_response() {
        let response = ApiResponse::created(42);
        assert!(response.success);
        assert_eq!(response.status, 201);
        assert_eq!(response.data, Some(42));
    }

    #[test]
    fn test_error_response() {
        let response = ApiResponse::<()>::not_found("Form not found");
        assert!(!response.success);
        assert_eq!(response.status, 404);
        assert!(response.data.is_none());
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, "NOT_FOUND");
        assert_eq!(error.message, "Form not found");
    }

    #[test]
    fn test_with_request_id() {
        let response = ApiResponse::ok("data").with_request_id("custom-id-123");
        assert_eq!(response.request_id, "custom-id-123");
    }

    #[test]
    fn test_with_pagination() {
        let pagination = PaginationInfo::new(2, 10, 45);
        let response = ApiResponse::ok(vec![1, 2, 3]).with_pagination(pagination);

        let pagination = response.pagination.unwrap();
        assert_eq!(pagination.current_page, 2);
        assert_eq!(pagination.per_page, 10);
        assert_eq!(pagination.total_items, 45);
        assert_eq!(pagination.total_pages, 5);
    }

    #[test]
    fn test_with_meta() {
        let response = ApiResponse::ok("data")
            .with_meta("version", serde_json::json!("1.0"))
            .with_meta("cached", serde_json::json!(true));

        assert_eq!(response.meta.get("version"), Some(&serde_json::json!("1.0")));
        assert_eq!(response.meta.get("cached"), Some(&serde_json::json!(true)));
    }

    #[test]
    fn test_validation_failed() {
        let mut errors = ValidationErrors::new();
        errors.add("email", "Invalid email format");
        errors.add("name", "Name is required");

        let response = ApiResponse::<()>::validation_failed(errors);
        assert!(!response.success);
        assert_eq!(response.status, 422);
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, "VALIDATION_FAILED");
        assert!(error.details.is_some());
    }

    #[test]
    fn test_pagination_calculation() {
        // Exact fit
        let p = PaginationInfo::new(1, 10, 30);
        assert_eq!(p.total_pages, 3);

        // Partial last page
        let p = PaginationInfo::new(1, 10, 25);
        assert_eq!(p.total_pages, 3);

        // Empty
        let p = PaginationInfo::new(1, 10, 0);
        assert_eq!(p.total_pages, 1);

        // Single page
        let p = PaginationInfo::new(1, 10, 5);
        assert_eq!(p.total_pages, 1);
    }
}

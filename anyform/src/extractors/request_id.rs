//! Request ID extractor.
//!
//! Extracts request ID from X-Request-ID header or generates a new UUID.

use axum::extract::FromRequestParts;
use http::request::Parts;
use std::convert::Infallible;
use uuid::Uuid;

/// Extractor for request ID.
///
/// Reads from `X-Request-ID` header if present, otherwise generates a new UUID.
/// Use this to track requests across logs and responses.
///
/// # Example
///
/// ```rust,ignore
/// use anyform::extractors::RequestId;
///
/// async fn handler(RequestId(id): RequestId) -> String {
///     format!("Request ID: {}", id)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct RequestId(pub String);

impl RequestId {
    /// Returns the request ID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes self and returns the inner String.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<S> FromRequestParts<S> for RequestId
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let id = parts
            .headers
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        Ok(RequestId(id))
    }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for RequestId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    #[tokio::test]
    async fn test_extracts_from_header() {
        let request = Request::builder()
            .header("x-request-id", "test-id-123")
            .body(())
            .unwrap();

        let (mut parts, _body) = request.into_parts();
        let RequestId(id) = RequestId::from_request_parts(&mut parts, &()).await.unwrap();

        assert_eq!(id, "test-id-123");
    }

    #[tokio::test]
    async fn test_generates_uuid_when_missing() {
        let request = Request::builder().body(()).unwrap();

        let (mut parts, _body) = request.into_parts();
        let RequestId(id) = RequestId::from_request_parts(&mut parts, &()).await.unwrap();

        // Should be a valid UUID
        assert!(Uuid::parse_str(&id).is_ok());
    }

    #[tokio::test]
    async fn test_generates_uuid_when_empty() {
        let request = Request::builder()
            .header("x-request-id", "")
            .body(())
            .unwrap();

        let (mut parts, _body) = request.into_parts();
        let RequestId(id) = RequestId::from_request_parts(&mut parts, &()).await.unwrap();

        // Should be a valid UUID (not empty string)
        assert!(Uuid::parse_str(&id).is_ok());
    }
}

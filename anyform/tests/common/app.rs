//! HTTP testing infrastructure for integration tests.
//!
//! Provides `TestApp` for making HTTP requests against the anyform router
//! and `TestResponse` for asserting on responses.

use axum::body::Body;
use axum::Router;
use bytes::Bytes;
use http::{Request, StatusCode};
use http_body_util::BodyExt;
use sea_orm::DatabaseConnection;
use serde::de::DeserializeOwned;
use tower::ServiceExt;

use super::db::TestDb;

/// Test application wrapper for HTTP integration tests.
///
/// Wraps a `TestDb` and an Axum `Router` for making test requests.
///
/// # Example
///
/// ```ignore
/// let app = TestApp::new().await;
/// let response = app.get("/api/forms/my-form/json").await;
/// response.assert_status(StatusCode::OK);
/// ```
pub struct TestApp {
    test_db: TestDb,
    router: Router,
}

impl TestApp {
    /// Creates a new test app with public routes only.
    pub async fn new() -> Self {
        let test_db = TestDb::new().await;
        let router = anyform::AnyFormRouter::new(test_db.db.clone());
        Self { test_db, router }
    }

    /// Creates a new test app with admin routes enabled.
    #[cfg(feature = "admin")]
    pub async fn with_admin() -> Self {
        let test_db = TestDb::new().await;
        let router = anyform::AnyFormRouter::builder()
            .database(test_db.db.clone())
            .enable_admin(true)
            .build();
        Self { test_db, router }
    }

    /// Returns a reference to the database connection.
    pub fn db(&self) -> &DatabaseConnection {
        &self.test_db.db
    }

    /// Sends a GET request and returns the response.
    pub async fn get(&self, uri: &str) -> TestResponse {
        let request = Request::builder()
            .uri(uri)
            .method("GET")
            .body(Body::empty())
            .expect("Failed to build GET request");
        self.send(request).await
    }

    /// Sends a POST request with JSON body.
    pub async fn post_json<T: serde::Serialize>(&self, uri: &str, body: &T) -> TestResponse {
        let json = serde_json::to_vec(body).expect("Failed to serialize JSON body");
        let request = Request::builder()
            .uri(uri)
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(json))
            .expect("Failed to build POST request");
        self.send(request).await
    }

    /// Sends a POST request with form-urlencoded body.
    pub async fn post_form(&self, uri: &str, data: &[(&str, &str)]) -> TestResponse {
        let body: String = form_urlencoded::Serializer::new(String::new())
            .extend_pairs(data.iter().copied())
            .finish();
        let request = Request::builder()
            .uri(uri)
            .method("POST")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from(body))
            .expect("Failed to build POST form request");
        self.send(request).await
    }

    /// Sends a PUT request with JSON body.
    pub async fn put_json<T: serde::Serialize>(&self, uri: &str, body: &T) -> TestResponse {
        let json = serde_json::to_vec(body).expect("Failed to serialize JSON body");
        let request = Request::builder()
            .uri(uri)
            .method("PUT")
            .header("Content-Type", "application/json")
            .body(Body::from(json))
            .expect("Failed to build PUT request");
        self.send(request).await
    }

    /// Sends a DELETE request.
    pub async fn delete(&self, uri: &str) -> TestResponse {
        let request = Request::builder()
            .uri(uri)
            .method("DELETE")
            .body(Body::empty())
            .expect("Failed to build DELETE request");
        self.send(request).await
    }

    /// Sends a raw request to the router.
    async fn send(&self, request: Request<Body>) -> TestResponse {
        let response = self
            .router
            .clone()
            .oneshot(request)
            .await
            .expect("Request failed");
        TestResponse::from_response(response).await
    }

    /// Sends a raw request (public method for advanced testing).
    pub async fn send_raw(&self, request: Request<Body>) -> TestResponse {
        self.send(request).await
    }
}

/// Wrapper around HTTP response for easier assertions.
pub struct TestResponse {
    /// The HTTP status code.
    pub status: StatusCode,
    /// The response headers.
    pub headers: http::HeaderMap,
    /// The response body as bytes.
    pub body: Bytes,
}

impl TestResponse {
    async fn from_response(response: http::Response<Body>) -> Self {
        let status = response.status();
        let headers = response.headers().clone();
        let body = response
            .into_body()
            .collect()
            .await
            .expect("Failed to read response body")
            .to_bytes();
        Self {
            status,
            headers,
            body,
        }
    }

    /// Parses the response body as JSON.
    pub fn json<T: DeserializeOwned>(&self) -> T {
        serde_json::from_slice(&self.body).expect("Failed to parse JSON response")
    }

    /// Returns the response body as a string.
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    /// Asserts the response has the expected status code.
    pub fn assert_status(&self, expected: StatusCode) -> &Self {
        assert_eq!(
            self.status, expected,
            "Expected status {}, got {}. Body: {}",
            expected,
            self.status,
            self.text()
        );
        self
    }

    /// Asserts the response is successful (2xx).
    pub fn assert_success(&self) -> &Self {
        assert!(
            self.status.is_success(),
            "Expected success status, got {}. Body: {}",
            self.status,
            self.text()
        );
        self
    }

    /// Asserts the response contains JSON with `success: true`.
    pub fn assert_api_success(&self) -> &Self {
        let json: serde_json::Value = self.json();
        assert_eq!(
            json.get("success"),
            Some(&serde_json::Value::Bool(true)),
            "Expected API success. Response: {}",
            serde_json::to_string_pretty(&json).unwrap_or_default()
        );
        self
    }

    /// Asserts the response contains JSON with `success: false` and the expected error code.
    pub fn assert_api_error(&self, expected_code: &str) -> &Self {
        let json: serde_json::Value = self.json();
        assert_eq!(
            json.get("success"),
            Some(&serde_json::Value::Bool(false)),
            "Expected API error. Response: {}",
            serde_json::to_string_pretty(&json).unwrap_or_default()
        );
        let error = json.get("error").expect("Missing 'error' field in response");
        let code = error
            .get("code")
            .and_then(|c| c.as_str())
            .expect("Missing 'error.code' field");
        assert_eq!(
            code, expected_code,
            "Expected error code '{}', got '{}'",
            expected_code, code
        );
        self
    }

    /// Asserts the response body contains the given text.
    pub fn assert_body_contains(&self, text: &str) -> &Self {
        let body = self.text();
        assert!(
            body.contains(text),
            "Expected body to contain '{}'. Body: {}",
            text,
            body
        );
        self
    }

    /// Asserts the Content-Type header matches the expected value.
    pub fn assert_content_type(&self, expected: &str) -> &Self {
        let content_type = self
            .headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(
            content_type.starts_with(expected),
            "Expected Content-Type starting with '{}', got '{}'",
            expected,
            content_type
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_creation() {
        let app = TestApp::new().await;
        // Verify database is accessible
        assert!(app.db().ping().await.is_ok());
    }

    #[cfg(feature = "admin")]
    #[tokio::test]
    async fn test_app_with_admin() {
        let app = TestApp::with_admin().await;
        assert!(app.db().ping().await.is_ok());
    }
}

//! Integration tests for health check endpoint.
//!
//! Tests cover:
//! - GET /health - Returns 200 OK with "OK" body

mod common;

use common::TestApp;
use http::StatusCode;

#[tokio::test]
async fn test_health_check_returns_200() {
    let app = TestApp::new().await;

    let response = app.get("/health").await;

    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_health_check_body_is_ok() {
    let app = TestApp::new().await;

    let response = app.get("/health").await;

    response.assert_status(StatusCode::OK);
    assert_eq!(response.text(), "OK");
}

#[tokio::test]
async fn test_health_check_content_type() {
    let app = TestApp::new().await;

    let response = app.get("/health").await;

    response.assert_status(StatusCode::OK);
    response.assert_content_type("text/plain");
}

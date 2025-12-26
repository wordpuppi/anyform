//! Integration tests for error handling.
//!
//! Tests cover:
//! - 404 Not Found responses
//! - 400 Bad Request responses
//! - Response format consistency

mod common;

use common::{contact_form, create_test_form, TestApp};
use http::StatusCode;

// ============================================================================
// 404 Not Found Errors
// ============================================================================

#[tokio::test]
async fn test_unknown_route_returns_404() {
    let app = TestApp::new().await;

    let response = app.get("/api/unknown-route").await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_form_not_found_returns_proper_error() {
    let app = TestApp::new().await;

    let response = app.get("/api/forms/nonexistent/json").await;

    response.assert_status(StatusCode::NOT_FOUND);
    response.assert_api_error("FORM_NOT_FOUND");
}

#[cfg(feature = "admin")]
#[tokio::test]
async fn test_admin_form_not_found_returns_proper_error() {
    let app = TestApp::with_admin().await;
    let random_id = uuid::Uuid::new_v4();

    let response = app.get(&format!("/api/admin/forms/{}", random_id)).await;

    response.assert_status(StatusCode::NOT_FOUND);
    response.assert_api_error("FORM_NOT_FOUND");
}

// ============================================================================
// 400 Bad Request Errors
// ============================================================================

#[cfg(feature = "admin")]
#[tokio::test]
async fn test_invalid_uuid_path_returns_400() {
    let app = TestApp::with_admin().await;

    let response = app.get("/api/admin/forms/not-a-valid-uuid").await;

    response.assert_status(StatusCode::BAD_REQUEST);
}

#[cfg(feature = "admin")]
#[tokio::test]
async fn test_invalid_json_body_returns_400() {
    let app = TestApp::with_admin().await;

    // Send malformed JSON by using raw post
    let request = http::Request::builder()
        .uri("/api/admin/forms")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(r#"{"invalid json"#))
        .unwrap();

    let response = app.send_raw(request).await;

    response.assert_status(StatusCode::BAD_REQUEST);
}

// ============================================================================
// Validation Error Responses
// ============================================================================

#[tokio::test]
async fn test_validation_error_includes_field_details() {
    let app = TestApp::new().await;

    // Create form with required field
    let input = anyform::CreateFormInput::new("Validation Test", "validation-test")
        .step(
            anyform::CreateStepInput::new("Step 1")
                .field(anyform::CreateFieldInput::new("required_field", "Required", "text").required())
                .field(anyform::CreateFieldInput::new("email_field", "Email", "email")),
        );
    let form = create_test_form(app.db(), input).await;

    // Submit with missing required and invalid email
    let data = serde_json::json!({
        "email_field": "not-an-email"
    });
    let response = app.post_json(&format!("/api/forms/{}", form.slug), &data).await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    response.assert_api_error("VALIDATION_FAILED");

    let json: serde_json::Value = response.json();
    let error = json["error"].as_object().unwrap();

    // Should have details
    assert!(error.contains_key("details"), "Error should contain details");
}

#[tokio::test]
async fn test_validation_error_has_correct_format() {
    let app = TestApp::new().await;

    let input = anyform::CreateFormInput::new("Format Test", "format-test")
        .step(
            anyform::CreateStepInput::new("Step 1")
                .field(anyform::CreateFieldInput::new("required_field", "Required", "text").required()),
        );
    let form = create_test_form(app.db(), input).await;

    let response = app.post_json(&format!("/api/forms/{}", form.slug), &serde_json::json!({})).await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);

    let json: serde_json::Value = response.json();

    // Check envelope format
    assert_eq!(json["success"], false);
    assert_eq!(json["status"], 422);
    assert!(json["error"].is_object());
    assert!(json["request_id"].is_string());
}

// ============================================================================
// Response Format Consistency
// ============================================================================

#[tokio::test]
async fn test_success_response_has_request_id() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let data = serde_json::json!({
        "name": "Test",
        "email": "test@example.com",
        "message": "Hello"
    });
    let response = app.post_json(&format!("/api/forms/{}", form.slug), &data).await;

    response.assert_status(StatusCode::CREATED);

    let json: serde_json::Value = response.json();
    assert!(json["request_id"].is_string());
    assert!(!json["request_id"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn test_error_response_has_request_id() {
    let app = TestApp::new().await;

    let response = app.get("/api/forms/nonexistent/json").await;

    response.assert_status(StatusCode::NOT_FOUND);

    let json: serde_json::Value = response.json();
    assert!(json["request_id"].is_string());
    assert!(!json["request_id"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn test_error_response_has_correct_status_code() {
    let app = TestApp::new().await;

    let response = app.get("/api/forms/nonexistent/json").await;

    let json: serde_json::Value = response.json();
    assert_eq!(json["status"], 404);
    assert_eq!(response.status.as_u16(), 404);
}

#[cfg(feature = "admin")]
#[tokio::test]
async fn test_api_response_envelope_format() {
    let app = TestApp::with_admin().await;

    let response = app.get("/api/admin/forms").await;

    response.assert_status(StatusCode::OK);

    let json: serde_json::Value = response.json();

    // Check all required envelope fields
    assert!(json.get("data").is_some(), "Missing 'data' field");
    assert!(json.get("success").is_some(), "Missing 'success' field");
    assert!(json.get("status").is_some(), "Missing 'status' field");
    assert!(json.get("request_id").is_some(), "Missing 'request_id' field");

    // Success response should have success=true and no error
    assert_eq!(json["success"], true);
    assert!(json.get("error").is_none() || json["error"].is_null());
}

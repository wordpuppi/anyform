//! Integration tests for public form endpoints.
//!
//! Tests cover:
//! - GET /api/forms/{slug} - HTML rendering
//! - GET /api/forms/{slug}/json - JSON schema
//! - POST /api/forms/{slug} - Form submission (JSON response)
//! - POST /api/forms/{slug}/submit - Form submission (redirect)
//! - GET /api/forms/{slug}/success - Success page

mod common;

use common::{contact_form, create_test_form, TestApp};
use http::StatusCode;

// ============================================================================
// GET /api/forms/{slug}/json - JSON Schema
// ============================================================================

#[tokio::test]
async fn test_get_form_json_success() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let response = app.get(&format!("/api/forms/{}/json", form.slug)).await;

    response.assert_status(StatusCode::OK);
    let json: serde_json::Value = response.json();
    assert_eq!(json["slug"], form.slug);
    assert_eq!(json["name"], form.name);
}

#[tokio::test]
async fn test_get_form_json_not_found() {
    let app = TestApp::new().await;

    let response = app.get("/api/forms/nonexistent-form/json").await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_form_json_deleted_form_returns_404() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    // Soft delete the form
    anyform::FormBuilder::soft_delete(app.db(), form.id)
        .await
        .unwrap();

    // Deleted forms are filtered out by find_by_slug, so they return 404
    let response = app.get(&format!("/api/forms/{}/json", form.slug)).await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_form_json_includes_steps_and_fields() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let response = app.get(&format!("/api/forms/{}/json", form.slug)).await;

    response.assert_status(StatusCode::OK);
    let json: serde_json::Value = response.json();

    // Should have steps array
    assert!(json["steps"].is_array());
    let steps = json["steps"].as_array().unwrap();
    assert!(!steps.is_empty());

    // First step should have fields
    let first_step = &steps[0];
    assert!(first_step["fields"].is_array());
    let fields = first_step["fields"].as_array().unwrap();
    assert!(!fields.is_empty());
}

#[tokio::test]
async fn test_get_form_json_includes_field_details() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let response = app.get(&format!("/api/forms/{}/json", form.slug)).await;

    response.assert_status(StatusCode::OK);
    let json: serde_json::Value = response.json();

    let steps = json["steps"].as_array().unwrap();
    let fields = steps[0]["fields"].as_array().unwrap();
    let field = &fields[0];

    // Field should have required properties
    assert!(field.get("id").is_some());
    assert!(field.get("name").is_some());
    assert!(field.get("label").is_some());
    assert!(field.get("field_type").is_some());
}

// ============================================================================
// GET /api/forms/{slug} - HTML Rendering
// ============================================================================

#[tokio::test]
async fn test_get_form_html_success() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let response = app.get(&format!("/api/forms/{}", form.slug)).await;

    response.assert_status(StatusCode::OK);
    response.assert_content_type("text/html");
    response.assert_body_contains("<form");
}

#[tokio::test]
async fn test_get_form_html_not_found() {
    let app = TestApp::new().await;

    let response = app.get("/api/forms/nonexistent-form").await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_form_html_includes_fields() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let response = app.get(&format!("/api/forms/{}", form.slug)).await;

    response.assert_status(StatusCode::OK);

    let html = response.text();
    // Contact form has name, email, message fields
    assert!(html.contains("name=\"name\"") || html.contains("id=\"name\""));
    assert!(html.contains("name=\"email\"") || html.contains("id=\"email\""));
}

#[tokio::test]
async fn test_get_form_html_deleted_form_returns_404() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    anyform::FormBuilder::soft_delete(app.db(), form.id)
        .await
        .unwrap();

    // Deleted forms are filtered out by find_by_slug, so they return 404
    let response = app.get(&format!("/api/forms/{}", form.slug)).await;

    response.assert_status(StatusCode::NOT_FOUND);
}

// ============================================================================
// POST /api/forms/{slug} - Form Submission (JSON Response)
// ============================================================================

#[tokio::test]
async fn test_submit_form_success() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let submission_data = serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com",
        "message": "Hello!"
    });

    let response = app
        .post_json(&format!("/api/forms/{}", form.slug), &submission_data)
        .await;

    response.assert_status(StatusCode::CREATED);
    response.assert_api_success();

    let json: serde_json::Value = response.json();
    assert!(json["data"]["submission_id"].is_string());
    assert!(json["data"]["message"].is_string());
}

#[tokio::test]
async fn test_submit_form_not_found() {
    let app = TestApp::new().await;

    let submission_data = serde_json::json!({
        "name": "John Doe"
    });

    let response = app
        .post_json("/api/forms/nonexistent-form", &submission_data)
        .await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_submit_form_creates_submission_record() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let submission_data = serde_json::json!({
        "name": "Jane Doe",
        "email": "jane@example.com",
        "message": "Test message"
    });

    let response = app
        .post_json(&format!("/api/forms/{}", form.slug), &submission_data)
        .await;

    response.assert_status(StatusCode::CREATED);

    // Verify submission was created in database
    use anyform::SubmissionEntity;

    let submissions = SubmissionEntity::find_by_form(app.db(), form.id)
        .await
        .unwrap();
    assert_eq!(submissions.len(), 1);
    assert_eq!(submissions[0].data["name"], "Jane Doe");
}

#[tokio::test]
async fn test_submit_form_validation_error_missing_required() {
    let app = TestApp::new().await;

    // Create a form with required fields (name, slug order)
    let input = anyform::CreateFormInput::new("Required Test", "required-test")
        .step(
            anyform::CreateStepInput::new("Step 1")
                .field(anyform::CreateFieldInput::new("required_field", "Required Field", "text").required()),
        );
    let form = create_test_form(app.db(), input).await;

    // Submit without the required field
    let submission_data = serde_json::json!({});

    let response = app
        .post_json(&format!("/api/forms/{}", form.slug), &submission_data)
        .await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    response.assert_api_error("VALIDATION_FAILED");
}

#[tokio::test]
async fn test_submit_form_validation_error_invalid_email() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let submission_data = serde_json::json!({
        "name": "John",
        "email": "not-an-email",
        "message": "Hello"
    });

    let response = app
        .post_json(&format!("/api/forms/{}", form.slug), &submission_data)
        .await;

    // Email validation should fail
    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    response.assert_api_error("VALIDATION_FAILED");
}

#[tokio::test]
async fn test_submit_form_deleted_form() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    anyform::FormBuilder::soft_delete(app.db(), form.id)
        .await
        .unwrap();

    let submission_data = serde_json::json!({
        "name": "John",
        "email": "john@example.com",
        "message": "Hello"
    });

    let response = app
        .post_json(&format!("/api/forms/{}", form.slug), &submission_data)
        .await;

    // Deleted forms are filtered out by find_by_slug, so they return 404
    response.assert_status(StatusCode::NOT_FOUND);
}

// ============================================================================
// POST /api/forms/{slug}/submit - Form Submission (Redirect Response)
// ============================================================================

#[tokio::test]
async fn test_submit_form_redirect_success() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let response = app
        .post_form(
            &format!("/api/forms/{}/submit", form.slug),
            &[
                ("name", "John Doe"),
                ("email", "john@example.com"),
                ("message", "Hello!"),
            ],
        )
        .await;

    // Should redirect to success page
    response.assert_status(StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn test_submit_form_redirect_validation_error_rerenders() {
    let app = TestApp::new().await;

    // Create a form with a required field (name, slug order)
    let input = anyform::CreateFormInput::new("Redirect Test", "redirect-test")
        .step(
            anyform::CreateStepInput::new("Step 1")
                .field(anyform::CreateFieldInput::new("required_field", "Required", "text").required()),
        );
    let form = create_test_form(app.db(), input).await;

    // Submit without the required field
    let response = app
        .post_form(&format!("/api/forms/{}/submit", form.slug), &[])
        .await;

    // Should re-render form with errors (200 OK with HTML)
    response.assert_status(StatusCode::OK);
    response.assert_content_type("text/html");
    response.assert_body_contains("<form");
}

// ============================================================================
// GET /api/forms/{slug}/success - Success Page
// ============================================================================

#[tokio::test]
async fn test_success_page_returns_html() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let response = app
        .get(&format!("/api/forms/{}/success", form.slug))
        .await;

    response.assert_status(StatusCode::OK);
    response.assert_content_type("text/html");
    response.assert_body_contains("Success");
}

#[tokio::test]
async fn test_success_page_not_found() {
    let app = TestApp::new().await;

    let response = app.get("/api/forms/nonexistent-form/success").await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_success_page_includes_default_message() {
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let response = app
        .get(&format!("/api/forms/{}/success", form.slug))
        .await;

    response.assert_status(StatusCode::OK);
    response.assert_body_contains("Thank you");
}

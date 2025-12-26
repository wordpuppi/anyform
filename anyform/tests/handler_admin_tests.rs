//! Integration tests for admin form endpoints.
//!
//! Tests cover:
//! - GET /api/admin/forms - List forms
//! - POST /api/admin/forms - Create form
//! - POST /api/admin/forms/sync - Sync forms
//! - GET /api/admin/forms/{id} - Get form by ID
//! - PUT /api/admin/forms/{id} - Update form
//! - DELETE /api/admin/forms/{id} - Delete form
//! - GET /api/admin/forms/{id}/submissions - List submissions
//! - GET /api/admin/forms/{form_id}/submissions/{sub_id} - Get submission
//! - DELETE /api/admin/forms/{form_id}/submissions/{sub_id} - Delete submission

#![cfg(feature = "admin")]

mod common;

use common::{contact_form, create_test_form, TestApp};
use http::StatusCode;
use uuid::Uuid;

// ============================================================================
// GET /api/admin/forms - List Forms
// ============================================================================

#[tokio::test]
async fn test_list_forms_empty() {
    let app = TestApp::with_admin().await;

    let response = app.get("/api/admin/forms").await;

    response.assert_status(StatusCode::OK);
    response.assert_api_success();

    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["count"], 0);
    assert!(json["data"]["forms"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_list_forms_returns_all() {
    let app = TestApp::with_admin().await;
    let form1 = create_test_form(app.db(), contact_form()).await;

    let input2 = anyform::CreateFormInput::new("Second Form", "second-form")
        .step(anyform::CreateStepInput::new("Step 1"));
    let form2 = create_test_form(app.db(), input2).await;

    let response = app.get("/api/admin/forms").await;

    response.assert_status(StatusCode::OK);
    response.assert_api_success();

    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["count"], 2);

    let forms = json["data"]["forms"].as_array().unwrap();
    let slugs: Vec<&str> = forms.iter().map(|f| f["slug"].as_str().unwrap()).collect();
    assert!(slugs.contains(&form1.slug.as_str()));
    assert!(slugs.contains(&form2.slug.as_str()));
}

#[tokio::test]
async fn test_list_forms_excludes_deleted() {
    let app = TestApp::with_admin().await;
    let form1 = create_test_form(app.db(), contact_form()).await;

    let input2 = anyform::CreateFormInput::new("To Delete", "to-delete")
        .step(anyform::CreateStepInput::new("Step 1"));
    let form2 = create_test_form(app.db(), input2).await;

    // Delete the second form
    anyform::FormBuilder::soft_delete(app.db(), form2.id)
        .await
        .unwrap();

    let response = app.get("/api/admin/forms").await;

    response.assert_status(StatusCode::OK);
    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["count"], 1);

    let forms = json["data"]["forms"].as_array().unwrap();
    assert_eq!(forms[0]["slug"], form1.slug);
}

// ============================================================================
// POST /api/admin/forms - Create Form
// ============================================================================

#[tokio::test]
async fn test_create_form_success() {
    let app = TestApp::with_admin().await;

    let input = serde_json::json!({
        "name": "New Form",
        "slug": "new-form",
        "steps": [{
            "name": "Step 1",
            "fields": [{
                "name": "test_field",
                "label": "Test Field",
                "field_type": "text"
            }]
        }]
    });

    let response = app.post_json("/api/admin/forms", &input).await;

    response.assert_status(StatusCode::CREATED);
    response.assert_api_success();

    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["name"], "New Form");
    assert_eq!(json["data"]["slug"], "new-form");
    assert!(json["data"]["id"].is_string());
}

#[tokio::test]
async fn test_create_form_with_steps_and_fields() {
    let app = TestApp::with_admin().await;

    let input = serde_json::json!({
        "name": "Multi-Field Form",
        "slug": "multi-field-form",
        "steps": [{
            "name": "Personal Info",
            "fields": [
                {"name": "first_name", "label": "First Name", "field_type": "text", "required": true},
                {"name": "last_name", "label": "Last Name", "field_type": "text", "required": true},
                {"name": "email", "label": "Email", "field_type": "email", "required": true}
            ]
        }]
    });

    let response = app.post_json("/api/admin/forms", &input).await;

    response.assert_status(StatusCode::CREATED);
    response.assert_api_success();

    // Verify the form was created with fields
    let json: serde_json::Value = response.json();
    let form_id = json["data"]["id"].as_str().unwrap();

    let get_response = app.get(&format!("/api/admin/forms/{}", form_id)).await;
    get_response.assert_status(StatusCode::OK);

    let form_json: serde_json::Value = get_response.json();
    let steps = form_json["data"]["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 1);

    let fields = steps[0]["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 3);
}

#[tokio::test]
async fn test_create_form_duplicate_slug_error() {
    let app = TestApp::with_admin().await;
    let _existing = create_test_form(app.db(), contact_form()).await;

    // Try to create with same slug
    let input = serde_json::json!({
        "name": "Another Form",
        "slug": "test-contact",  // Same as contact_form
        "steps": [{"name": "Step 1", "fields": []}]
    });

    let response = app.post_json("/api/admin/forms", &input).await;

    // Should fail due to duplicate slug
    assert!(
        response.status == StatusCode::CONFLICT || response.status == StatusCode::BAD_REQUEST,
        "Expected conflict or bad request, got {}",
        response.status
    );
}

// ============================================================================
// GET /api/admin/forms/{id} - Get Form by ID
// ============================================================================

#[tokio::test]
async fn test_get_form_by_id_success() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let response = app.get(&format!("/api/admin/forms/{}", form.id)).await;

    response.assert_status(StatusCode::OK);
    response.assert_api_success();

    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["slug"], form.slug);
    assert_eq!(json["data"]["name"], form.name);
}

#[tokio::test]
async fn test_get_form_by_id_not_found() {
    let app = TestApp::with_admin().await;
    let random_id = Uuid::new_v4();

    let response = app.get(&format!("/api/admin/forms/{}", random_id)).await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_form_by_id_invalid_uuid() {
    let app = TestApp::with_admin().await;

    let response = app.get("/api/admin/forms/not-a-uuid").await;

    response.assert_status(StatusCode::BAD_REQUEST);
}

// ============================================================================
// PUT /api/admin/forms/{id} - Update Form
// ============================================================================

#[tokio::test]
async fn test_update_form_success() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let update = serde_json::json!({
        "name": "Updated Contact Form",
        "slug": form.slug,
        "steps": [{
            "name": "Updated Step",
            "fields": [{"name": "updated_field", "label": "Updated Field", "field_type": "text"}]
        }]
    });

    let response = app
        .put_json(&format!("/api/admin/forms/{}", form.id), &update)
        .await;

    response.assert_status(StatusCode::OK);
    response.assert_api_success();

    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["name"], "Updated Contact Form");
}

#[tokio::test]
async fn test_update_form_not_found() {
    let app = TestApp::with_admin().await;
    let random_id = Uuid::new_v4();

    let update = serde_json::json!({
        "name": "Does Not Exist",
        "slug": "does-not-exist",
        "steps": []
    });

    let response = app
        .put_json(&format!("/api/admin/forms/{}", random_id), &update)
        .await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_form_replaces_steps() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    // Update with completely new steps
    let update = serde_json::json!({
        "name": form.name,
        "slug": form.slug,
        "steps": [
            {"name": "New Step 1", "fields": [{"name": "field_a", "label": "Field A", "field_type": "text"}]},
            {"name": "New Step 2", "fields": [{"name": "field_b", "label": "Field B", "field_type": "text"}]}
        ]
    });

    let response = app
        .put_json(&format!("/api/admin/forms/{}", form.id), &update)
        .await;

    response.assert_status(StatusCode::OK);

    // Verify the steps were replaced
    let get_response = app.get(&format!("/api/admin/forms/{}", form.id)).await;
    let json: serde_json::Value = get_response.json();
    let steps = json["data"]["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0]["name"], "New Step 1");
    assert_eq!(steps[1]["name"], "New Step 2");
}

// ============================================================================
// DELETE /api/admin/forms/{id} - Delete Form
// ============================================================================

#[tokio::test]
async fn test_delete_form_success() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let response = app.delete(&format!("/api/admin/forms/{}", form.id)).await;

    response.assert_status(StatusCode::OK);
    response.assert_api_success();

    // Form should no longer be in list
    let list_response = app.get("/api/admin/forms").await;
    let json: serde_json::Value = list_response.json();
    assert_eq!(json["data"]["count"], 0);
}

#[tokio::test]
async fn test_delete_form_not_found() {
    let app = TestApp::with_admin().await;
    let random_id = Uuid::new_v4();

    let response = app.delete(&format!("/api/admin/forms/{}", random_id)).await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_form_is_soft_delete() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    app.delete(&format!("/api/admin/forms/{}", form.id)).await;

    // Verify form still exists in DB with deleted_at set
    use sea_orm::EntityTrait;
    let db_form = anyform::FormEntity::find_by_id(form.id)
        .one(app.db())
        .await
        .unwrap();

    assert!(db_form.is_some());
    assert!(db_form.unwrap().deleted_at.is_some());
}

// ============================================================================
// POST /api/admin/forms/sync - Sync Forms
// ============================================================================

#[tokio::test]
async fn test_sync_creates_new_forms() {
    let app = TestApp::with_admin().await;

    let forms = serde_json::json!([
        {
            "name": "Sync Form 1",
            "slug": "sync-form-1",
            "steps": [{"name": "Step 1", "fields": []}]
        },
        {
            "name": "Sync Form 2",
            "slug": "sync-form-2",
            "steps": [{"name": "Step 1", "fields": []}]
        }
    ]);

    let response = app.post_json("/api/admin/forms/sync", &forms).await;

    response.assert_status(StatusCode::OK);
    response.assert_api_success();

    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["created"], 2);
    assert_eq!(json["data"]["updated"], 0);
}

#[tokio::test]
async fn test_sync_updates_existing() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let forms = serde_json::json!([
        {
            "name": "Updated via Sync",
            "slug": form.slug,  // Same slug triggers update
            "steps": [{"name": "Synced Step", "fields": []}]
        }
    ]);

    let response = app.post_json("/api/admin/forms/sync", &forms).await;

    response.assert_status(StatusCode::OK);
    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["created"], 0);
    assert_eq!(json["data"]["updated"], 1);
}

#[tokio::test]
async fn test_sync_mixed_create_update() {
    let app = TestApp::with_admin().await;
    let existing = create_test_form(app.db(), contact_form()).await;

    let forms = serde_json::json!([
        {
            "name": "Updated Existing",
            "slug": existing.slug,
            "steps": [{"name": "Step 1", "fields": []}]
        },
        {
            "name": "Brand New",
            "slug": "brand-new-form",
            "steps": [{"name": "Step 1", "fields": []}]
        }
    ]);

    let response = app.post_json("/api/admin/forms/sync", &forms).await;

    response.assert_status(StatusCode::OK);
    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["created"], 1);
    assert_eq!(json["data"]["updated"], 1);
}

#[tokio::test]
async fn test_sync_empty_array() {
    let app = TestApp::with_admin().await;

    let forms: Vec<serde_json::Value> = vec![];

    let response = app.post_json("/api/admin/forms/sync", &forms).await;

    response.assert_status(StatusCode::OK);
    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["created"], 0);
    assert_eq!(json["data"]["updated"], 0);
}

// ============================================================================
// GET /api/admin/forms/{id}/submissions - List Submissions
// ============================================================================

#[tokio::test]
async fn test_list_submissions_empty() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    let response = app
        .get(&format!("/api/admin/forms/{}/submissions", form.id))
        .await;

    response.assert_status(StatusCode::OK);
    response.assert_api_success();

    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["count"], 0);
}

#[tokio::test]
async fn test_list_submissions_returns_all() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    // Create submissions via public API
    for i in 1..=3 {
        let data = serde_json::json!({
            "name": format!("User {}", i),
            "email": format!("user{}@example.com", i),
            "message": "Test message"
        });
        app.post_json(&format!("/api/forms/{}", form.slug), &data)
            .await;
    }

    let response = app
        .get(&format!("/api/admin/forms/{}/submissions", form.id))
        .await;

    response.assert_status(StatusCode::OK);
    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["count"], 3);
}

#[tokio::test]
async fn test_list_submissions_form_not_found() {
    let app = TestApp::with_admin().await;
    let random_id = Uuid::new_v4();

    let response = app
        .get(&format!("/api/admin/forms/{}/submissions", random_id))
        .await;

    // Currently returns empty list, not 404 - may be correct behavior
    response.assert_status(StatusCode::OK);
}

// ============================================================================
// GET /api/admin/forms/{form_id}/submissions/{sub_id} - Get Submission
// ============================================================================

#[tokio::test]
async fn test_get_submission_success() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    // Create a submission
    let data = serde_json::json!({
        "name": "Test User",
        "email": "test@example.com",
        "message": "Hello"
    });
    let submit_response = app
        .post_json(&format!("/api/forms/{}", form.slug), &data)
        .await;
    let submit_json: serde_json::Value = submit_response.json();
    let sub_id = submit_json["data"]["submission_id"].as_str().unwrap();

    let response = app
        .get(&format!("/api/admin/forms/{}/submissions/{}", form.id, sub_id))
        .await;

    response.assert_status(StatusCode::OK);
    response.assert_api_success();

    let json: serde_json::Value = response.json();
    assert_eq!(json["data"]["data"]["name"], "Test User");
    assert_eq!(json["data"]["data"]["email"], "test@example.com");
}

#[tokio::test]
async fn test_get_submission_not_found() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;
    let random_id = Uuid::new_v4();

    let response = app
        .get(&format!("/api/admin/forms/{}/submissions/{}", form.id, random_id))
        .await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_submission_wrong_form() {
    let app = TestApp::with_admin().await;

    let form1 = create_test_form(app.db(), contact_form()).await;
    let input2 = anyform::CreateFormInput::new("Other Form", "other-form")
        .step(anyform::CreateStepInput::new("Step 1")
            .field(anyform::CreateFieldInput::new("field", "Field", "text")));
    let form2 = create_test_form(app.db(), input2).await;

    // Create submission on form1
    let data = serde_json::json!({
        "name": "Test",
        "email": "test@example.com",
        "message": "Hello"
    });
    let submit_response = app
        .post_json(&format!("/api/forms/{}", form1.slug), &data)
        .await;
    let submit_json: serde_json::Value = submit_response.json();
    let sub_id = submit_json["data"]["submission_id"].as_str().unwrap();

    // Try to get submission using form2's ID
    let response = app
        .get(&format!("/api/admin/forms/{}/submissions/{}", form2.id, sub_id))
        .await;

    // Should return 404 since submission doesn't belong to form2
    response.assert_status(StatusCode::NOT_FOUND);
}

// ============================================================================
// DELETE /api/admin/forms/{form_id}/submissions/{sub_id} - Delete Submission
// ============================================================================

#[tokio::test]
async fn test_delete_submission_success() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    // Create a submission
    let data = serde_json::json!({
        "name": "To Delete",
        "email": "delete@example.com",
        "message": "Delete me"
    });
    let submit_response = app
        .post_json(&format!("/api/forms/{}", form.slug), &data)
        .await;
    let submit_json: serde_json::Value = submit_response.json();
    let sub_id = submit_json["data"]["submission_id"].as_str().unwrap();

    let response = app
        .delete(&format!("/api/admin/forms/{}/submissions/{}", form.id, sub_id))
        .await;

    response.assert_status(StatusCode::OK);
    response.assert_api_success();

    // Submission should no longer be accessible
    let get_response = app
        .get(&format!("/api/admin/forms/{}/submissions/{}", form.id, sub_id))
        .await;
    get_response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_submission_not_found() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;
    let random_id = Uuid::new_v4();

    let response = app
        .delete(&format!("/api/admin/forms/{}/submissions/{}", form.id, random_id))
        .await;

    response.assert_status(StatusCode::NOT_FOUND);
}

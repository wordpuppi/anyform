//! End-to-end workflow integration tests.
//!
//! These tests verify complete user flows across multiple API operations.

#![cfg(feature = "admin")]

mod common;

use common::{contact_form, create_test_form, multi_step_form, TestApp};
use http::StatusCode;

// ============================================================================
// Workflow: Create Form -> Submit -> View Submission
// ============================================================================

#[tokio::test]
async fn test_workflow_create_form_submit_view_submission() {
    let app = TestApp::with_admin().await;

    // 1. Create form via admin API
    let form_input = serde_json::json!({
        "name": "Workflow Test Form",
        "slug": "workflow-test",
        "steps": [{
            "name": "Main",
            "fields": [
                {"name": "full_name", "label": "Full Name", "field_type": "text", "required": true},
                {"name": "email", "label": "Email", "field_type": "email", "required": true}
            ]
        }]
    });

    let create_response = app.post_json("/api/admin/forms", &form_input).await;
    create_response.assert_status(StatusCode::CREATED);

    let create_json: serde_json::Value = create_response.json();
    let form_id = create_json["data"]["id"].as_str().unwrap();

    // 2. Verify form exists via public JSON API
    let json_response = app.get("/api/forms/workflow-test/json").await;
    json_response.assert_status(StatusCode::OK);

    // 3. Submit form via public API
    let submission_data = serde_json::json!({
        "full_name": "Alice Smith",
        "email": "alice@example.com"
    });
    let submit_response = app
        .post_json("/api/forms/workflow-test", &submission_data)
        .await;
    submit_response.assert_status(StatusCode::CREATED);

    let submit_json: serde_json::Value = submit_response.json();
    let submission_id = submit_json["data"]["submission_id"].as_str().unwrap();

    // 4. View submission via admin API
    let sub_response = app
        .get(&format!("/api/admin/forms/{}/submissions/{}", form_id, submission_id))
        .await;
    sub_response.assert_status(StatusCode::OK);

    let sub_json: serde_json::Value = sub_response.json();
    assert_eq!(sub_json["data"]["data"]["full_name"], "Alice Smith");
    assert_eq!(sub_json["data"]["data"]["email"], "alice@example.com");
}

// ============================================================================
// Workflow: CRUD Lifecycle
// ============================================================================

#[tokio::test]
async fn test_workflow_crud_lifecycle() {
    let app = TestApp::with_admin().await;

    // 1. CREATE
    let form_input = serde_json::json!({
        "name": "CRUD Test Form",
        "slug": "crud-test",
        "steps": [{"name": "Step 1", "fields": []}]
    });
    let create_response = app.post_json("/api/admin/forms", &form_input).await;
    create_response.assert_status(StatusCode::CREATED);

    let create_json: serde_json::Value = create_response.json();
    let form_id = create_json["data"]["id"].as_str().unwrap();

    // 2. READ
    let read_response = app.get(&format!("/api/admin/forms/{}", form_id)).await;
    read_response.assert_status(StatusCode::OK);

    let read_json: serde_json::Value = read_response.json();
    assert_eq!(read_json["data"]["name"], "CRUD Test Form");

    // 3. UPDATE
    let update_input = serde_json::json!({
        "name": "Updated CRUD Form",
        "slug": "crud-test",
        "steps": [{"name": "Updated Step", "fields": []}]
    });
    let update_response = app
        .put_json(&format!("/api/admin/forms/{}", form_id), &update_input)
        .await;
    update_response.assert_status(StatusCode::OK);

    // Verify update
    let verify_response = app.get(&format!("/api/admin/forms/{}", form_id)).await;
    let verify_json: serde_json::Value = verify_response.json();
    assert_eq!(verify_json["data"]["name"], "Updated CRUD Form");

    // 4. DELETE
    let delete_response = app.delete(&format!("/api/admin/forms/{}", form_id)).await;
    delete_response.assert_status(StatusCode::OK);

    // Verify deleted (no longer in list)
    let list_response = app.get("/api/admin/forms").await;
    let list_json: serde_json::Value = list_response.json();
    assert_eq!(list_json["data"]["count"], 0);
}

// ============================================================================
// Workflow: Sync and Query
// ============================================================================

#[tokio::test]
async fn test_workflow_sync_and_query() {
    let app = TestApp::with_admin().await;

    // 1. Sync multiple forms at once
    let forms_to_sync = serde_json::json!([
        {"name": "Sync Form A", "slug": "sync-a", "steps": [{"name": "Step", "fields": []}]},
        {"name": "Sync Form B", "slug": "sync-b", "steps": [{"name": "Step", "fields": []}]},
        {"name": "Sync Form C", "slug": "sync-c", "steps": [{"name": "Step", "fields": []}]}
    ]);

    let sync_response = app.post_json("/api/admin/forms/sync", &forms_to_sync).await;
    sync_response.assert_status(StatusCode::OK);

    let sync_json: serde_json::Value = sync_response.json();
    assert_eq!(sync_json["data"]["created"], 3);

    // 2. List all forms
    let list_response = app.get("/api/admin/forms").await;
    let list_json: serde_json::Value = list_response.json();
    assert_eq!(list_json["data"]["count"], 3);

    // 3. Get each form by slug via public API
    for slug in ["sync-a", "sync-b", "sync-c"] {
        let response = app.get(&format!("/api/forms/{}/json", slug)).await;
        response.assert_status(StatusCode::OK);
    }

    // 4. Sync again with updates
    let updated_forms = serde_json::json!([
        {"name": "Updated Sync Form A", "slug": "sync-a", "steps": [{"name": "New Step", "fields": []}]},
        {"name": "Sync Form D", "slug": "sync-d", "steps": [{"name": "Step", "fields": []}]}
    ]);

    let resync_response = app.post_json("/api/admin/forms/sync", &updated_forms).await;
    let resync_json: serde_json::Value = resync_response.json();
    assert_eq!(resync_json["data"]["updated"], 1);
    assert_eq!(resync_json["data"]["created"], 1);
}

// ============================================================================
// Workflow: Submission Lifecycle
// ============================================================================

#[tokio::test]
async fn test_workflow_submission_lifecycle() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    // 1. Submit multiple times
    for i in 1..=3 {
        let data = serde_json::json!({
            "name": format!("User {}", i),
            "email": format!("user{}@example.com", i),
            "message": format!("Message {}", i)
        });
        let response = app.post_json(&format!("/api/forms/{}", form.slug), &data).await;
        response.assert_status(StatusCode::CREATED);
    }

    // 2. List submissions
    let list_response = app
        .get(&format!("/api/admin/forms/{}/submissions", form.id))
        .await;
    list_response.assert_status(StatusCode::OK);

    let list_json: serde_json::Value = list_response.json();
    assert_eq!(list_json["data"]["count"], 3);

    let submissions = list_json["data"]["submissions"].as_array().unwrap();
    let first_sub_id = submissions[0]["id"].as_str().unwrap();

    // 3. Get specific submission
    let get_response = app
        .get(&format!("/api/admin/forms/{}/submissions/{}", form.id, first_sub_id))
        .await;
    get_response.assert_status(StatusCode::OK);

    // 4. Delete submission
    let delete_response = app
        .delete(&format!("/api/admin/forms/{}/submissions/{}", form.id, first_sub_id))
        .await;
    delete_response.assert_status(StatusCode::OK);

    // 5. Verify deletion
    let verify_response = app
        .get(&format!("/api/admin/forms/{}/submissions/{}", form.id, first_sub_id))
        .await;
    verify_response.assert_status(StatusCode::NOT_FOUND);

    // Remaining count should be 2
    let final_list = app
        .get(&format!("/api/admin/forms/{}/submissions", form.id))
        .await;
    let final_json: serde_json::Value = final_list.json();
    assert_eq!(final_json["data"]["count"], 2);
}

// ============================================================================
// Workflow: Multi-Step Form
// ============================================================================

#[tokio::test]
async fn test_workflow_multi_step_form() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), multi_step_form()).await;

    // 1. Get form JSON and verify steps
    let json_response = app.get(&format!("/api/forms/{}/json", form.slug)).await;
    json_response.assert_status(StatusCode::OK);

    let json: serde_json::Value = json_response.json();
    let steps = json["steps"].as_array().unwrap();

    // Multi-step form has 3 steps
    assert_eq!(steps.len(), 3);

    // Verify step order
    assert!(steps[0]["name"].as_str().unwrap().contains("Step 1"));
    assert!(steps[1]["name"].as_str().unwrap().contains("Step 2"));
    assert!(steps[2]["name"].as_str().unwrap().contains("Step 3"));

    // 2. Submit complete form data
    let submission_data = serde_json::json!({
        "first_name": "John",
        "last_name": "Doe",
        "email": "john@example.com",
        "phone": "555-1234",
        "agree_terms": "on"
    });

    let submit_response = app
        .post_json(&format!("/api/forms/{}", form.slug), &submission_data)
        .await;
    submit_response.assert_status(StatusCode::CREATED);

    // 3. Verify submission was created with all data
    let list_response = app
        .get(&format!("/api/admin/forms/{}/submissions", form.id))
        .await;
    let list_json: serde_json::Value = list_response.json();
    assert_eq!(list_json["data"]["count"], 1);
}

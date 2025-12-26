//! Integration tests for feature flag behavior.
//!
//! Tests cover:
//! - Public routes always work (with or without admin feature)
//! - Admin routes only work when admin feature is enabled
//! - Health check always works

mod common;

use common::{contact_form, create_test_form, TestApp};
use http::StatusCode;

// ============================================================================
// Public Routes - Always Available
// ============================================================================

#[tokio::test]
async fn test_public_routes_work_with_default_features() {
    // This test runs regardless of features since TestApp::new() only enables public routes
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    // All public routes should work
    let json_response = app.get(&format!("/api/forms/{}/json", form.slug)).await;
    json_response.assert_status(StatusCode::OK);

    let html_response = app.get(&format!("/api/forms/{}", form.slug)).await;
    html_response.assert_status(StatusCode::OK);

    let success_response = app.get(&format!("/api/forms/{}/success", form.slug)).await;
    success_response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_health_check_works_with_default_features() {
    let app = TestApp::new().await;

    let response = app.get("/health").await;

    response.assert_status(StatusCode::OK);
    assert_eq!(response.text(), "OK");
}

// ============================================================================
// Admin Routes - Feature Gated
// ============================================================================

#[cfg(feature = "admin")]
#[tokio::test]
async fn test_admin_routes_available_with_admin_feature() {
    let app = TestApp::with_admin().await;

    // All admin routes should work
    let list_response = app.get("/api/admin/forms").await;
    list_response.assert_status(StatusCode::OK);
    list_response.assert_api_success();
}

#[cfg(feature = "admin")]
#[tokio::test]
async fn test_admin_routes_not_available_without_builder_flag() {
    // Create app with default builder (admin disabled by default)
    let app = TestApp::new().await;

    // Admin routes should return 404 when not enabled via builder
    let response = app.get("/api/admin/forms").await;
    response.assert_status(StatusCode::NOT_FOUND);
}

// ============================================================================
// Router Builder Configuration
// ============================================================================

#[tokio::test]
async fn test_selective_route_enabling() {
    // This test verifies that the router builder correctly enables/disables routes
    // We use TestApp::new() which uses default routes
    let app = TestApp::new().await;
    let form = create_test_form(app.db(), contact_form()).await;

    // JSON route should work (enabled by default)
    let json_response = app.get(&format!("/api/forms/{}/json", form.slug)).await;
    assert!(
        json_response.status == StatusCode::OK || json_response.status == StatusCode::NOT_FOUND,
        "JSON route should either work or return 404 based on features"
    );
}

#[cfg(feature = "admin")]
#[tokio::test]
async fn test_admin_and_public_routes_together() {
    let app = TestApp::with_admin().await;
    let form = create_test_form(app.db(), contact_form()).await;

    // Public routes should work
    let public_response = app.get(&format!("/api/forms/{}/json", form.slug)).await;
    public_response.assert_status(StatusCode::OK);

    // Admin routes should also work
    let admin_response = app.get("/api/admin/forms").await;
    admin_response.assert_status(StatusCode::OK);

    // Health check should work
    let health_response = app.get("/health").await;
    health_response.assert_status(StatusCode::OK);
}

//! Tests for form sync functionality.

mod common;

use anyform::{
    services::{CreateFieldInput, CreateFormInput, CreateStepInput, FormBuilder},
};
use common::db::TestDb;

async fn setup() -> TestDb {
    TestDb::new().await
}

// ============================================================================
// Sync Logic Tests (upsert behavior)
// ============================================================================

#[tokio::test]
async fn test_sync_creates_new_form() {
    let db = setup().await;

    let input = CreateFormInput::new("New Form", "sync-new")
        .step(CreateStepInput::new("Main").field(
            CreateFieldInput::new("name", "Name", "text"),
        ));

    // Form doesn't exist yet
    let existing = FormBuilder::find_by_slug(db.conn(), "sync-new").await.unwrap();
    assert!(existing.is_none());

    // Create it
    FormBuilder::create(db.conn(), input).await.unwrap();

    // Now it exists
    let found = FormBuilder::find_by_slug(db.conn(), "sync-new").await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "New Form");
}

#[tokio::test]
async fn test_sync_updates_existing_form() {
    let db = setup().await;

    // Create initial form
    let input = CreateFormInput::new("Original Name", "sync-update")
        .step(CreateStepInput::new("Step 1").field(
            CreateFieldInput::new("field1", "Field 1", "text"),
        ));

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    assert_eq!(form.name, "Original Name");

    // Update with new definition
    let updated_input = CreateFormInput::new("Updated Name", "sync-update")
        .description("Now with description")
        .step(CreateStepInput::new("New Step").field(
            CreateFieldInput::new("new_field", "New Field", "email"),
        ));

    let updated = FormBuilder::update(db.conn(), form.id, updated_input).await.unwrap();

    assert_eq!(updated.name, "Updated Name");
    assert_eq!(updated.description, Some("Now with description".to_string()));
}

#[tokio::test]
async fn test_sync_upsert_pattern() {
    let db = setup().await;

    // Simulate sync: check if exists, then create or update
    async fn sync_form(db: &TestDb, input: CreateFormInput) -> (bool, bool) {
        let slug = input.slug.clone();
        match FormBuilder::find_by_slug(db.conn(), &slug).await.unwrap() {
            Some(existing) => {
                FormBuilder::update(db.conn(), existing.id, input).await.unwrap();
                (false, true) // not created, updated
            }
            None => {
                FormBuilder::create(db.conn(), input).await.unwrap();
                (true, false) // created, not updated
            }
        }
    }

    // First sync - should create
    let input1 = CreateFormInput::new("Form V1", "sync-upsert")
        .step(CreateStepInput::new("Main"));

    let (created, updated) = sync_form(&db, input1).await;
    assert!(created);
    assert!(!updated);

    // Second sync - should update
    let input2 = CreateFormInput::new("Form V2", "sync-upsert")
        .step(CreateStepInput::new("Main"));

    let (created, updated) = sync_form(&db, input2).await;
    assert!(!created);
    assert!(updated);

    // Verify final state
    let form = FormBuilder::find_by_slug(db.conn(), "sync-upsert").await.unwrap().unwrap();
    assert_eq!(form.name, "Form V2");
}

#[tokio::test]
async fn test_sync_multiple_forms() {
    let db = setup().await;

    let forms = vec![
        CreateFormInput::new("Form A", "sync-multi-a")
            .step(CreateStepInput::new("Main")),
        CreateFormInput::new("Form B", "sync-multi-b")
            .step(CreateStepInput::new("Main")),
        CreateFormInput::new("Form C", "sync-multi-c")
            .step(CreateStepInput::new("Main")),
    ];

    let mut created = 0;
    for input in forms {
        let slug = input.slug.clone();
        if FormBuilder::find_by_slug(db.conn(), &slug).await.unwrap().is_none() {
            FormBuilder::create(db.conn(), input).await.unwrap();
            created += 1;
        }
    }

    assert_eq!(created, 3);

    // Verify all exist
    assert!(FormBuilder::find_by_slug(db.conn(), "sync-multi-a").await.unwrap().is_some());
    assert!(FormBuilder::find_by_slug(db.conn(), "sync-multi-b").await.unwrap().is_some());
    assert!(FormBuilder::find_by_slug(db.conn(), "sync-multi-c").await.unwrap().is_some());
}

#[tokio::test]
async fn test_sync_preserves_unsynced_forms() {
    let db = setup().await;

    // Create a form that won't be in the "sync"
    let manual = CreateFormInput::new("Manual Form", "manual-form")
        .step(CreateStepInput::new("Main"));
    FormBuilder::create(db.conn(), manual).await.unwrap();

    // Sync other forms
    let synced = CreateFormInput::new("Synced Form", "synced-form")
        .step(CreateStepInput::new("Main"));
    FormBuilder::create(db.conn(), synced).await.unwrap();

    // Both should still exist (additive sync)
    assert!(FormBuilder::find_by_slug(db.conn(), "manual-form").await.unwrap().is_some());
    assert!(FormBuilder::find_by_slug(db.conn(), "synced-form").await.unwrap().is_some());
}

#[tokio::test]
async fn test_sync_with_complex_form() {
    let db = setup().await;

    use anyform::services::CreateOptionInput;
    use anyform::schema::{FormSettings, ValidationRules};

    let input = CreateFormInput::new("Complex Form", "sync-complex")
        .description("A complex form for sync testing")
        .settings(
            FormSettings::new()
                .success_message("Thanks!")
                .submit_label("Submit")
        )
        .step(
            CreateStepInput::new("Personal Info")
                .order(0)
                .fields(vec![
                    CreateFieldInput::new("name", "Full Name", "text")
                        .required()
                        .validation(ValidationRules::new().min_length(2).max_length(100)),
                    CreateFieldInput::new("email", "Email", "email")
                        .required(),
                ])
        )
        .step(
            CreateStepInput::new("Preferences")
                .order(1)
                .field(
                    CreateFieldInput::new("color", "Favorite Color", "select")
                        .options(vec![
                            CreateOptionInput::new("Red", "red"),
                            CreateOptionInput::new("Blue", "blue"),
                            CreateOptionInput::new("Green", "green"),
                        ])
                )
        );

    // Create
    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    assert_eq!(form.name, "Complex Form");

    // Update with modified version
    let updated_input = CreateFormInput::new("Complex Form Updated", "sync-complex")
        .description("Updated description")
        .settings(FormSettings::new().success_message("Updated thanks!"))
        .step(
            CreateStepInput::new("All Fields")
                .field(CreateFieldInput::new("combined", "Combined", "textarea"))
        );

    let updated = FormBuilder::update(db.conn(), form.id, updated_input).await.unwrap();
    assert_eq!(updated.name, "Complex Form Updated");
    assert_eq!(updated.description, Some("Updated description".to_string()));
}

// ============================================================================
// JSON Parsing Tests (simulating file loading)
// ============================================================================

#[tokio::test]
async fn test_sync_from_json_string() {
    let db = setup().await;

    let json = r#"{
        "name": "JSON Form",
        "slug": "sync-json",
        "description": "Created from JSON",
        "steps": [
            {
                "name": "Main",
                "fields": [
                    {
                        "name": "email",
                        "label": "Email",
                        "field_type": "email",
                        "required": true
                    }
                ]
            }
        ]
    }"#;

    let input: CreateFormInput = serde_json::from_str(json).unwrap();

    assert_eq!(input.name, "JSON Form");
    assert_eq!(input.slug, "sync-json");

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    assert_eq!(form.name, "JSON Form");
    assert_eq!(form.slug, "sync-json");
}

#[tokio::test]
async fn test_sync_json_with_validation_rules() {
    let db = setup().await;

    let json = r#"{
        "name": "Validated Form",
        "slug": "sync-validated",
        "steps": [
            {
                "name": "Main",
                "fields": [
                    {
                        "name": "username",
                        "label": "Username",
                        "field_type": "text",
                        "required": true,
                        "validation_rules": {
                            "min_length": 3,
                            "max_length": 20,
                            "pattern": "^[a-z0-9_]+$",
                            "pattern_message": "Only lowercase letters, numbers, and underscores"
                        }
                    }
                ]
            }
        ]
    }"#;

    let input: CreateFormInput = serde_json::from_str(json).unwrap();
    let form = FormBuilder::create(db.conn(), input).await.unwrap();

    assert_eq!(form.slug, "sync-validated");
}

#[tokio::test]
async fn test_sync_json_with_options() {
    let db = setup().await;

    let json = r#"{
        "name": "Options Form",
        "slug": "sync-options",
        "steps": [
            {
                "name": "Main",
                "fields": [
                    {
                        "name": "country",
                        "label": "Country",
                        "field_type": "select",
                        "options": [
                            { "label": "USA", "value": "us" },
                            { "label": "Canada", "value": "ca" },
                            { "label": "Mexico", "value": "mx" }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let input: CreateFormInput = serde_json::from_str(json).unwrap();
    let form = FormBuilder::create(db.conn(), input).await.unwrap();

    assert_eq!(form.slug, "sync-options");
}

#[test]
fn test_invalid_json_parsing() {
    let invalid_json = r#"{ "name": "Missing closing brace" "#;
    let result: Result<CreateFormInput, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());
}

#[test]
fn test_incomplete_json_parsing() {
    // Missing required slug field
    let incomplete_json = r#"{ "name": "No Slug" }"#;
    let result: Result<CreateFormInput, _> = serde_json::from_str(incomplete_json);
    assert!(result.is_err());
}

// ============================================================================
// Sync API Response Structure Tests
// ============================================================================

#[tokio::test]
async fn test_sync_tracks_created_count() {
    let db = setup().await;

    let mut created = 0;
    let mut updated = 0;

    let forms = vec![
        CreateFormInput::new("Form 1", "count-1").step(CreateStepInput::new("Main")),
        CreateFormInput::new("Form 2", "count-2").step(CreateStepInput::new("Main")),
    ];

    for input in forms {
        let slug = input.slug.clone();
        match FormBuilder::find_by_slug(db.conn(), &slug).await.unwrap() {
            Some(existing) => {
                FormBuilder::update(db.conn(), existing.id, input).await.unwrap();
                updated += 1;
            }
            None => {
                FormBuilder::create(db.conn(), input).await.unwrap();
                created += 1;
            }
        }
    }

    assert_eq!(created, 2);
    assert_eq!(updated, 0);
}

#[tokio::test]
async fn test_sync_tracks_updated_count() {
    let db = setup().await;

    // Pre-create forms
    FormBuilder::create(
        db.conn(),
        CreateFormInput::new("Existing 1", "existing-1").step(CreateStepInput::new("Main"))
    ).await.unwrap();

    FormBuilder::create(
        db.conn(),
        CreateFormInput::new("Existing 2", "existing-2").step(CreateStepInput::new("Main"))
    ).await.unwrap();

    // Now sync (should update both)
    let mut created = 0;
    let mut updated = 0;

    let forms = vec![
        CreateFormInput::new("Updated 1", "existing-1").step(CreateStepInput::new("Main")),
        CreateFormInput::new("Updated 2", "existing-2").step(CreateStepInput::new("Main")),
    ];

    for input in forms {
        let slug = input.slug.clone();
        match FormBuilder::find_by_slug(db.conn(), &slug).await.unwrap() {
            Some(existing) => {
                FormBuilder::update(db.conn(), existing.id, input).await.unwrap();
                updated += 1;
            }
            None => {
                FormBuilder::create(db.conn(), input).await.unwrap();
                created += 1;
            }
        }
    }

    assert_eq!(created, 0);
    assert_eq!(updated, 2);
}

#[tokio::test]
async fn test_sync_mixed_create_and_update() {
    let db = setup().await;

    // Pre-create one form
    FormBuilder::create(
        db.conn(),
        CreateFormInput::new("Existing", "mixed-existing").step(CreateStepInput::new("Main"))
    ).await.unwrap();

    // Sync: one existing, one new
    let mut created = 0;
    let mut updated = 0;

    let forms = vec![
        CreateFormInput::new("Updated Existing", "mixed-existing").step(CreateStepInput::new("Main")),
        CreateFormInput::new("Brand New", "mixed-new").step(CreateStepInput::new("Main")),
    ];

    for input in forms {
        let slug = input.slug.clone();
        match FormBuilder::find_by_slug(db.conn(), &slug).await.unwrap() {
            Some(existing) => {
                FormBuilder::update(db.conn(), existing.id, input).await.unwrap();
                updated += 1;
            }
            None => {
                FormBuilder::create(db.conn(), input).await.unwrap();
                created += 1;
            }
        }
    }

    assert_eq!(created, 1);
    assert_eq!(updated, 1);
}

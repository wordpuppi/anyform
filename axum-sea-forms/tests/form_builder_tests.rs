//! Tests for the FormBuilder service.

mod common;

use axum_sea_forms::{
    entities::{
        field::Entity as FieldEntity,
        field_option::Entity as FieldOptionEntity,
        form::Entity as FormEntity,
        step::Entity as StepEntity,
    },
    services::{CreateFieldInput, CreateFormInput, CreateOptionInput, CreateStepInput, FormBuilder},
    schema::{FormSettings, ValidationRules},
};
use common::db::TestDb;
use sea_orm::EntityTrait;

async fn setup() -> TestDb {
    TestDb::new().await
}

// ============================================================================
// Form Creation
// ============================================================================

#[tokio::test]
async fn test_create_simple_form() {
    let db = setup().await;

    let input = CreateFormInput::new("Simple Form", "simple-form")
        .description("A simple test form");

    let form = FormBuilder::create(db.conn(), input).await.unwrap();

    assert_eq!(form.name, "Simple Form");
    assert_eq!(form.slug, "simple-form");
    assert_eq!(form.description, Some("A simple test form".to_string()));
    assert!(form.deleted_at.is_none());
}

#[tokio::test]
async fn test_create_form_with_settings() {
    let db = setup().await;

    let input = CreateFormInput::new("Settings Form", "settings-form")
        .settings(
            FormSettings::new()
                .success_message("Thank you!")
                .submit_label("Send")
                .redirect_url("/thanks"),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let settings = form.settings();

    assert_eq!(settings.success_message, Some("Thank you!".to_string()));
    assert_eq!(settings.submit_label, Some("Send".to_string()));
    assert_eq!(settings.redirect_url, Some("/thanks".to_string()));
}

#[tokio::test]
async fn test_create_form_with_step() {
    let db = setup().await;

    let input = CreateFormInput::new("Form with Step", "form-with-step")
        .step(CreateStepInput::new("Main Step").description("The main step"));

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let steps = StepEntity::find_by_form(db.conn(), form.id).await.unwrap();

    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0].name, "Main Step");
    assert_eq!(steps[0].description, Some("The main step".to_string()));
}

#[tokio::test]
async fn test_create_form_with_fields() {
    let db = setup().await;

    let input = CreateFormInput::new("Form with Fields", "form-with-fields")
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("name", "Your Name", "text").required(),
                CreateFieldInput::new("email", "Email", "email").required(),
                CreateFieldInput::new("message", "Message", "textarea"),
            ]),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let steps = StepEntity::find_by_form(db.conn(), form.id).await.unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    assert_eq!(fields.len(), 3);
    assert_eq!(fields[0].name, "name");
    assert!(fields[0].required);
    assert_eq!(fields[1].name, "email");
    assert!(fields[1].required);
    assert_eq!(fields[2].name, "message");
    assert!(!fields[2].required);
}

#[tokio::test]
async fn test_create_form_with_options() {
    let db = setup().await;

    let input = CreateFormInput::new("Form with Options", "form-with-options")
        .step(
            CreateStepInput::new("Main").field(
                CreateFieldInput::new("country", "Country", "select")
                    .required()
                    .options(vec![
                        CreateOptionInput::new("United States", "us"),
                        CreateOptionInput::new("Canada", "ca"),
                        CreateOptionInput::new("Mexico", "mx"),
                    ]),
            ),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let steps = StepEntity::find_by_form(db.conn(), form.id).await.unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();
    let options = FieldOptionEntity::find_by_field(db.conn(), fields[0].id)
        .await
        .unwrap();

    assert_eq!(options.len(), 3);
    assert_eq!(options[0].label, "United States");
    assert_eq!(options[0].value, "us");
    assert_eq!(options[1].label, "Canada");
    assert_eq!(options[2].label, "Mexico");
}

#[tokio::test]
async fn test_create_multi_step_form() {
    let db = setup().await;

    let input = CreateFormInput::new("Multi-Step Form", "multi-step")
        .step(CreateStepInput::new("Step 1").order(0))
        .step(CreateStepInput::new("Step 2").order(1))
        .step(CreateStepInput::new("Step 3").order(2));

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let steps = StepEntity::find_by_form(db.conn(), form.id).await.unwrap();

    assert_eq!(steps.len(), 3);
    assert_eq!(steps[0].name, "Step 1");
    assert_eq!(steps[0].order, 0);
    assert_eq!(steps[1].name, "Step 2");
    assert_eq!(steps[1].order, 1);
    assert_eq!(steps[2].name, "Step 3");
    assert_eq!(steps[2].order, 2);
}

#[tokio::test]
async fn test_create_quiz_form() {
    let db = setup().await;

    let input = CreateFormInput::new("Quiz", "quiz")
        .settings(FormSettings::new().is_quiz(true))
        .step(
            CreateStepInput::new("Questions").field(
                CreateFieldInput::new("q1", "What is 2+2?", "radio")
                    .correct_answer("4")
                    .points(10)
                    .options(vec![
                        CreateOptionInput::new("3", "3"),
                        CreateOptionInput::new("4", "4").correct().points(10),
                        CreateOptionInput::new("5", "5"),
                    ]),
            ),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let settings = form.settings();
    assert!(settings.is_quiz);

    let steps = StepEntity::find_by_form(db.conn(), form.id).await.unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    assert_eq!(fields[0].correct_answer, Some("4".to_string()));
    assert_eq!(fields[0].points, Some(10));

    let options = FieldOptionEntity::find_by_field(db.conn(), fields[0].id)
        .await
        .unwrap();

    assert!(!options[0].is_correct);
    assert!(options[1].is_correct);
    assert_eq!(options[1].points, Some(10));
}

// ============================================================================
// Slug Uniqueness
// ============================================================================

#[tokio::test]
async fn test_duplicate_slug_rejected() {
    let db = setup().await;

    let input1 = CreateFormInput::new("Form 1", "duplicate-slug");
    FormBuilder::create(db.conn(), input1).await.unwrap();

    let input2 = CreateFormInput::new("Form 2", "duplicate-slug");
    let result = FormBuilder::create(db.conn(), input2).await;

    assert!(result.is_err());
}

// ============================================================================
// Form Update
// ============================================================================

#[tokio::test]
async fn test_update_form() {
    let db = setup().await;

    let input = CreateFormInput::new("Original", "update-test")
        .step(CreateStepInput::new("Step 1").field(
            CreateFieldInput::new("field1", "Field 1", "text"),
        ));

    let form = FormBuilder::create(db.conn(), input).await.unwrap();

    let update_input = CreateFormInput::new("Updated", "update-test")
        .description("Now with description")
        .step(CreateStepInput::new("New Step").fields(vec![
            CreateFieldInput::new("new_field1", "New Field 1", "email"),
            CreateFieldInput::new("new_field2", "New Field 2", "text"),
        ]));

    let updated = FormBuilder::update(db.conn(), form.id, update_input)
        .await
        .unwrap();

    assert_eq!(updated.name, "Updated");
    assert_eq!(updated.description, Some("Now with description".to_string()));

    let steps = StepEntity::find_by_form(db.conn(), updated.id)
        .await
        .unwrap();
    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0].name, "New Step");

    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "new_field1");
    assert_eq!(fields[1].name, "new_field2");
}

// ============================================================================
// Form Deletion
// ============================================================================

#[tokio::test]
async fn test_soft_delete_form() {
    let db = setup().await;

    let input = CreateFormInput::new("To Delete", "to-delete");
    let form = FormBuilder::create(db.conn(), input).await.unwrap();

    FormBuilder::soft_delete(db.conn(), form.id).await.unwrap();

    // Should not find via find_by_slug (active only)
    let found = FormBuilder::find_by_slug(db.conn(), "to-delete")
        .await
        .unwrap();
    assert!(found.is_none());

    // Should still exist in database (soft deleted)
    let raw = FormEntity::find_by_id(form.id)
        .one(db.conn())
        .await
        .unwrap();
    assert!(raw.is_some());
    assert!(raw.unwrap().deleted_at.is_some());
}

#[tokio::test]
async fn test_restore_form() {
    let db = setup().await;

    let input = CreateFormInput::new("To Restore", "to-restore");
    let form = FormBuilder::create(db.conn(), input).await.unwrap();

    FormBuilder::soft_delete(db.conn(), form.id).await.unwrap();

    // Restore
    let restored = FormBuilder::restore(db.conn(), form.id).await.unwrap();
    assert!(restored.deleted_at.is_none());

    // Should find via find_by_slug again
    let found = FormBuilder::find_by_slug(db.conn(), "to-restore")
        .await
        .unwrap();
    assert!(found.is_some());
}

#[tokio::test]
async fn test_hard_delete_form() {
    let db = setup().await;

    let input = CreateFormInput::new("To Hard Delete", "to-hard-delete")
        .step(
            CreateStepInput::new("Main").field(
                CreateFieldInput::new("field", "Field", "text"),
            ),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let form_id = form.id;

    FormBuilder::hard_delete(db.conn(), form_id).await.unwrap();

    // Should not exist at all
    let raw = FormEntity::find_by_id(form_id)
        .one(db.conn())
        .await
        .unwrap();
    assert!(raw.is_none());
}

// ============================================================================
// Form Queries
// ============================================================================

#[tokio::test]
async fn test_find_by_slug() {
    let db = setup().await;

    let input = CreateFormInput::new("Find Me", "find-me");
    FormBuilder::create(db.conn(), input).await.unwrap();

    let found = FormBuilder::find_by_slug(db.conn(), "find-me")
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Find Me");

    let not_found = FormBuilder::find_by_slug(db.conn(), "not-exists")
        .await
        .unwrap();
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_list_forms() {
    let db = setup().await;

    FormBuilder::create(db.conn(), CreateFormInput::new("Form A", "form-a"))
        .await
        .unwrap();
    FormBuilder::create(db.conn(), CreateFormInput::new("Form B", "form-b"))
        .await
        .unwrap();
    FormBuilder::create(db.conn(), CreateFormInput::new("Form C", "form-c"))
        .await
        .unwrap();

    let forms = FormBuilder::list(db.conn()).await.unwrap();
    assert_eq!(forms.len(), 3);
}

// ============================================================================
// Field Order
// ============================================================================

#[tokio::test]
async fn test_field_order_preservation() {
    let db = setup().await;

    let input = CreateFormInput::new("Ordered Fields", "ordered-fields")
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("z_field", "Z Field", "text").order(2),
                CreateFieldInput::new("a_field", "A Field", "text").order(0),
                CreateFieldInput::new("m_field", "M Field", "text").order(1),
            ]),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let steps = StepEntity::find_by_form(db.conn(), form.id).await.unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    // Should be ordered by order field
    assert_eq!(fields[0].name, "a_field");
    assert_eq!(fields[1].name, "m_field");
    assert_eq!(fields[2].name, "z_field");
}

// ============================================================================
// Validation Rules
// ============================================================================

#[tokio::test]
async fn test_validation_rules_stored() {
    let db = setup().await;

    let input = CreateFormInput::new("Validated Form", "validated-form")
        .step(
            CreateStepInput::new("Main").field(
                CreateFieldInput::new("username", "Username", "text")
                    .validation(
                        ValidationRules::new()
                            .min_length(3)
                            .max_length(20)
                            .pattern("^[a-z0-9_]+$"),
                    ),
            ),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let steps = StepEntity::find_by_form(db.conn(), form.id).await.unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    let rules = fields[0].validation();
    assert_eq!(rules.min_length, Some(3));
    assert_eq!(rules.max_length, Some(20));
    assert_eq!(rules.pattern, Some("^[a-z0-9_]+$".to_string()));
}

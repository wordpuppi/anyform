//! Validation tests for axum-sea-forms.

mod common;

use anyform::{
    entities::field::Entity as FieldEntity,
    services::{CreateFieldInput, CreateFormInput, CreateStepInput, FormBuilder},
    schema::{FieldValue, ValidationRules},
    validation::validate_submission,
};
use common::db::TestDb;
use std::collections::HashMap;

async fn setup() -> TestDb {
    TestDb::new().await
}

fn make_data(pairs: Vec<(&str, &str)>) -> HashMap<String, FieldValue> {
    pairs
        .into_iter()
        .map(|(k, v)| (k.to_string(), FieldValue::from(v)))
        .collect()
}

// ============================================================================
// Required Field Validation
// ============================================================================

#[tokio::test]
async fn test_required_field_empty_value() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-required")
        .step(
            CreateStepInput::new("Main")
                .field(CreateFieldInput::new("name", "Name", "text").required()),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    let data: HashMap<String, FieldValue> = HashMap::new();
    let errors = validate_submission(&fields, &data);

    assert!(!errors.is_empty());
    assert!(errors.get("name").is_some());
}

#[tokio::test]
async fn test_required_field_with_value() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-required-ok")
        .step(
            CreateStepInput::new("Main")
                .field(CreateFieldInput::new("name", "Name", "text").required()),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    let data = make_data(vec![("name", "John")]);
    let errors = validate_submission(&fields, &data);

    assert!(errors.is_empty());
}

#[tokio::test]
async fn test_optional_field_empty_value() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-optional")
        .step(
            CreateStepInput::new("Main")
                .field(CreateFieldInput::new("name", "Name", "text")), // not required
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    let data: HashMap<String, FieldValue> = HashMap::new();
    let errors = validate_submission(&fields, &data);

    assert!(errors.is_empty());
}

// ============================================================================
// Email Validation
// ============================================================================

#[tokio::test]
async fn test_valid_email() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-email-valid")
        .step(
            CreateStepInput::new("Main")
                .field(CreateFieldInput::new("email", "Email", "email").required()),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    let data = make_data(vec![("email", "test@example.com")]);
    let errors = validate_submission(&fields, &data);
    assert!(errors.is_empty());
}

#[tokio::test]
async fn test_invalid_email() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-email-invalid")
        .step(
            CreateStepInput::new("Main")
                .field(CreateFieldInput::new("email", "Email", "email").required()),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    let data = make_data(vec![("email", "not-an-email")]);
    let errors = validate_submission(&fields, &data);
    assert!(!errors.is_empty());
    assert!(errors.get("email").is_some());
}

// ============================================================================
// String Length Validation
// ============================================================================

#[tokio::test]
async fn test_min_length_validation() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-min-length")
        .step(
            CreateStepInput::new("Main").field(
                CreateFieldInput::new("name", "Name", "text")
                    .validation(ValidationRules::new().min_length(5)),
            ),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    // Too short
    let data = make_data(vec![("name", "abc")]);
    let errors = validate_submission(&fields, &data);
    assert!(!errors.is_empty());

    // Exactly minimum
    let data = make_data(vec![("name", "abcde")]);
    let errors = validate_submission(&fields, &data);
    assert!(errors.is_empty());

    // Above minimum
    let data = make_data(vec![("name", "abcdefgh")]);
    let errors = validate_submission(&fields, &data);
    assert!(errors.is_empty());
}

#[tokio::test]
async fn test_max_length_validation() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-max-length")
        .step(
            CreateStepInput::new("Main").field(
                CreateFieldInput::new("name", "Name", "text")
                    .validation(ValidationRules::new().max_length(10)),
            ),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    // Too long
    let data = make_data(vec![("name", "this is way too long")]);
    let errors = validate_submission(&fields, &data);
    assert!(!errors.is_empty());

    // Exactly maximum
    let data = make_data(vec![("name", "1234567890")]);
    let errors = validate_submission(&fields, &data);
    assert!(errors.is_empty());

    // Below maximum
    let data = make_data(vec![("name", "short")]);
    let errors = validate_submission(&fields, &data);
    assert!(errors.is_empty());
}

// ============================================================================
// Pattern/Regex Validation
// ============================================================================

#[tokio::test]
async fn test_pattern_validation() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-pattern")
        .step(
            CreateStepInput::new("Main").field(
                CreateFieldInput::new("code", "Code", "text").validation(
                    ValidationRules::new()
                        .pattern("^[A-Z]{3}-\\d{4}$")
                        .pattern_message("Must be format XXX-0000"),
                ),
            ),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    // Valid pattern
    let data = make_data(vec![("code", "ABC-1234")]);
    let errors = validate_submission(&fields, &data);
    assert!(errors.is_empty());

    // Invalid pattern
    let data = make_data(vec![("code", "abc-1234")]);
    let errors = validate_submission(&fields, &data);
    assert!(!errors.is_empty());

    let data = make_data(vec![("code", "ABCD-1234")]);
    let errors = validate_submission(&fields, &data);
    assert!(!errors.is_empty());
}

// ============================================================================
// Numeric Validation
// ============================================================================

#[tokio::test]
async fn test_numeric_min_max() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-numeric")
        .step(
            CreateStepInput::new("Main").field(
                CreateFieldInput::new("age", "Age", "number")
                    .validation(ValidationRules::new().min(18.0).max(100.0)),
            ),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    // Below minimum
    let data = make_data(vec![("age", "15")]);
    let errors = validate_submission(&fields, &data);
    assert!(!errors.is_empty());

    // At minimum
    let data = make_data(vec![("age", "18")]);
    let errors = validate_submission(&fields, &data);
    assert!(errors.is_empty());

    // In range
    let data = make_data(vec![("age", "50")]);
    let errors = validate_submission(&fields, &data);
    assert!(errors.is_empty());

    // At maximum
    let data = make_data(vec![("age", "100")]);
    let errors = validate_submission(&fields, &data);
    assert!(errors.is_empty());

    // Above maximum
    let data = make_data(vec![("age", "150")]);
    let errors = validate_submission(&fields, &data);
    assert!(!errors.is_empty());
}

// ============================================================================
// URL Validation
// ============================================================================

#[tokio::test]
async fn test_url_validation() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-url")
        .step(
            CreateStepInput::new("Main")
                .field(CreateFieldInput::new("website", "Website", "url").required()),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    // Valid URLs
    for url in ["https://example.com", "http://example.com"] {
        let data = make_data(vec![("website", url)]);
        let errors = validate_submission(&fields, &data);
        assert!(errors.is_empty(), "URL '{}' should be valid", url);
    }

    // Invalid URLs
    for url in ["not-a-url", "ftp://example.com", "example.com"] {
        let data = make_data(vec![("website", url)]);
        let errors = validate_submission(&fields, &data);
        assert!(!errors.is_empty(), "URL '{}' should be invalid", url);
    }
}

// ============================================================================
// Phone Validation
// ============================================================================

#[tokio::test]
async fn test_phone_validation() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-phone")
        .step(
            CreateStepInput::new("Main")
                .field(CreateFieldInput::new("phone", "Phone", "tel").required()),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    // Valid phone numbers
    for phone in ["+1234567890", "123-456-7890", "(123) 456-7890", "1234567890"] {
        let data = make_data(vec![("phone", phone)]);
        let errors = validate_submission(&fields, &data);
        assert!(errors.is_empty(), "Phone '{}' should be valid", phone);
    }
}

// ============================================================================
// Date Validation
// ============================================================================

#[tokio::test]
async fn test_date_validation() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-date")
        .step(
            CreateStepInput::new("Main")
                .field(CreateFieldInput::new("birthdate", "Birth Date", "date").required()),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    // Valid date
    let data = make_data(vec![("birthdate", "2000-01-15")]);
    let errors = validate_submission(&fields, &data);
    assert!(errors.is_empty());

    // Invalid date
    let data = make_data(vec![("birthdate", "not-a-date")]);
    let errors = validate_submission(&fields, &data);
    assert!(!errors.is_empty());
}

// ============================================================================
// Multiple Field Validation
// ============================================================================

#[tokio::test]
async fn test_multiple_validation_errors() {
    let db = setup().await;

    let form = CreateFormInput::new("Test", "test-multiple-errors")
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("name", "Name", "text").required(),
                CreateFieldInput::new("email", "Email", "email").required(),
                CreateFieldInput::new("age", "Age", "number")
                    .required()
                    .validation(ValidationRules::new().min(18.0)),
            ]),
        );

    let form = FormBuilder::create(db.conn(), form).await.unwrap();
    let steps = anyform::entities::step::Entity::find_by_form(db.conn(), form.id)
        .await
        .unwrap();
    let fields = FieldEntity::find_by_step(db.conn(), steps[0].id)
        .await
        .unwrap();

    // All fields invalid
    let data = make_data(vec![
        ("email", "invalid"),
        ("age", "10"),
    ]);

    let errors = validate_submission(&fields, &data);
    assert_eq!(errors.len(), 3); // name required, email invalid, age too low
}

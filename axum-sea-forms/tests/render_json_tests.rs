//! Tests for the JSON renderer.

mod common;

use axum_sea_forms::{
    render::JsonRenderer,
    services::{CreateFieldInput, CreateFormInput, CreateOptionInput, CreateStepInput, FormBuilder},
    schema::FormSettings,
};
use common::db::TestDb;

async fn setup() -> TestDb {
    TestDb::new().await
}

// ============================================================================
// Basic JSON Rendering
// ============================================================================

#[tokio::test]
async fn test_render_simple_form() {
    let db = setup().await;

    let input = CreateFormInput::new("Simple Form", "json-simple")
        .description("A simple form");

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let result = JsonRenderer::render(db.conn(), &form).await.unwrap();

    assert_eq!(result.name, "Simple Form");
    assert_eq!(result.slug, "json-simple");
    assert_eq!(result.description, Some("A simple form".to_string()));
}

#[tokio::test]
async fn test_render_form_with_settings() {
    let db = setup().await;

    let input = CreateFormInput::new("Settings Form", "json-settings")
        .settings(
            FormSettings::new()
                .success_message("Thank you!")
                .submit_label("Send Message"),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let result = JsonRenderer::render(db.conn(), &form).await.unwrap();

    assert_eq!(result.settings.success_message, Some("Thank you!".to_string()));
    assert_eq!(result.settings.submit_label, Some("Send Message".to_string()));
}

#[tokio::test]
async fn test_render_form_with_fields() {
    let db = setup().await;

    let input = CreateFormInput::new("Fields Form", "json-fields")
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("name", "Your Name", "text")
                    .required()
                    .placeholder("John Doe"),
                CreateFieldInput::new("email", "Email", "email")
                    .required()
                    .help_text("We'll never share your email"),
            ]),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let result = JsonRenderer::render(db.conn(), &form).await.unwrap();

    assert_eq!(result.steps.len(), 1);
    assert_eq!(result.steps[0].fields.len(), 2);

    assert_eq!(result.steps[0].fields[0].name, "name");
    assert_eq!(result.steps[0].fields[0].label, "Your Name");
    assert_eq!(result.steps[0].fields[0].field_type, "text");
    assert!(result.steps[0].fields[0].required);
    assert_eq!(result.steps[0].fields[0].placeholder, Some("John Doe".to_string()));

    assert_eq!(result.steps[0].fields[1].name, "email");
    assert_eq!(result.steps[0].fields[1].help_text, Some("We'll never share your email".to_string()));
}

#[tokio::test]
async fn test_render_form_with_options() {
    let db = setup().await;

    let input = CreateFormInput::new("Options Form", "json-options")
        .step(
            CreateStepInput::new("Main").field(
                CreateFieldInput::new("country", "Country", "select")
                    .options(vec![
                        CreateOptionInput::new("United States", "us"),
                        CreateOptionInput::new("Canada", "ca"),
                        CreateOptionInput::new("Mexico", "mx"),
                    ]),
            ),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let result = JsonRenderer::render(db.conn(), &form).await.unwrap();

    let options = &result.steps[0].fields[0].options;
    assert_eq!(options.len(), 3);

    assert_eq!(options[0].label, "United States");
    assert_eq!(options[0].value, "us");
    assert_eq!(options[1].label, "Canada");
    assert_eq!(options[2].label, "Mexico");
}

#[tokio::test]
async fn test_render_multi_step_form() {
    let db = setup().await;

    let input = CreateFormInput::new("Multi-Step", "json-multi-step")
        .step(CreateStepInput::new("Step 1").order(0))
        .step(CreateStepInput::new("Step 2").order(1))
        .step(CreateStepInput::new("Step 3").order(2));

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let result = JsonRenderer::render(db.conn(), &form).await.unwrap();

    assert_eq!(result.steps.len(), 3);
    assert_eq!(result.steps[0].name, "Step 1");
    assert_eq!(result.steps[1].name, "Step 2");
    assert_eq!(result.steps[2].name, "Step 3");
}

#[tokio::test]
async fn test_render_validation_rules() {
    let db = setup().await;

    let input = CreateFormInput::new("Validation Form", "json-validation")
        .step(
            CreateStepInput::new("Main").field(
                CreateFieldInput::new("username", "Username", "text")
                    .validation(
                        axum_sea_forms::schema::ValidationRules::new()
                            .min_length(3)
                            .max_length(20),
                    ),
            ),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let result = JsonRenderer::render(db.conn(), &form).await.unwrap();

    let validation = &result.steps[0].fields[0].validation;
    assert_eq!(validation.min_length, Some(3));
    assert_eq!(validation.max_length, Some(20));
}

#[tokio::test]
async fn test_render_quiz_form() {
    let db = setup().await;

    let input = CreateFormInput::new("Quiz", "json-quiz")
        .settings(FormSettings::new().is_quiz(true).show_answers(true))
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
    let result = JsonRenderer::render(db.conn(), &form).await.unwrap();

    assert!(result.settings.is_quiz);
    assert!(result.settings.show_answers);
}

// ============================================================================
// Snapshot Tests
// ============================================================================

#[tokio::test]
async fn test_contact_form_json_snapshot() {
    let db = setup().await;

    let input = CreateFormInput::new("Contact Form", "contact-snapshot")
        .description("Get in touch with us")
        .settings(
            FormSettings::new()
                .success_message("Thank you for your message!")
                .submit_label("Send"),
        )
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("name", "Your Name", "text")
                    .required()
                    .placeholder("John Doe"),
                CreateFieldInput::new("email", "Email", "email")
                    .required()
                    .placeholder("you@example.com"),
                CreateFieldInput::new("subject", "Subject", "select")
                    .options(vec![
                        CreateOptionInput::new("General Inquiry", "general"),
                        CreateOptionInput::new("Support", "support"),
                        CreateOptionInput::new("Sales", "sales"),
                    ]),
                CreateFieldInput::new("message", "Message", "textarea")
                    .required()
                    .placeholder("How can we help?"),
            ]),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let result = JsonRenderer::render(db.conn(), &form).await.unwrap();

    // Convert to JSON Value for snapshot, replacing dynamic IDs
    let mut json = serde_json::to_value(&result).unwrap();
    json["id"] = serde_json::Value::String("[ID]".to_string());
    if let Some(steps) = json["steps"].as_array_mut() {
        for step in steps {
            step["id"] = serde_json::Value::String("[STEP_ID]".to_string());
            if let Some(fields) = step["fields"].as_array_mut() {
                for field in fields {
                    field["id"] = serde_json::Value::String("[FIELD_ID]".to_string());
                    if let Some(options) = field["options"].as_array_mut() {
                        for opt in options {
                            opt["id"] = serde_json::Value::String("[OPTION_ID]".to_string());
                        }
                    }
                }
            }
        }
    }

    insta::assert_json_snapshot!(json);
}

// ============================================================================
// Pretty Print
// ============================================================================

#[tokio::test]
async fn test_render_pretty() {
    let db = setup().await;

    let input = CreateFormInput::new("Pretty Form", "json-pretty");
    let form = FormBuilder::create(db.conn(), input).await.unwrap();

    let pretty = JsonRenderer::render_pretty(db.conn(), &form).await.unwrap();

    // Should be formatted with newlines and indentation
    assert!(pretty.contains('\n'));
    assert!(pretty.contains("  "));
}

// ============================================================================
// Field Types
// ============================================================================

#[tokio::test]
async fn test_render_all_field_types() {
    let db = setup().await;

    let input = CreateFormInput::new("All Types", "json-all-types")
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("text_field", "Text", "text"),
                CreateFieldInput::new("email_field", "Email", "email"),
                CreateFieldInput::new("url_field", "URL", "url"),
                CreateFieldInput::new("tel_field", "Phone", "tel"),
                CreateFieldInput::new("number_field", "Number", "number"),
                CreateFieldInput::new("textarea_field", "Textarea", "textarea"),
                CreateFieldInput::new("date_field", "Date", "date"),
                CreateFieldInput::new("time_field", "Time", "time"),
                CreateFieldInput::new("hidden_field", "Hidden", "hidden"),
            ]),
        );

    let form = FormBuilder::create(db.conn(), input).await.unwrap();
    let result = JsonRenderer::render(db.conn(), &form).await.unwrap();

    let fields = &result.steps[0].fields;
    assert_eq!(fields.len(), 9);

    let field_types: Vec<&str> = fields.iter().map(|f| f.field_type.as_str()).collect();
    assert!(field_types.contains(&"text"));
    assert!(field_types.contains(&"email"));
    assert!(field_types.contains(&"url"));
    assert!(field_types.contains(&"tel"));
    assert!(field_types.contains(&"number"));
    assert!(field_types.contains(&"textarea"));
    assert!(field_types.contains(&"date"));
    assert!(field_types.contains(&"time"));
    assert!(field_types.contains(&"hidden"));
}

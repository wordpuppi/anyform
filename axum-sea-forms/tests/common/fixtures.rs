//! Test fixtures and data builders.

use axum_sea_forms::{
    services::{CreateFieldInput, CreateFormInput, CreateOptionInput, CreateStepInput},
    schema::{FormSettings, UiOptions, ValidationRules},
};
use sea_orm::DatabaseConnection;

/// Creates a minimal contact form for testing.
pub fn contact_form() -> CreateFormInput {
    CreateFormInput::new("Test Contact Form", "test-contact")
        .description("A test contact form")
        .settings(FormSettings::new().success_message("Thank you!"))
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("name", "Name", "text")
                    .required()
                    .validation(ValidationRules::new().min_length(2).max_length(100)),
                CreateFieldInput::new("email", "Email", "email").required(),
                CreateFieldInput::new("message", "Message", "textarea")
                    .required()
                    .ui(UiOptions::new().rows(5)),
            ]),
        )
}

/// Creates a form with select/radio/checkbox options.
pub fn options_form() -> CreateFormInput {
    CreateFormInput::new("Test Options Form", "test-options")
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("country", "Country", "select")
                    .required()
                    .options(vec![
                        CreateOptionInput::new("United States", "us"),
                        CreateOptionInput::new("Canada", "ca"),
                        CreateOptionInput::new("Mexico", "mx"),
                    ]),
                CreateFieldInput::new("contact_method", "Contact Method", "radio")
                    .options(vec![
                        CreateOptionInput::new("Email", "email"),
                        CreateOptionInput::new("Phone", "phone"),
                    ]),
                CreateFieldInput::new("interests", "Interests", "checkbox")
                    .options(vec![
                        CreateOptionInput::new("News", "news"),
                        CreateOptionInput::new("Updates", "updates"),
                        CreateOptionInput::new("Offers", "offers"),
                    ]),
            ]),
        )
}

/// Creates a multi-step form for testing.
pub fn multi_step_form() -> CreateFormInput {
    CreateFormInput::new("Test Multi-Step Form", "test-multi-step")
        .step(
            CreateStepInput::new("Step 1: Personal Info")
                .order(0)
                .fields(vec![
                    CreateFieldInput::new("first_name", "First Name", "text").required(),
                    CreateFieldInput::new("last_name", "Last Name", "text").required(),
                ]),
        )
        .step(
            CreateStepInput::new("Step 2: Contact Info")
                .order(1)
                .fields(vec![
                    CreateFieldInput::new("email", "Email", "email").required(),
                    CreateFieldInput::new("phone", "Phone", "tel"),
                ]),
        )
        .step(
            CreateStepInput::new("Step 3: Confirmation")
                .order(2)
                .fields(vec![CreateFieldInput::new(
                    "agree_terms",
                    "I agree to the terms",
                    "checkbox",
                )
                .required()]),
        )
}

/// Creates a quiz form with scoring.
pub fn quiz_form() -> CreateFormInput {
    CreateFormInput::new("Test Quiz", "test-quiz")
        .settings(FormSettings::new().is_quiz(true).show_answers(true))
        .step(
            CreateStepInput::new("Questions").fields(vec![
                CreateFieldInput::new("q1", "What is 2 + 2?", "radio")
                    .required()
                    .correct_answer("4")
                    .points(10)
                    .options(vec![
                        CreateOptionInput::new("3", "3"),
                        CreateOptionInput::new("4", "4").correct().points(10),
                        CreateOptionInput::new("5", "5"),
                    ]),
                CreateFieldInput::new("q2", "Capital of France?", "select")
                    .required()
                    .correct_answer("paris")
                    .points(10)
                    .options(vec![
                        CreateOptionInput::new("London", "london"),
                        CreateOptionInput::new("Paris", "paris").correct().points(10),
                        CreateOptionInput::new("Berlin", "berlin"),
                    ]),
            ]),
        )
}

/// Creates a form with all validation types.
pub fn validation_form() -> CreateFormInput {
    CreateFormInput::new("Test Validation Form", "test-validation")
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("required_field", "Required Field", "text").required(),
                CreateFieldInput::new("min_length", "Min Length (5)", "text")
                    .validation(ValidationRules::new().min_length(5)),
                CreateFieldInput::new("max_length", "Max Length (10)", "text")
                    .validation(ValidationRules::new().max_length(10)),
                CreateFieldInput::new("range_length", "Length 5-10", "text")
                    .validation(ValidationRules::new().min_length(5).max_length(10)),
                CreateFieldInput::new("pattern", "Only letters", "text")
                    .validation(
                        ValidationRules::new()
                            .pattern("^[a-zA-Z]+$")
                            .pattern_message("Only letters allowed"),
                    ),
                CreateFieldInput::new("email_field", "Email", "email"),
                CreateFieldInput::new("url_field", "URL", "url"),
                CreateFieldInput::new("number_field", "Number (0-100)", "number")
                    .validation(ValidationRules::new().min(0.0).max(100.0)),
            ]),
        )
}

/// Creates a form with all field types.
pub fn all_field_types_form() -> CreateFormInput {
    CreateFormInput::new("All Field Types", "test-all-types")
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("text_field", "Text", "text"),
                CreateFieldInput::new("email_field", "Email", "email"),
                CreateFieldInput::new("url_field", "URL", "url"),
                CreateFieldInput::new("tel_field", "Phone", "tel"),
                CreateFieldInput::new("number_field", "Number", "number"),
                CreateFieldInput::new("textarea_field", "Textarea", "textarea")
                    .ui(UiOptions::new().rows(4)),
                CreateFieldInput::new("date_field", "Date", "date"),
                CreateFieldInput::new("time_field", "Time", "time"),
                CreateFieldInput::new("datetime_field", "DateTime", "datetime"),
                CreateFieldInput::new("hidden_field", "Hidden", "hidden")
                    .default_value("secret"),
                CreateFieldInput::new("heading_field", "Section Heading", "heading"),
                CreateFieldInput::new("paragraph_field", "Helper text here", "paragraph"),
            ]),
        )
}

/// Helper to create a form and return it.
pub async fn create_test_form(
    db: &DatabaseConnection,
    input: CreateFormInput,
) -> axum_sea_forms::Form {
    axum_sea_forms::FormBuilder::create(db, input)
        .await
        .expect("Failed to create test form")
}

/// Sample form submission data.
pub fn sample_submission_data() -> serde_json::Value {
    serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com",
        "message": "This is a test message"
    })
}

/// Empty submission data.
pub fn empty_submission_data() -> serde_json::Value {
    serde_json::json!({})
}

/// Invalid email submission data.
pub fn invalid_email_data() -> serde_json::Value {
    serde_json::json!({
        "name": "John Doe",
        "email": "not-an-email",
        "message": "Test message"
    })
}

//! Database seeding for example forms.

use sea_orm::DatabaseConnection;

use crate::error::FormError;
use crate::schema::{FormSettings, UiOptions, ValidationRules};
use crate::services::{
    CreateFieldInput, CreateFormInput, CreateOptionInput, CreateStepInput, FormBuilder,
};

/// Seeds all example forms into the database.
pub async fn seed_all(db: &DatabaseConnection) -> Result<(), FormError> {
    seed_contact_form(db).await?;
    seed_feedback_form(db).await?;
    seed_quiz_form(db).await?;
    Ok(())
}

/// Seeds only the contact form.
pub async fn seed_contact_form(db: &DatabaseConnection) -> Result<(), FormError> {
    // Check if already exists
    if FormBuilder::find_by_slug(db, "contact").await?.is_some() {
        return Ok(());
    }

    let form = CreateFormInput::new("Contact Form", "contact")
        .description("Get in touch with us")
        .settings(
            FormSettings::new()
                .success_message("Thank you for contacting us! We'll get back to you soon.")
                .submit_label("Send Message"),
        )
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("name", "Your Name", "text")
                    .required()
                    .placeholder("John Doe")
                    .validation(ValidationRules::new().min_length(2).max_length(100)),
                CreateFieldInput::new("email", "Email Address", "email")
                    .required()
                    .placeholder("you@example.com")
                    .help_text("We'll never share your email with anyone else."),
                CreateFieldInput::new("phone", "Phone Number", "tel")
                    .placeholder("+1 (555) 123-4567")
                    .help_text("Optional - for faster response"),
                CreateFieldInput::new("message", "Message", "textarea")
                    .required()
                    .placeholder("How can we help you?")
                    .validation(ValidationRules::new().min_length(10).max_length(2000))
                    .ui(UiOptions::new().rows(5)),
                CreateFieldInput::new("preferred_contact", "Preferred Contact Method", "radio")
                    .options(vec![
                        CreateOptionInput::new("Email", "email"),
                        CreateOptionInput::new("Phone", "phone"),
                        CreateOptionInput::new("Either is fine", "either"),
                    ])
                    .default_value("email"),
            ]),
        );

    FormBuilder::create(db, form).await?;
    Ok(())
}

/// Seeds only the feedback survey form.
pub async fn seed_feedback_form(db: &DatabaseConnection) -> Result<(), FormError> {
    // Check if already exists
    if FormBuilder::find_by_slug(db, "feedback").await?.is_some() {
        return Ok(());
    }

    let form = CreateFormInput::new("Feedback Survey", "feedback")
        .description("Help us improve by sharing your feedback")
        .settings(
            FormSettings::new()
                .success_message("Thank you for your feedback!")
                .submit_label("Submit Feedback"),
        )
        .step(
            CreateStepInput::new("Main").fields(vec![
                CreateFieldInput::new("satisfaction", "How satisfied are you with our service?", "rating")
                    .required()
                    .ui(UiOptions {
                        max_rating: Some(5),
                        ..Default::default()
                    }),
                CreateFieldInput::new("recommend", "How likely are you to recommend us?", "nps")
                    .required()
                    .help_text("0 = Not at all likely, 10 = Extremely likely")
                    .ui(UiOptions {
                        scale_min: Some(0),
                        scale_max: Some(10),
                        scale_labels: Some(crate::schema::ScaleLabels {
                            min_label: Some("Not likely".to_string()),
                            max_label: Some("Very likely".to_string()),
                            mid_label: None,
                        }),
                        ..Default::default()
                    }),
                CreateFieldInput::new("features_used", "Which features have you used?", "multi_select")
                    .options(vec![
                        CreateOptionInput::new("Form Builder", "form_builder"),
                        CreateOptionInput::new("Templates", "templates"),
                        CreateOptionInput::new("Analytics", "analytics"),
                        CreateOptionInput::new("API Integration", "api"),
                        CreateOptionInput::new("Export/Import", "export"),
                    ])
                    .help_text("Select all that apply"),
                CreateFieldInput::new("improvements", "What could we improve?", "textarea")
                    .placeholder("Share your suggestions...")
                    .ui(UiOptions::new().rows(4)),
            ]),
        );

    FormBuilder::create(db, form).await?;
    Ok(())
}

/// Seeds only the quiz example form.
pub async fn seed_quiz_form(db: &DatabaseConnection) -> Result<(), FormError> {
    // Check if already exists
    if FormBuilder::find_by_slug(db, "quiz").await?.is_some() {
        return Ok(());
    }

    let form = CreateFormInput::new("Knowledge Quiz", "quiz")
        .description("Test your knowledge!")
        .settings(
            FormSettings::new()
                .success_message("Quiz completed! Check your score below.")
                .submit_label("Submit Quiz")
                .is_quiz(true)
                .show_answers(true),
        )
        .step(
            CreateStepInput::new("Questions").fields(vec![
                CreateFieldInput::new("q1", "What is the capital of France?", "radio")
                    .required()
                    .correct_answer("paris")
                    .points(10)
                    .options(vec![
                        CreateOptionInput::new("London", "london"),
                        CreateOptionInput::new("Paris", "paris").correct().points(10),
                        CreateOptionInput::new("Berlin", "berlin"),
                        CreateOptionInput::new("Madrid", "madrid"),
                    ]),
                CreateFieldInput::new("q2", "Which planet is known as the Red Planet?", "select")
                    .required()
                    .correct_answer("mars")
                    .points(10)
                    .options(vec![
                        CreateOptionInput::new("Venus", "venus"),
                        CreateOptionInput::new("Mars", "mars").correct().points(10),
                        CreateOptionInput::new("Jupiter", "jupiter"),
                        CreateOptionInput::new("Saturn", "saturn"),
                    ]),
                CreateFieldInput::new("q3", "What is 2 + 2?", "radio")
                    .required()
                    .correct_answer("4")
                    .points(10)
                    .options(vec![
                        CreateOptionInput::new("3", "3"),
                        CreateOptionInput::new("4", "4").correct().points(10),
                        CreateOptionInput::new("5", "5"),
                        CreateOptionInput::new("22", "22"),
                    ]),
            ]),
        );

    FormBuilder::create(db, form).await?;
    Ok(())
}

/// Clears all seeded example forms.
pub async fn clear_seeded_forms(db: &DatabaseConnection) -> Result<(), FormError> {
    for slug in ["contact", "feedback", "quiz"] {
        if let Some(form) = FormBuilder::find_by_slug(db, slug).await? {
            FormBuilder::hard_delete(db, form.id).await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_form_structure() {
        let form = CreateFormInput::new("Contact Form", "contact")
            .step(
                CreateStepInput::new("Main")
                    .field(CreateFieldInput::new("name", "Name", "text").required()),
            );

        assert_eq!(form.name, "Contact Form");
        assert_eq!(form.slug, "contact");
        assert_eq!(form.steps.len(), 1);
        assert_eq!(form.steps[0].fields.len(), 1);
    }
}

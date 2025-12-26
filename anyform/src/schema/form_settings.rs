//! Form-level settings.

use serde::{Deserialize, Serialize};

/// Settings for a form.
///
/// These settings are stored as JSON in the database.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormSettings {
    /// Label for the submit button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_label: Option<String>,

    /// Message shown after successful submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_message: Option<String>,

    /// URL to redirect to after submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,

    /// Email addresses to notify on submission.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notify_emails: Vec<String>,

    /// Whether to show a progress indicator for multi-step forms.
    #[serde(default)]
    pub show_progress: bool,

    /// Whether to allow saving partial submissions.
    #[serde(default)]
    pub allow_partial_save: bool,

    /// Custom CSS class for the form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_class: Option<String>,

    /// Form action URL (defaults to current page).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,

    /// HTTP method for form submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    /// Whether this is a quiz form with scoring.
    #[serde(default)]
    pub is_quiz: bool,

    /// Whether to show correct answers after submission (for quizzes).
    #[serde(default)]
    pub show_answers: bool,

    /// Additional custom settings as JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<serde_json::Value>,
}

impl FormSettings {
    /// Creates new default form settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the submit button label.
    #[must_use]
    pub fn submit_label(mut self, label: impl Into<String>) -> Self {
        self.submit_label = Some(label.into());
        self
    }

    /// Sets the success message.
    #[must_use]
    pub fn success_message(mut self, message: impl Into<String>) -> Self {
        self.success_message = Some(message.into());
        self
    }

    /// Sets the redirect URL.
    #[must_use]
    pub fn redirect_url(mut self, url: impl Into<String>) -> Self {
        self.redirect_url = Some(url.into());
        self
    }

    /// Adds a notification email.
    #[must_use]
    pub fn notify_email(mut self, email: impl Into<String>) -> Self {
        self.notify_emails.push(email.into());
        self
    }

    /// Gets the submit label or a default.
    #[must_use]
    pub fn submit_label_or_default(&self) -> &str {
        self.submit_label.as_deref().unwrap_or("Submit")
    }

    /// Gets the HTTP method or a default.
    #[must_use]
    pub fn method_or_default(&self) -> &str {
        self.method.as_deref().unwrap_or("POST")
    }

    /// Sets whether this is a quiz form.
    #[must_use]
    pub fn is_quiz(mut self, is_quiz: bool) -> Self {
        self.is_quiz = is_quiz;
        self
    }

    /// Sets whether to show answers after submission.
    #[must_use]
    pub fn show_answers(mut self, show: bool) -> Self {
        self.show_answers = show;
        self
    }

    /// Sets the form action URL (where the form submits to).
    ///
    /// When set, the form will submit directly to this URL instead of
    /// the default anyform submission handler.
    #[must_use]
    pub fn action_url(mut self, url: impl Into<String>) -> Self {
        self.action_url = Some(url.into());
        self
    }

    /// Sets the HTTP method for form submission.
    ///
    /// Common values: "POST", "PUT", "PATCH". Defaults to "POST".
    #[must_use]
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_url_builder() {
        let settings = FormSettings::new()
            .action_url("https://api.example.com/leads");

        assert_eq!(
            settings.action_url,
            Some("https://api.example.com/leads".to_string())
        );
    }

    #[test]
    fn test_method_builder() {
        let settings = FormSettings::new().method("PUT");

        assert_eq!(settings.method, Some("PUT".to_string()));
    }

    #[test]
    fn test_action_url_and_method_together() {
        let settings = FormSettings::new()
            .action_url("https://hooks.example.com/webhook")
            .method("POST");

        assert_eq!(
            settings.action_url,
            Some("https://hooks.example.com/webhook".to_string())
        );
        assert_eq!(settings.method, Some("POST".to_string()));
    }

    #[test]
    fn test_method_or_default() {
        let settings_default = FormSettings::new();
        assert_eq!(settings_default.method_or_default(), "POST");

        let settings_custom = FormSettings::new().method("PUT");
        assert_eq!(settings_custom.method_or_default(), "PUT");
    }

    #[test]
    fn test_action_url_serialization() {
        let settings = FormSettings::new()
            .action_url("https://api.example.com/submit")
            .method("POST");

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("action_url"));
        assert!(json.contains("https://api.example.com/submit"));

        // Deserialize back
        let parsed: FormSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed.action_url,
            Some("https://api.example.com/submit".to_string())
        );
        assert_eq!(parsed.method, Some("POST".to_string()));
    }

    #[test]
    fn test_action_url_not_serialized_when_none() {
        let settings = FormSettings::new().submit_label("Send");

        let json = serde_json::to_string(&settings).unwrap();
        assert!(!json.contains("action_url"));
        assert!(!json.contains("method"));
    }
}

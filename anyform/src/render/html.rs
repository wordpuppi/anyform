//! HTML rendering for forms with multi-step and WASM hydration support.

use crate::condition::ConditionRule;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use std::fmt::Write;

use crate::entities::{field, field_option, form, step};
use crate::error::{FormError, ValidationErrors};
use crate::schema::{FieldValue, ValidationRules, ValueType};

/// Options for HTML rendering.
#[derive(Debug, Clone, Default)]
pub struct HtmlOptions {
    /// CSS class for the form element.
    pub form_class: Option<String>,
    /// CSS class for field containers.
    pub field_class: Option<String>,
    /// CSS class for labels.
    pub label_class: Option<String>,
    /// CSS class for input elements.
    pub input_class: Option<String>,
    /// CSS class for error messages.
    pub error_class: Option<String>,
    /// CSS class for help text.
    pub help_class: Option<String>,
    /// CSS class for the submit button.
    pub button_class: Option<String>,
    /// Whether to include CSRF token field.
    pub include_csrf: bool,
    /// CSRF token value if including.
    pub csrf_token: Option<String>,
    /// Whether to show required indicator (*).
    pub show_required_indicator: bool,
    /// Custom action URL (overrides form settings).
    pub action: Option<String>,
    /// Custom method (overrides form settings).
    pub method: Option<String>,
    /// Base URL for WASM client files. None = no WASM loader.
    pub wasm_base_url: Option<String>,
    /// Whether to force multi-step mode (auto-detected if None).
    pub multi_step: Option<bool>,
    /// Whether to include inline CSS for multi-step forms.
    pub include_styles: bool,
}

impl HtmlOptions {
    /// Creates new default options.
    #[must_use]
    pub fn new() -> Self {
        Self {
            show_required_indicator: true,
            include_styles: true,
            ..Default::default()
        }
    }

    /// Sets the form CSS class.
    #[must_use]
    pub fn form_class(mut self, class: impl Into<String>) -> Self {
        self.form_class = Some(class.into());
        self
    }

    /// Sets the field container CSS class.
    #[must_use]
    pub fn field_class(mut self, class: impl Into<String>) -> Self {
        self.field_class = Some(class.into());
        self
    }

    /// Sets the action URL.
    #[must_use]
    pub fn action(mut self, url: impl Into<String>) -> Self {
        self.action = Some(url.into());
        self
    }

    /// Sets the base URL for WASM client files.
    ///
    /// When set, the HTML renderer will include a script tag to load the WASM
    /// hydration module, enabling client-side step navigation and validation.
    #[must_use]
    pub fn wasm_base_url(mut self, url: impl Into<String>) -> Self {
        self.wasm_base_url = Some(url.into());
        self
    }

    /// Forces multi-step mode on or off.
    ///
    /// By default, multi-step mode is auto-detected based on the number of steps.
    #[must_use]
    pub fn multi_step(mut self, enabled: bool) -> Self {
        self.multi_step = Some(enabled);
        self
    }

    /// Whether to include inline CSS styles.
    #[must_use]
    pub fn include_styles(mut self, include: bool) -> Self {
        self.include_styles = include;
        self
    }
}

/// Inline CSS for multi-step forms.
const MULTI_STEP_CSS: &str = r#"<style>
.af-step:not([data-af-visible="true"]) { display: none; }
.af-field:not([data-af-visible="true"]) { display: none; }
.af-field.af-error input,
.af-field.af-error select,
.af-field.af-error textarea { border-color: var(--af-error, #ef4444); }
.af-field .af-error-message { color: var(--af-error, #ef4444); font-size: 0.875rem; }
</style>
"#;

/// Renders forms to HTML.
pub struct HtmlRenderer;

impl HtmlRenderer {
    /// Renders a form to HTML.
    pub async fn render(
        db: &DatabaseConnection,
        form: &form::Model,
        options: &HtmlOptions,
    ) -> Result<String, FormError> {
        Self::render_with_values(db, form, options, &HashMap::new(), &ValidationErrors::new())
            .await
    }

    /// Renders a form to HTML with pre-filled values and errors.
    pub async fn render_with_values(
        db: &DatabaseConnection,
        form: &form::Model,
        options: &HtmlOptions,
        values: &HashMap<String, FieldValue>,
        errors: &ValidationErrors,
    ) -> Result<String, FormError> {
        let settings = form.settings();
        let mut html = String::new();

        // Load all steps and fields
        let steps = step::Entity::find_by_form(db, form.id).await?;
        let mut needs_multipart = false;
        let mut all_fields: Vec<(&step::Model, Vec<field::Model>)> = Vec::new();

        for step in &steps {
            let fields = field::Entity::find_by_step(db, step.id).await?;
            for field in &fields {
                if let Some(vt) = field.value_type() {
                    if vt.is_file_type() {
                        needs_multipart = true;
                    }
                }
            }
            all_fields.push((step, fields));
        }

        // Determine if this is a multi-step form
        let is_multi_step = options.multi_step.unwrap_or(all_fields.len() > 1);

        // Include CSS for multi-step forms
        if is_multi_step && options.include_styles {
            html.push_str(MULTI_STEP_CSS);
        }

        // Form opening tag
        let default_action = format!("/forms/{}", form.slug);
        let action = options
            .action
            .as_deref()
            .or(settings.action_url.as_deref())
            .unwrap_or(&default_action);

        let method = options
            .method
            .as_deref()
            .or(settings.method.as_deref())
            .unwrap_or("POST");

        let enctype = if needs_multipart {
            " enctype=\"multipart/form-data\""
        } else {
            ""
        };

        // Build form class (always include af-form for WASM hydration)
        let mut form_class = String::from("af-form");
        if let Some(custom_class) = options
            .form_class
            .as_deref()
            .or(settings.css_class.as_deref())
        {
            form_class.push(' ');
            form_class.push_str(custom_class);
        }

        writeln!(
            html,
            "<form method=\"{method}\" action=\"{action}\"{enctype} class=\"{form_class}\" data-af-form=\"{}\">",
            form.slug
        )
        .unwrap();

        // CSRF token
        if options.include_csrf {
            if let Some(token) = &options.csrf_token {
                writeln!(
                    html,
                    "  <input type=\"hidden\" name=\"_csrf\" value=\"{token}\">"
                )
                .unwrap();
            }
        }

        // Render steps and fields
        for (step_index, (step, fields)) in all_fields.iter().enumerate() {
            Self::render_step(
                &mut html,
                step,
                fields,
                step_index,
                is_multi_step,
                values,
                errors,
                options,
                db,
            )
            .await?;
        }

        // Navigation buttons (multi-step) or submit button (single-step)
        if is_multi_step {
            Self::render_navigation(&mut html, &settings.submit_label_or_default(), options);
        } else {
            let button_class = options
                .button_class
                .as_ref()
                .map(|c| format!(" class=\"{c}\""))
                .unwrap_or_default();

            writeln!(
                html,
                "  <button type=\"submit\"{button_class}>{}</button>",
                escape_html(&settings.submit_label_or_default())
            )
            .unwrap();
        }

        // WASM loader script
        if let Some(wasm_url) = &options.wasm_base_url {
            let url = wasm_url.trim_end_matches('/');
            writeln!(
                html,
                "  <script type=\"module\" src=\"{url}/af-client.js\"></script>"
            )
            .unwrap();
        }

        writeln!(html, "</form>").unwrap();

        Ok(html)
    }

    /// Renders a single step container with its fields.
    async fn render_step(
        html: &mut String,
        step: &step::Model,
        fields: &[field::Model],
        step_index: usize,
        is_multi_step: bool,
        values: &HashMap<String, FieldValue>,
        errors: &ValidationErrors,
        options: &HtmlOptions,
        db: &DatabaseConnection,
    ) -> Result<(), FormError> {
        if is_multi_step {
            // Multi-step: use div with data attributes
            let visible = if step_index == 0 { "true" } else { "false" };
            let condition_attr = step
                .condition
                .as_ref()
                .map(|c| format!(" data-af-condition='{}'", escape_json_attr(c)))
                .unwrap_or_default();

            writeln!(
                html,
                "  <div class=\"af-step\" data-af-step=\"{step_index}\" data-af-visible=\"{visible}\"{condition_attr}>"
            )
            .unwrap();

            writeln!(html, "    <h2>{}</h2>", escape_html(&step.name)).unwrap();

            if let Some(desc) = &step.description {
                writeln!(html, "    <p>{}</p>", escape_html(desc)).unwrap();
            }
        } else if fields.len() > 1 || step.description.is_some() {
            // Single-step with multiple fields: use fieldset
            writeln!(html, "  <fieldset>").unwrap();
            writeln!(html, "    <legend>{}</legend>", escape_html(&step.name)).unwrap();

            if let Some(desc) = &step.description {
                writeln!(html, "    <p>{}</p>", escape_html(desc)).unwrap();
            }
        }

        // Render fields
        for field in fields {
            let field_options = if field.requires_options() {
                field_option::Entity::find_by_field(db, field.id).await?
            } else {
                Vec::new()
            };

            let value = values
                .get(&field.id.to_string())
                .or_else(|| values.get(&field.name));
            let field_errors = errors.get(&field.name);

            Self::render_field(html, field, &field_options, value, field_errors, options, is_multi_step);
        }

        if is_multi_step {
            writeln!(html, "  </div>").unwrap();
        } else if fields.len() > 1 || step.description.is_some() {
            writeln!(html, "  </fieldset>").unwrap();
        }

        Ok(())
    }

    /// Renders navigation buttons for multi-step forms.
    fn render_navigation(html: &mut String, submit_label: &str, options: &HtmlOptions) {
        let button_class = options
            .button_class
            .as_ref()
            .map(|c| format!(" {c}"))
            .unwrap_or_default();

        writeln!(html, "  <div class=\"af-navigation\">").unwrap();
        writeln!(
            html,
            "    <button type=\"button\" class=\"af-prev{button_class}\" disabled>Back</button>"
        )
        .unwrap();
        writeln!(
            html,
            "    <button type=\"button\" class=\"af-next{button_class}\">Next</button>"
        )
        .unwrap();
        writeln!(
            html,
            "    <button type=\"submit\" class=\"af-submit{button_class}\" style=\"display:none\">{}</button>",
            escape_html(submit_label)
        )
        .unwrap();
        writeln!(html, "  </div>").unwrap();
    }

    /// Renders a single field to HTML.
    fn render_field(
        html: &mut String,
        field: &field::Model,
        options: &[field_option::Model],
        value: Option<&FieldValue>,
        errors: Option<&Vec<String>>,
        html_options: &HtmlOptions,
        is_multi_step: bool,
    ) {
        let ui = field.ui();
        let value_type = field.value_type();

        // Skip display-only fields that don't render as inputs
        if let Some(ValueType::Heading) = value_type {
            let level = ui.heading_level.unwrap_or(2);
            writeln!(html, "    <h{level}>{}</h{level}>", escape_html(&field.label)).unwrap();
            return;
        }

        if let Some(ValueType::Paragraph) = value_type {
            if let Some(text) = &field.help_text {
                writeln!(html, "    <p>{}</p>", escape_html(text)).unwrap();
            }
            return;
        }

        // Build field container class
        let mut field_class = String::from("af-field");
        if let Some(custom_class) = &html_options.field_class {
            field_class.push(' ');
            field_class.push_str(custom_class);
        } else {
            field_class.push_str(" field");
        }

        if let Some(width) = &ui.width {
            write!(field_class, " field--{width}").unwrap();
        }

        if errors.is_some() {
            field_class.push_str(" field--error af-error");
        }

        // Build data attributes
        let mut data_attrs = format!(" data-af-field=\"{}\"", field.name);

        // Always visible initially (WASM will hide based on conditions)
        if is_multi_step {
            data_attrs.push_str(" data-af-visible=\"true\"");
        }

        // Condition attribute
        if let Some(condition) = &ui.condition {
            write!(
                data_attrs,
                " data-af-condition='{}'",
                render_condition_json(condition)
            )
            .unwrap();
        }

        // Validation attribute
        let validation = field.validation();
        if field.required || !validation.is_empty() {
            write!(
                data_attrs,
                " data-af-validation='{}'",
                render_validation_json(field.required, &validation)
            )
            .unwrap();
        }

        writeln!(html, "    <div class=\"{field_class}\"{data_attrs}>").unwrap();

        // Label
        let label_class = html_options
            .label_class
            .as_ref()
            .map(|c| format!(" class=\"{c}\""))
            .unwrap_or_default();

        let required_indicator = if field.required && html_options.show_required_indicator {
            " <span class=\"required\">*</span>"
        } else {
            ""
        };

        writeln!(
            html,
            "      <label for=\"{}\"{label_class}>{}{required_indicator}</label>",
            field.name,
            escape_html(&field.label)
        )
        .unwrap();

        // Input element
        let input_class = html_options
            .input_class
            .as_deref()
            .or(ui.input_class.as_deref())
            .map(|c| format!(" class=\"{c}\""))
            .unwrap_or_default();

        let required = if field.required { " required" } else { "" };
        let disabled = if ui.disabled { " disabled" } else { "" };
        let readonly = if ui.readonly { " readonly" } else { "" };

        let placeholder = field
            .placeholder
            .as_ref()
            .map(|p| format!(" placeholder=\"{}\"", escape_html(p)))
            .unwrap_or_default();

        let current_value = value.map(FieldValue::to_string_value).unwrap_or_default();

        match value_type {
            Some(ValueType::Textarea) => {
                let rows = ui.rows.unwrap_or(4);
                writeln!(
                    html,
                    "      <textarea name=\"{}\" id=\"{}\" rows=\"{rows}\"{input_class}{required}{disabled}{readonly}{placeholder}>{}</textarea>",
                    field.name,
                    field.name,
                    escape_html(&current_value)
                )
                .unwrap();
            }
            Some(ValueType::Select) => {
                writeln!(
                    html,
                    "      <select name=\"{}\" id=\"{}\"{input_class}{required}{disabled}>",
                    field.name, field.name
                )
                .unwrap();

                if !field.required {
                    writeln!(html, "        <option value=\"\">-- Select --</option>").unwrap();
                }

                for opt in options {
                    let selected = if current_value == opt.value {
                        " selected"
                    } else {
                        ""
                    };
                    writeln!(
                        html,
                        "        <option value=\"{}\"{selected}>{}</option>",
                        escape_html(&opt.value),
                        escape_html(&opt.label)
                    )
                    .unwrap();
                }

                writeln!(html, "      </select>").unwrap();
            }
            Some(ValueType::Radio) => {
                for opt in options {
                    let checked = if current_value == opt.value {
                        " checked"
                    } else {
                        ""
                    };
                    let opt_id = format!("{}_{}", field.name, opt.value);
                    writeln!(
                        html,
                        "      <label><input type=\"radio\" name=\"{}\" id=\"{}\" value=\"{}\"{checked}{required}{disabled}> {}</label>",
                        field.name,
                        opt_id,
                        escape_html(&opt.value),
                        escape_html(&opt.label)
                    )
                    .unwrap();
                }
            }
            Some(ValueType::Checkbox) => {
                let checked = value
                    .and_then(FieldValue::as_bool)
                    .unwrap_or(false);
                let checked_attr = if checked { " checked" } else { "" };
                writeln!(
                    html,
                    "      <input type=\"checkbox\" name=\"{}\" id=\"{}\" value=\"1\"{input_class}{checked_attr}{disabled}>",
                    field.name, field.name
                )
                .unwrap();
            }
            Some(ValueType::Hidden) => {
                writeln!(
                    html,
                    "      <input type=\"hidden\" name=\"{}\" id=\"{}\" value=\"{}\">",
                    field.name,
                    field.name,
                    escape_html(&current_value)
                )
                .unwrap();
            }
            Some(vt) => {
                let input_type = vt.html_input_type();
                writeln!(
                    html,
                    "      <input type=\"{input_type}\" name=\"{}\" id=\"{}\" value=\"{}\"{input_class}{required}{disabled}{readonly}{placeholder}>",
                    field.name,
                    field.name,
                    escape_html(&current_value)
                )
                .unwrap();
            }
            None => {
                // Default to text input
                writeln!(
                    html,
                    "      <input type=\"text\" name=\"{}\" id=\"{}\" value=\"{}\"{input_class}{required}{disabled}{readonly}{placeholder}>",
                    field.name,
                    field.name,
                    escape_html(&current_value)
                )
                .unwrap();
            }
        }

        // Help text
        if let Some(help) = &field.help_text {
            let help_class = html_options
                .help_class
                .as_ref()
                .map(|c| format!(" class=\"{c}\""))
                .unwrap_or_else(|| " class=\"help\"".to_string());

            writeln!(html, "      <small{help_class}>{}</small>", escape_html(help)).unwrap();
        }

        // Error messages
        if let Some(errs) = errors {
            let error_class = html_options
                .error_class
                .as_ref()
                .map(|c| format!(" class=\"{c}\""))
                .unwrap_or_else(|| " class=\"error af-error-message\"".to_string());

            for err in errs {
                writeln!(html, "      <span{error_class}>{}</span>", escape_html(err)).unwrap();
            }
        }

        writeln!(html, "    </div>").unwrap();
    }
}

/// Renders a ConditionRule to JSON for data attribute.
fn render_condition_json(condition: &ConditionRule) -> String {
    serde_json::to_string(condition).unwrap_or_else(|_| "{}".to_string())
}

/// Renders validation rules to JSON for data attribute.
fn render_validation_json(required: bool, rules: &ValidationRules) -> String {
    let mut obj = serde_json::Map::new();

    if required {
        obj.insert("required".to_string(), serde_json::Value::Bool(true));
    }

    if let Some(min) = rules.min_length {
        obj.insert("minLength".to_string(), serde_json::json!(min));
    }

    if let Some(max) = rules.max_length {
        obj.insert("maxLength".to_string(), serde_json::json!(max));
    }

    if let Some(min) = rules.min {
        obj.insert("min".to_string(), serde_json::json!(min));
    }

    if let Some(max) = rules.max {
        obj.insert("max".to_string(), serde_json::json!(max));
    }

    if let Some(pattern) = &rules.pattern {
        obj.insert("pattern".to_string(), serde_json::json!(pattern));
    }

    serde_json::to_string(&obj).unwrap_or_else(|_| "{}".to_string())
}

/// Escapes HTML special characters.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Escapes a JSON value for use in an HTML attribute.
fn escape_json_attr(value: &serde_json::Value) -> String {
    serde_json::to_string(value)
        .unwrap_or_else(|_| "{}".to_string())
        .replace('\'', "&#39;")
}

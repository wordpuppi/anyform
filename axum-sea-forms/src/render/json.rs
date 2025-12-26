//! JSON rendering for forms.

use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::entities::{field, field_option, form, step};
use crate::error::FormError;
use crate::schema::{FormSettings, UiOptions, ValidationRules};

/// Renders forms to JSON format for SPAs and headless usage.
pub struct JsonRenderer;

impl JsonRenderer {
    /// Renders a form to JSON.
    pub async fn render(
        db: &DatabaseConnection,
        form: &form::Model,
    ) -> Result<FormJson, FormError> {
        let steps = step::Entity::find_by_form(db, form.id).await?;

        let mut step_jsons = Vec::new();
        for step in &steps {
            let fields = field::Entity::find_by_step(db, step.id).await?;

            let mut field_jsons = Vec::new();
            for field in &fields {
                let options = if field.requires_options() {
                    field_option::Entity::find_by_field(db, field.id).await?
                } else {
                    Vec::new()
                };

                field_jsons.push(FieldJson {
                    id: field.id.to_string(),
                    name: field.name.clone(),
                    label: field.label.clone(),
                    field_type: field.field_type.clone(),
                    order: field.order,
                    required: field.required,
                    placeholder: field.placeholder.clone(),
                    help_text: field.help_text.clone(),
                    default_value: field.default_value.clone(),
                    validation: field.validation(),
                    ui_options: field.ui(),
                    options: options
                        .into_iter()
                        .map(|o| FieldOptionJson {
                            id: o.id.to_string(),
                            label: o.label,
                            value: o.value,
                            order: o.order,
                        })
                        .collect(),
                });
            }

            step_jsons.push(StepJson {
                id: step.id.to_string(),
                name: step.name.clone(),
                description: step.description.clone(),
                order: step.order,
                condition: step.condition_expr(),
                fields: field_jsons,
            });
        }

        Ok(FormJson {
            id: form.id.to_string(),
            name: form.name.clone(),
            slug: form.slug.clone(),
            description: form.description.clone(),
            settings: form.settings(),
            steps: step_jsons,
        })
    }

    /// Renders a form to a JSON string.
    pub async fn render_string(
        db: &DatabaseConnection,
        form: &form::Model,
    ) -> Result<String, FormError> {
        let json = Self::render(db, form).await?;
        serde_json::to_string(&json).map_err(|e| FormError::InvalidData(e.to_string()))
    }

    /// Renders a form to a pretty-printed JSON string.
    pub async fn render_pretty(
        db: &DatabaseConnection,
        form: &form::Model,
    ) -> Result<String, FormError> {
        let json = Self::render(db, form).await?;
        serde_json::to_string_pretty(&json).map_err(|e| FormError::InvalidData(e.to_string()))
    }
}

/// JSON representation of a form.
#[derive(Debug, Clone, Serialize)]
pub struct FormJson {
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub settings: FormSettings,
    pub steps: Vec<StepJson>,
}

/// JSON representation of a form step.
#[derive(Debug, Clone, Serialize)]
pub struct StepJson {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub order: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub fields: Vec<FieldJson>,
}

/// JSON representation of a form field.
#[derive(Debug, Clone, Serialize)]
pub struct FieldJson {
    pub id: String,
    pub name: String,
    pub label: String,
    pub field_type: String,
    pub order: i32,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "ValidationRules::is_empty")]
    pub validation: ValidationRules,
    #[serde(skip_serializing_if = "is_default_ui")]
    pub ui_options: UiOptions,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<FieldOptionJson>,
}

/// JSON representation of a field option.
#[derive(Debug, Clone, Serialize)]
pub struct FieldOptionJson {
    pub id: String,
    pub label: String,
    pub value: String,
    pub order: i32,
}

fn is_default_ui(ui: &UiOptions) -> bool {
    ui.css_class.is_none()
        && ui.input_class.is_none()
        && ui.label_class.is_none()
        && ui.width.is_none()
        && ui.rows.is_none()
        && !ui.autofocus
        && !ui.disabled
        && !ui.readonly
}

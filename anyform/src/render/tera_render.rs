//! Tera template context builder for forms.

use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::collections::HashMap;
use tera::Context;

use crate::entities::{field, field_option, form, step};
use crate::error::{FormError, ValidationErrors};
use crate::schema::{FieldValue, FormSettings, UiOptions, ValidationRules};

/// Builds Tera template contexts for forms.
pub struct TeraRenderer;

impl TeraRenderer {
    /// Builds a Tera context for rendering a form.
    pub async fn context(
        db: &DatabaseConnection,
        form: &form::Model,
    ) -> Result<Context, FormError> {
        Self::context_with_values(db, form, &HashMap::new(), &ValidationErrors::new()).await
    }

    /// Builds a Tera context with pre-filled values and errors.
    pub async fn context_with_values(
        db: &DatabaseConnection,
        form: &form::Model,
        values: &HashMap<String, FieldValue>,
        errors: &ValidationErrors,
    ) -> Result<Context, FormError> {
        let form_data = Self::build_form_data(db, form).await?;

        let mut ctx = Context::new();
        ctx.insert("form", &form_data);
        ctx.insert("values", &values);
        ctx.insert("errors", &errors.errors);

        Ok(ctx)
    }

    /// Builds the form data structure for templates.
    async fn build_form_data(
        db: &DatabaseConnection,
        form: &form::Model,
    ) -> Result<FormData, FormError> {
        let steps = step::Entity::find_by_form(db, form.id).await?;

        let mut step_data = Vec::new();
        let mut needs_multipart = false;

        for step in &steps {
            let fields = field::Entity::find_by_step(db, step.id).await?;

            let mut field_data = Vec::new();
            for f in &fields {
                let options = if f.requires_options() {
                    field_option::Entity::find_by_field(db, f.id)
                        .await?
                        .into_iter()
                        .map(|o| OptionData {
                            id: o.id.to_string(),
                            label: o.label,
                            value: o.value,
                            order: o.order,
                            is_correct: o.is_correct,
                        })
                        .collect()
                } else {
                    Vec::new()
                };

                if let Some(vt) = f.value_type() {
                    if vt.is_file_type() {
                        needs_multipart = true;
                    }
                }

                field_data.push(FieldData {
                    id: f.id.to_string(),
                    name: f.name.clone(),
                    label: f.label.clone(),
                    field_type: f.field_type.clone(),
                    order: f.order,
                    required: f.required,
                    placeholder: f.placeholder.clone(),
                    help_text: f.help_text.clone(),
                    default_value: f.default_value.clone(),
                    validation: f.validation(),
                    ui_options: f.ui(),
                    options,
                });
            }

            step_data.push(StepData {
                id: step.id.to_string(),
                name: step.name.clone(),
                description: step.description.clone(),
                order: step.order,
                condition: step.condition_expr(),
                fields: field_data,
            });
        }

        Ok(FormData {
            id: form.id.to_string(),
            name: form.name.clone(),
            slug: form.slug.clone(),
            description: form.description.clone(),
            settings: form.settings(),
            steps: step_data,
            needs_multipart,
        })
    }
}

/// Form data for Tera templates.
#[derive(Debug, Clone, Serialize)]
pub struct FormData {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub settings: FormSettings,
    pub steps: Vec<StepData>,
    pub needs_multipart: bool,
}

/// Step data for Tera templates.
#[derive(Debug, Clone, Serialize)]
pub struct StepData {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order: i32,
    pub condition: Option<String>,
    pub fields: Vec<FieldData>,
}

/// Field data for Tera templates.
#[derive(Debug, Clone, Serialize)]
pub struct FieldData {
    pub id: String,
    pub name: String,
    pub label: String,
    pub field_type: String,
    pub order: i32,
    pub required: bool,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub default_value: Option<String>,
    pub validation: ValidationRules,
    pub ui_options: UiOptions,
    pub options: Vec<OptionData>,
}

/// Option data for Tera templates.
#[derive(Debug, Clone, Serialize)]
pub struct OptionData {
    pub id: String,
    pub label: String,
    pub value: String,
    pub order: i32,
    pub is_correct: bool,
}

//! Form builder service for creating, updating, and deleting forms.

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{
    field::{ActiveModel as FieldActiveModel, Entity as FieldEntity},
    field_option::{ActiveModel as FieldOptionActiveModel, Entity as FieldOptionEntity},
    form::{ActiveModel as FormActiveModel, Column as FormColumn, Entity as FormEntity, Model as Form},
    step::{ActiveModel as StepActiveModel, Entity as StepEntity},
};
use crate::error::FormError;
use crate::schema::{FormSettings, UiOptions, ValidationRules};

/// Input for creating a new form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFormInput {
    /// Form name (display title).
    pub name: String,

    /// URL-safe slug (must be unique).
    pub slug: String,

    /// Optional description.
    #[serde(default)]
    pub description: Option<String>,

    /// Form settings (submit label, redirect URL, etc.).
    #[serde(default)]
    pub settings: FormSettings,

    /// Steps in the form (at least one required).
    #[serde(default)]
    pub steps: Vec<CreateStepInput>,
}

impl CreateFormInput {
    /// Creates a new form input with name and slug.
    #[must_use]
    pub fn new(name: impl Into<String>, slug: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            slug: slug.into(),
            description: None,
            settings: FormSettings::default(),
            steps: Vec::new(),
        }
    }

    /// Sets the description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the form settings.
    #[must_use]
    pub fn settings(mut self, settings: FormSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Adds a step to the form.
    #[must_use]
    pub fn step(mut self, step: CreateStepInput) -> Self {
        self.steps.push(step);
        self
    }

    /// Adds multiple steps to the form.
    #[must_use]
    pub fn steps(mut self, steps: Vec<CreateStepInput>) -> Self {
        self.steps = steps;
        self
    }
}

/// Input for creating a form step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStepInput {
    /// Step name.
    pub name: String,

    /// Optional description.
    #[serde(default)]
    pub description: Option<String>,

    /// Display order (0-indexed).
    #[serde(default)]
    pub order: i32,

    /// Conditional display expression (evalexpr syntax).
    #[serde(default)]
    pub condition: Option<String>,

    /// Fields in this step.
    #[serde(default)]
    pub fields: Vec<CreateFieldInput>,
}

impl CreateStepInput {
    /// Creates a new step input.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            order: 0,
            condition: None,
            fields: Vec::new(),
        }
    }

    /// Sets the description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the order.
    #[must_use]
    pub fn order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }

    /// Sets the condition expression.
    #[must_use]
    pub fn condition(mut self, condition: impl Into<String>) -> Self {
        self.condition = Some(condition.into());
        self
    }

    /// Adds a field to the step.
    #[must_use]
    pub fn field(mut self, field: CreateFieldInput) -> Self {
        self.fields.push(field);
        self
    }

    /// Adds multiple fields to the step.
    #[must_use]
    pub fn fields(mut self, fields: Vec<CreateFieldInput>) -> Self {
        self.fields = fields;
        self
    }
}

/// Input for creating a form field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldInput {
    /// Field identifier (snake_case).
    pub name: String,

    /// Display label.
    pub label: String,

    /// Field type (text, email, select, etc.).
    pub field_type: String,

    /// Display order within step.
    #[serde(default)]
    pub order: i32,

    /// Whether the field is required.
    #[serde(default)]
    pub required: bool,

    /// Placeholder text.
    #[serde(default)]
    pub placeholder: Option<String>,

    /// Help text shown below field.
    #[serde(default)]
    pub help_text: Option<String>,

    /// Default value.
    #[serde(default)]
    pub default_value: Option<String>,

    /// Validation rules.
    #[serde(default)]
    pub validation_rules: ValidationRules,

    /// UI options.
    #[serde(default)]
    pub ui_options: UiOptions,

    /// Options for select/radio/checkbox fields.
    #[serde(default)]
    pub options: Vec<CreateOptionInput>,

    // Quiz fields
    /// Correct answer for quiz questions.
    #[serde(default)]
    pub correct_answer: Option<String>,

    /// Points awarded for correct answer.
    #[serde(default)]
    pub points: Option<i32>,

    /// Weight for weighted scoring.
    #[serde(default)]
    pub weight: Option<f64>,
}

impl CreateFieldInput {
    /// Creates a new field input.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        label: impl Into<String>,
        field_type: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            field_type: field_type.into(),
            order: 0,
            required: false,
            placeholder: None,
            help_text: None,
            default_value: None,
            validation_rules: ValidationRules::default(),
            ui_options: UiOptions::default(),
            options: Vec::new(),
            correct_answer: None,
            points: None,
            weight: None,
        }
    }

    /// Sets the order.
    #[must_use]
    pub fn order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }

    /// Makes the field required.
    #[must_use]
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the placeholder.
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the help text.
    #[must_use]
    pub fn help_text(mut self, help_text: impl Into<String>) -> Self {
        self.help_text = Some(help_text.into());
        self
    }

    /// Sets the default value.
    #[must_use]
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Sets validation rules.
    #[must_use]
    pub fn validation(mut self, rules: ValidationRules) -> Self {
        self.validation_rules = rules;
        self
    }

    /// Sets UI options.
    #[must_use]
    pub fn ui(mut self, options: UiOptions) -> Self {
        self.ui_options = options;
        self
    }

    /// Adds an option.
    #[must_use]
    pub fn option(mut self, option: CreateOptionInput) -> Self {
        self.options.push(option);
        self
    }

    /// Adds multiple options.
    #[must_use]
    pub fn options(mut self, options: Vec<CreateOptionInput>) -> Self {
        self.options = options;
        self
    }

    /// Sets correct answer for quiz.
    #[must_use]
    pub fn correct_answer(mut self, answer: impl Into<String>) -> Self {
        self.correct_answer = Some(answer.into());
        self
    }

    /// Sets points for quiz.
    #[must_use]
    pub fn points(mut self, points: i32) -> Self {
        self.points = Some(points);
        self
    }
}

/// Input for creating a field option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOptionInput {
    /// Display label.
    pub label: String,

    /// Submitted value.
    pub value: String,

    /// Display order.
    #[serde(default)]
    pub order: i32,

    /// Whether this is the correct answer (for quizzes).
    #[serde(default)]
    pub is_correct: bool,

    /// Points for this option (for quizzes).
    #[serde(default)]
    pub points: Option<i32>,
}

impl CreateOptionInput {
    /// Creates a new option.
    #[must_use]
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            order: 0,
            is_correct: false,
            points: None,
        }
    }

    /// Sets the order.
    #[must_use]
    pub fn order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }

    /// Marks this option as correct.
    #[must_use]
    pub fn correct(mut self) -> Self {
        self.is_correct = true;
        self
    }

    /// Sets points for this option.
    #[must_use]
    pub fn points(mut self, points: i32) -> Self {
        self.points = Some(points);
        self
    }
}

/// Service for creating, updating, and deleting forms.
///
/// # Example
///
/// ```rust,ignore
/// use anyform::services::{FormBuilder, CreateFormInput, CreateStepInput, CreateFieldInput};
///
/// let input = CreateFormInput::new("Contact Form", "contact")
///     .description("Get in touch with us")
///     .step(
///         CreateStepInput::new("Main")
///             .field(CreateFieldInput::new("name", "Your Name", "text").required())
///             .field(CreateFieldInput::new("email", "Email Address", "email").required())
///             .field(CreateFieldInput::new("message", "Message", "textarea").required())
///     );
///
/// let form = FormBuilder::create(&db, input).await?;
/// ```
pub struct FormBuilder;

impl FormBuilder {
    /// Creates a new form with all nested steps, fields, and options.
    ///
    /// This operation is transactional - if any part fails, the entire
    /// operation is rolled back.
    pub async fn create(db: &DatabaseConnection, input: CreateFormInput) -> Result<Form, FormError> {
        // Check for slug uniqueness
        let existing = FormEntity::find()
            .filter(FormColumn::Slug.eq(&input.slug))
            .filter(FormColumn::DeletedAt.is_null())
            .one(db)
            .await?;

        if existing.is_some() {
            return Err(FormError::InvalidData(format!(
                "Form with slug '{}' already exists",
                input.slug
            )));
        }

        // Start transaction
        let txn = db.begin().await?;

        let now = chrono::Utc::now().fixed_offset();
        let form_id = Uuid::new_v4();

        // Create form
        let form = FormActiveModel {
            id: ActiveValue::Set(form_id),
            name: ActiveValue::Set(input.name),
            slug: ActiveValue::Set(input.slug),
            description: ActiveValue::Set(input.description),
            settings: ActiveValue::Set(Some(serde_json::to_value(&input.settings).unwrap_or_default())),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
            deleted_at: ActiveValue::Set(None),
        };

        let form = form.insert(&txn).await?;

        // Create steps (use default step if none provided)
        let steps = if input.steps.is_empty() {
            vec![CreateStepInput::new("Main")]
        } else {
            input.steps
        };

        for (step_idx, step_input) in steps.into_iter().enumerate() {
            let step_id = Uuid::new_v4();
            let step_order = if step_input.order == 0 {
                step_idx as i32
            } else {
                step_input.order
            };

            let step = StepActiveModel {
                id: ActiveValue::Set(step_id),
                form_id: ActiveValue::Set(form_id),
                name: ActiveValue::Set(step_input.name),
                description: ActiveValue::Set(step_input.description),
                order: ActiveValue::Set(step_order),
                condition: ActiveValue::Set(
                    step_input.condition.map(|c| serde_json::Value::String(c)),
                ),
                created_at: ActiveValue::Set(now),
            };

            step.insert(&txn).await?;

            // Create fields
            for (field_idx, field_input) in step_input.fields.into_iter().enumerate() {
                let field_id = Uuid::new_v4();
                let field_order = if field_input.order == 0 {
                    field_idx as i32
                } else {
                    field_input.order
                };

                let validation_json = if field_input.validation_rules.is_empty() {
                    None
                } else {
                    Some(serde_json::to_value(&field_input.validation_rules).unwrap_or_default())
                };

                let ui_json = serde_json::to_value(&field_input.ui_options).ok();
                let ui_json = ui_json.filter(|v| v != &serde_json::json!({}));

                let field = FieldActiveModel {
                    id: ActiveValue::Set(field_id),
                    step_id: ActiveValue::Set(step_id),
                    name: ActiveValue::Set(field_input.name),
                    label: ActiveValue::Set(field_input.label),
                    field_type: ActiveValue::Set(field_input.field_type),
                    order: ActiveValue::Set(field_order),
                    required: ActiveValue::Set(field_input.required),
                    placeholder: ActiveValue::Set(field_input.placeholder),
                    help_text: ActiveValue::Set(field_input.help_text),
                    default_value: ActiveValue::Set(field_input.default_value),
                    validation_rules: ActiveValue::Set(validation_json),
                    ui_options: ActiveValue::Set(ui_json),
                    correct_answer: ActiveValue::Set(field_input.correct_answer),
                    points: ActiveValue::Set(field_input.points),
                    weight: ActiveValue::Set(field_input.weight),
                    created_at: ActiveValue::Set(now),
                };

                field.insert(&txn).await?;

                // Create options
                for (opt_idx, opt_input) in field_input.options.into_iter().enumerate() {
                    let opt_order = if opt_input.order == 0 {
                        opt_idx as i32
                    } else {
                        opt_input.order
                    };

                    let option = FieldOptionActiveModel {
                        id: ActiveValue::Set(Uuid::new_v4()),
                        field_id: ActiveValue::Set(field_id),
                        label: ActiveValue::Set(opt_input.label),
                        value: ActiveValue::Set(opt_input.value),
                        order: ActiveValue::Set(opt_order),
                        is_correct: ActiveValue::Set(opt_input.is_correct),
                        points: ActiveValue::Set(opt_input.points),
                    };

                    option.insert(&txn).await?;
                }
            }
        }

        txn.commit().await?;

        Ok(form)
    }

    /// Updates an existing form by replacing all steps, fields, and options.
    ///
    /// This is a full replacement - existing steps/fields/options are deleted
    /// and replaced with the new input.
    pub async fn update(
        db: &DatabaseConnection,
        form_id: Uuid,
        input: CreateFormInput,
    ) -> Result<Form, FormError> {
        // Find existing form
        let existing = FormEntity::find_by_id(form_id)
            .filter(FormColumn::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or_else(|| FormError::NotFound(form_id.to_string()))?;

        // Check slug uniqueness (if changed)
        if existing.slug != input.slug {
            let slug_exists = FormEntity::find()
                .filter(FormColumn::Slug.eq(&input.slug))
                .filter(FormColumn::DeletedAt.is_null())
                .one(db)
                .await?;

            if slug_exists.is_some() {
                return Err(FormError::InvalidData(format!(
                    "Form with slug '{}' already exists",
                    input.slug
                )));
            }
        }

        let txn = db.begin().await?;

        // Delete existing steps (cascades to fields and options via FK)
        StepEntity::delete_many()
            .filter(crate::entities::step::Column::FormId.eq(form_id))
            .exec(&txn)
            .await?;

        let now = chrono::Utc::now().fixed_offset();

        // Update form
        let form = FormActiveModel {
            id: ActiveValue::Unchanged(form_id),
            name: ActiveValue::Set(input.name),
            slug: ActiveValue::Set(input.slug),
            description: ActiveValue::Set(input.description),
            settings: ActiveValue::Set(Some(serde_json::to_value(&input.settings).unwrap_or_default())),
            created_at: ActiveValue::Unchanged(existing.created_at),
            updated_at: ActiveValue::Set(now),
            deleted_at: ActiveValue::Unchanged(existing.deleted_at),
        };

        let form = form.update(&txn).await?;

        // Create steps (same logic as create)
        let steps = if input.steps.is_empty() {
            vec![CreateStepInput::new("Main")]
        } else {
            input.steps
        };

        for (step_idx, step_input) in steps.into_iter().enumerate() {
            let step_id = Uuid::new_v4();
            let step_order = if step_input.order == 0 {
                step_idx as i32
            } else {
                step_input.order
            };

            let step = StepActiveModel {
                id: ActiveValue::Set(step_id),
                form_id: ActiveValue::Set(form_id),
                name: ActiveValue::Set(step_input.name),
                description: ActiveValue::Set(step_input.description),
                order: ActiveValue::Set(step_order),
                condition: ActiveValue::Set(
                    step_input.condition.map(|c| serde_json::Value::String(c)),
                ),
                created_at: ActiveValue::Set(now),
            };

            step.insert(&txn).await?;

            for (field_idx, field_input) in step_input.fields.into_iter().enumerate() {
                let field_id = Uuid::new_v4();
                let field_order = if field_input.order == 0 {
                    field_idx as i32
                } else {
                    field_input.order
                };

                let validation_json = if field_input.validation_rules.is_empty() {
                    None
                } else {
                    Some(serde_json::to_value(&field_input.validation_rules).unwrap_or_default())
                };

                let ui_json = serde_json::to_value(&field_input.ui_options).ok();
                let ui_json = ui_json.filter(|v| v != &serde_json::json!({}));

                let field = FieldActiveModel {
                    id: ActiveValue::Set(field_id),
                    step_id: ActiveValue::Set(step_id),
                    name: ActiveValue::Set(field_input.name),
                    label: ActiveValue::Set(field_input.label),
                    field_type: ActiveValue::Set(field_input.field_type),
                    order: ActiveValue::Set(field_order),
                    required: ActiveValue::Set(field_input.required),
                    placeholder: ActiveValue::Set(field_input.placeholder),
                    help_text: ActiveValue::Set(field_input.help_text),
                    default_value: ActiveValue::Set(field_input.default_value),
                    validation_rules: ActiveValue::Set(validation_json),
                    ui_options: ActiveValue::Set(ui_json),
                    correct_answer: ActiveValue::Set(field_input.correct_answer),
                    points: ActiveValue::Set(field_input.points),
                    weight: ActiveValue::Set(field_input.weight),
                    created_at: ActiveValue::Set(now),
                };

                field.insert(&txn).await?;

                for (opt_idx, opt_input) in field_input.options.into_iter().enumerate() {
                    let opt_order = if opt_input.order == 0 {
                        opt_idx as i32
                    } else {
                        opt_input.order
                    };

                    let option = FieldOptionActiveModel {
                        id: ActiveValue::Set(Uuid::new_v4()),
                        field_id: ActiveValue::Set(field_id),
                        label: ActiveValue::Set(opt_input.label),
                        value: ActiveValue::Set(opt_input.value),
                        order: ActiveValue::Set(opt_order),
                        is_correct: ActiveValue::Set(opt_input.is_correct),
                        points: ActiveValue::Set(opt_input.points),
                    };

                    option.insert(&txn).await?;
                }
            }
        }

        txn.commit().await?;

        Ok(form)
    }

    /// Soft-deletes a form by setting deleted_at.
    ///
    /// The form and its data remain in the database but won't appear
    /// in normal queries.
    pub async fn soft_delete(db: &DatabaseConnection, form_id: Uuid) -> Result<(), FormError> {
        let form = FormEntity::find_by_id(form_id)
            .filter(FormColumn::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or_else(|| FormError::NotFound(form_id.to_string()))?;

        let now = chrono::Utc::now().fixed_offset();

        let form = FormActiveModel {
            id: ActiveValue::Unchanged(form.id),
            name: ActiveValue::Unchanged(form.name),
            slug: ActiveValue::Unchanged(form.slug),
            description: ActiveValue::Unchanged(form.description),
            settings: ActiveValue::Unchanged(form.settings),
            created_at: ActiveValue::Unchanged(form.created_at),
            updated_at: ActiveValue::Set(now),
            deleted_at: ActiveValue::Set(Some(now)),
        };

        form.update(db).await?;

        Ok(())
    }

    /// Restores a soft-deleted form.
    pub async fn restore(db: &DatabaseConnection, form_id: Uuid) -> Result<Form, FormError> {
        let form = FormEntity::find_by_id(form_id)
            .one(db)
            .await?
            .ok_or_else(|| FormError::NotFound(form_id.to_string()))?;

        if form.deleted_at.is_none() {
            return Ok(form);
        }

        let now = chrono::Utc::now().fixed_offset();

        let form = FormActiveModel {
            id: ActiveValue::Unchanged(form.id),
            name: ActiveValue::Unchanged(form.name),
            slug: ActiveValue::Unchanged(form.slug),
            description: ActiveValue::Unchanged(form.description),
            settings: ActiveValue::Unchanged(form.settings),
            created_at: ActiveValue::Unchanged(form.created_at),
            updated_at: ActiveValue::Set(now),
            deleted_at: ActiveValue::Set(None),
        };

        let form = form.update(db).await?;

        Ok(form)
    }

    /// Finds a form by slug (active forms only).
    pub async fn find_by_slug(db: &DatabaseConnection, slug: &str) -> Result<Option<Form>, FormError> {
        let form = FormEntity::find_by_slug(db, slug).await?;
        Ok(form)
    }

    /// Finds a form by ID (active forms only).
    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<Form>, FormError> {
        let form = FormEntity::find_by_id(id)
            .filter(FormColumn::DeletedAt.is_null())
            .one(db)
            .await?;
        Ok(form)
    }

    /// Lists all active forms.
    pub async fn list(db: &DatabaseConnection) -> Result<Vec<Form>, FormError> {
        let forms = FormEntity::find_active(db).await?;
        Ok(forms)
    }

    /// Permanently deletes a form and all related data.
    ///
    /// **Warning**: This is irreversible. Use `soft_delete` for safe deletion.
    pub async fn hard_delete(db: &DatabaseConnection, form_id: Uuid) -> Result<(), FormError> {
        let txn = db.begin().await?;

        // Delete options for all fields in all steps
        let steps = StepEntity::find()
            .filter(crate::entities::step::Column::FormId.eq(form_id))
            .all(&txn)
            .await?;

        for step in &steps {
            let fields = FieldEntity::find()
                .filter(crate::entities::field::Column::StepId.eq(step.id))
                .all(&txn)
                .await?;

            for field in &fields {
                FieldOptionEntity::delete_many()
                    .filter(crate::entities::field_option::Column::FieldId.eq(field.id))
                    .exec(&txn)
                    .await?;
            }

            FieldEntity::delete_many()
                .filter(crate::entities::field::Column::StepId.eq(step.id))
                .exec(&txn)
                .await?;
        }

        StepEntity::delete_many()
            .filter(crate::entities::step::Column::FormId.eq(form_id))
            .exec(&txn)
            .await?;

        // Delete submissions
        crate::entities::submission::Entity::delete_many()
            .filter(crate::entities::submission::Column::FormId.eq(form_id))
            .exec(&txn)
            .await?;

        // Delete results
        crate::entities::result::Entity::delete_many()
            .filter(crate::entities::result::Column::FormId.eq(form_id))
            .exec(&txn)
            .await?;

        // Delete form
        FormEntity::delete_by_id(form_id).exec(&txn).await?;

        txn.commit().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_form_input_builder() {
        let input = CreateFormInput::new("Contact", "contact")
            .description("Contact form")
            .step(
                CreateStepInput::new("Main")
                    .field(CreateFieldInput::new("name", "Name", "text").required())
                    .field(CreateFieldInput::new("email", "Email", "email").required()),
            );

        assert_eq!(input.name, "Contact");
        assert_eq!(input.slug, "contact");
        assert_eq!(input.description, Some("Contact form".to_string()));
        assert_eq!(input.steps.len(), 1);
        assert_eq!(input.steps[0].fields.len(), 2);
    }

    #[test]
    fn test_create_field_with_options() {
        let field = CreateFieldInput::new("country", "Country", "select")
            .required()
            .option(CreateOptionInput::new("United States", "us"))
            .option(CreateOptionInput::new("Canada", "ca"))
            .option(CreateOptionInput::new("Mexico", "mx"));

        assert_eq!(field.options.len(), 3);
        assert!(field.required);
    }

    #[test]
    fn test_quiz_field() {
        let field = CreateFieldInput::new("q1", "What is 2+2?", "radio")
            .required()
            .correct_answer("4")
            .points(10)
            .option(CreateOptionInput::new("3", "3"))
            .option(CreateOptionInput::new("4", "4").correct().points(10))
            .option(CreateOptionInput::new("5", "5"));

        assert_eq!(field.correct_answer, Some("4".to_string()));
        assert_eq!(field.points, Some(10));
        assert!(field.options[1].is_correct);
    }
}

//! Field entity.

use sea_orm::entity::prelude::*;
use sea_orm::{QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::condition::ConditionRule;

use crate::schema::{UiOptions, ValidationRules, ValueType};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "asf_fields")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub step_id: Uuid,

    /// Field identifier (snake_case).
    pub name: String,

    /// Display label.
    pub label: String,

    /// Field type as string (e.g., "text", "email", "select").
    pub field_type: String,

    #[sea_orm(column_name = "order")]
    pub order: i32,

    pub required: bool,

    pub placeholder: Option<String>,

    pub help_text: Option<String>,

    pub default_value: Option<String>,

    /// Validation rules as JSON.
    #[sea_orm(column_type = "Json")]
    pub validation_rules: Option<serde_json::Value>,

    /// UI options as JSON.
    #[sea_orm(column_type = "Json")]
    pub ui_options: Option<serde_json::Value>,

    // Quiz fields (Phase 3)
    pub correct_answer: Option<String>,

    pub points: Option<i32>,

    pub weight: Option<f64>,

    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::step::Entity",
        from = "Column::StepId",
        to = "super::step::Column::Id"
    )]
    Step,
    #[sea_orm(has_many = "super::field_option::Entity")]
    Options,
}

impl Related<super::step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Step.def()
    }
}

impl Related<super::field_option::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Options.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Returns the field type as a `ValueType` enum.
    #[must_use]
    pub fn value_type(&self) -> Option<ValueType> {
        self.field_type.parse().ok()
    }

    /// Returns the validation rules, parsed from JSON.
    #[must_use]
    pub fn validation(&self) -> ValidationRules {
        self.validation_rules
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default()
    }

    /// Returns the UI options, parsed from JSON.
    #[must_use]
    pub fn ui(&self) -> UiOptions {
        self.ui_options
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default()
    }

    /// Returns true if this field type requires options.
    #[must_use]
    pub fn requires_options(&self) -> bool {
        self.value_type()
            .map_or(false, |vt| vt.requires_options())
    }

    /// Returns true if this field is display-only.
    #[must_use]
    pub fn is_display_only(&self) -> bool {
        self.value_type()
            .map_or(false, |vt| vt.is_display_only())
    }

    /// Returns the condition rule for dynamic field visibility.
    #[must_use]
    pub fn condition(&self) -> Option<ConditionRule> {
        self.ui().condition
    }
}

impl Entity {
    /// Find all fields for a step, ordered by position.
    pub async fn find_by_step(
        db: &DatabaseConnection,
        step_id: Uuid,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(Column::StepId.eq(step_id))
            .order_by_asc(Column::Order)
            .all(db)
            .await
    }
}

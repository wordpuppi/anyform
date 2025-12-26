//! Step entity.

use crate::condition::ConditionRule;
use sea_orm::entity::prelude::*;
use sea_orm::{QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "af_steps")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub form_id: Uuid,

    pub name: String,

    pub description: Option<String>,

    #[sea_orm(column_name = "order")]
    pub order: i32,

    /// Conditional display logic as JSON (evalexpr expression).
    #[sea_orm(column_type = "Json")]
    pub condition: Option<serde_json::Value>,

    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::form::Entity",
        from = "Column::FormId",
        to = "super::form::Column::Id"
    )]
    Form,
    #[sea_orm(has_many = "super::field::Entity")]
    Fields,
}

impl Related<super::form::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Form.def()
    }
}

impl Related<super::field::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Fields.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Returns the condition expression, if any.
    #[must_use]
    pub fn condition_expr(&self) -> Option<String> {
        self.condition
            .as_ref()
            .and_then(|v| v.as_str().map(ToString::to_string))
    }

    /// Returns the condition rule for dynamic step visibility.
    #[must_use]
    pub fn condition_rule(&self) -> Option<ConditionRule> {
        self.condition
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

impl Entity {
    /// Find all steps for a form, ordered by position.
    pub async fn find_by_form(
        db: &DatabaseConnection,
        form_id: Uuid,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(Column::FormId.eq(form_id))
            .order_by_asc(Column::Order)
            .all(db)
            .await
    }
}

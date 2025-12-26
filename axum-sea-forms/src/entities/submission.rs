//! Submission entity.

use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::schema::FieldValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "asf_submissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub form_id: Uuid,

    /// Submission data as JSON: { "field_id": value }
    #[sea_orm(column_type = "Json")]
    pub data: serde_json::Value,

    /// Metadata as JSON: { ip, user_agent, referrer, etc. }
    #[sea_orm(column_type = "Json")]
    pub metadata: Option<serde_json::Value>,

    // Multi-step tracking
    pub current_step_id: Option<Uuid>,

    pub completed_at: Option<DateTimeWithTimeZone>,

    // Quiz scoring (Phase 3)
    pub score: Option<i32>,

    pub max_score: Option<i32>,

    pub result_key: Option<String>,

    pub created_at: DateTimeWithTimeZone,

    pub updated_at: DateTimeWithTimeZone,

    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::form::Entity",
        from = "Column::FormId",
        to = "super::form::Column::Id"
    )]
    Form,
    #[sea_orm(
        belongs_to = "super::step::Entity",
        from = "Column::CurrentStepId",
        to = "super::step::Column::Id"
    )]
    CurrentStep,
}

impl Related<super::form::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Form.def()
    }
}

impl Related<super::step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CurrentStep.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Returns true if the submission is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.completed_at.is_some()
    }

    /// Returns true if the submission is soft-deleted.
    #[must_use]
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Returns the submission data as a map of field ID to value.
    #[must_use]
    pub fn data_map(&self) -> HashMap<String, FieldValue> {
        serde_json::from_value(self.data.clone()).unwrap_or_default()
    }

    /// Gets a field value by field ID.
    #[must_use]
    pub fn get_field(&self, field_id: &str) -> Option<FieldValue> {
        self.data
            .as_object()
            .and_then(|obj| obj.get(field_id))
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Returns the score as a percentage (0-100).
    #[must_use]
    pub fn score_percentage(&self) -> Option<f64> {
        match (self.score, self.max_score) {
            (Some(score), Some(max)) if max > 0 => {
                Some((f64::from(score) / f64::from(max)) * 100.0)
            }
            _ => None,
        }
    }
}

/// Metadata about a submission.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubmissionMetadata {
    /// IP address of the submitter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// User agent string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    /// Referrer URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,

    /// User ID if authenticated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Session ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// Additional custom metadata.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Entity {
    /// Find all submissions for a form.
    pub async fn find_by_form(
        db: &DatabaseConnection,
        form_id: Uuid,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(Column::FormId.eq(form_id))
            .filter(Column::DeletedAt.is_null())
            .order_by_desc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// Find completed submissions for a form.
    pub async fn find_completed_by_form(
        db: &DatabaseConnection,
        form_id: Uuid,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(Column::FormId.eq(form_id))
            .filter(Column::DeletedAt.is_null())
            .filter(Column::CompletedAt.is_not_null())
            .order_by_desc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// Find a submission by ID (active only).
    pub async fn find_active_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<Model>, DbErr> {
        Self::find_by_id(id)
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
    }

    /// Soft-delete a submission.
    pub async fn soft_delete(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        let submission = Self::find_active_by_id(db, id).await?;

        if let Some(sub) = submission {
            let now = chrono::Utc::now().fixed_offset();

            let model = ActiveModel {
                id: ActiveValue::Unchanged(sub.id),
                form_id: ActiveValue::Unchanged(sub.form_id),
                data: ActiveValue::Unchanged(sub.data),
                metadata: ActiveValue::Unchanged(sub.metadata),
                current_step_id: ActiveValue::Unchanged(sub.current_step_id),
                completed_at: ActiveValue::Unchanged(sub.completed_at),
                score: ActiveValue::Unchanged(sub.score),
                max_score: ActiveValue::Unchanged(sub.max_score),
                result_key: ActiveValue::Unchanged(sub.result_key),
                created_at: ActiveValue::Unchanged(sub.created_at),
                updated_at: ActiveValue::Set(now),
                deleted_at: ActiveValue::Set(Some(now)),
            };

            model.update(db).await?;
        }

        Ok(())
    }
}

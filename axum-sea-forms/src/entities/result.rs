//! Result entity (for quiz result buckets).

use sea_orm::entity::prelude::*;
use sea_orm::{QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "asf_results")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub form_id: Uuid,

    /// Result identifier (e.g., "introvert", "expert").
    pub key: String,

    pub title: String,

    pub description: Option<String>,

    /// Minimum score for this result bucket.
    pub min_score: Option<i32>,

    /// Maximum score for this result bucket.
    pub max_score: Option<i32>,

    #[sea_orm(column_name = "order")]
    pub order: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::form::Entity",
        from = "Column::FormId",
        to = "super::form::Column::Id"
    )]
    Form,
}

impl Related<super::form::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Form.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Returns true if the given score falls within this result's range.
    #[must_use]
    pub fn matches_score(&self, score: i32) -> bool {
        let min_ok = self.min_score.map_or(true, |min| score >= min);
        let max_ok = self.max_score.map_or(true, |max| score <= max);
        min_ok && max_ok
    }
}

impl Entity {
    /// Find all results for a form, ordered by position.
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

    /// Find a result by form and key.
    pub async fn find_by_key(
        db: &DatabaseConnection,
        form_id: Uuid,
        key: &str,
    ) -> Result<Option<Model>, DbErr> {
        Self::find()
            .filter(Column::FormId.eq(form_id))
            .filter(Column::Key.eq(key))
            .one(db)
            .await
    }

    /// Find the result that matches a given score.
    pub async fn find_by_score(
        db: &DatabaseConnection,
        form_id: Uuid,
        score: i32,
    ) -> Result<Option<Model>, DbErr> {
        let results = Self::find_by_form(db, form_id).await?;
        Ok(results.into_iter().find(|r| r.matches_score(score)))
    }
}

//! Form entity.

use sea_orm::entity::prelude::*;
use sea_orm::{QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::schema::FormSettings;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "af_forms")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub name: String,

    #[sea_orm(unique)]
    pub slug: String,

    pub description: Option<String>,

    #[sea_orm(column_type = "Json")]
    pub settings: Option<serde_json::Value>,

    pub created_at: DateTimeWithTimeZone,

    pub updated_at: DateTimeWithTimeZone,

    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::step::Entity")]
    Steps,
    #[sea_orm(has_many = "super::submission::Entity")]
    Submissions,
    #[sea_orm(has_many = "super::result::Entity")]
    Results,
}

impl Related<super::step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Steps.def()
    }
}

impl Related<super::submission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submissions.def()
    }
}

impl Related<super::result::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Results.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Returns the form settings, parsed from JSON.
    #[must_use]
    pub fn settings(&self) -> FormSettings {
        self.settings
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default()
    }

    /// Returns true if the form is soft-deleted.
    #[must_use]
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

impl Entity {
    /// Find a form by its slug.
    pub async fn find_by_slug(
        db: &DatabaseConnection,
        slug: &str,
    ) -> Result<Option<Model>, DbErr> {
        Self::find()
            .filter(Column::Slug.eq(slug))
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
    }

    /// Find all non-deleted forms.
    pub async fn find_active(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(Column::DeletedAt.is_null())
            .order_by_desc(Column::CreatedAt)
            .all(db)
            .await
    }
}

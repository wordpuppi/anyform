//! Field option entity (for select, radio, checkbox fields).

use sea_orm::entity::prelude::*;
use sea_orm::{QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "asf_field_options")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub field_id: Uuid,

    pub label: String,

    pub value: String,

    #[sea_orm(column_name = "order")]
    pub order: i32,

    // Quiz fields
    pub is_correct: bool,

    pub points: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::field::Entity",
        from = "Column::FieldId",
        to = "super::field::Column::Id"
    )]
    Field,
}

impl Related<super::field::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Field.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    /// Find all options for a field, ordered by position.
    pub async fn find_by_field(
        db: &DatabaseConnection,
        field_id: Uuid,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(Column::FieldId.eq(field_id))
            .order_by_asc(Column::Order)
            .all(db)
            .await
    }
}

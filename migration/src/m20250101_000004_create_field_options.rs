use sea_orm_migration::prelude::*;

use crate::m20250101_000003_create_fields::AsfFields;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AsfFieldOptions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AsfFieldOptions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AsfFieldOptions::FieldId).uuid().not_null())
                    .col(
                        ColumnDef::new(AsfFieldOptions::Label)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AsfFieldOptions::Value)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AsfFieldOptions::Order)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    // Quiz fields
                    .col(
                        ColumnDef::new(AsfFieldOptions::IsCorrect)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(AsfFieldOptions::Points).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_asf_field_options_field")
                            .from(AsfFieldOptions::Table, AsfFieldOptions::FieldId)
                            .to(AsfFields::Table, AsfFields::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asf_field_options_field")
                    .table(AsfFieldOptions::Table)
                    .col(AsfFieldOptions::FieldId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asf_field_options_order")
                    .table(AsfFieldOptions::Table)
                    .col(AsfFieldOptions::FieldId)
                    .col(AsfFieldOptions::Order)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AsfFieldOptions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AsfFieldOptions {
    Table,
    Id,
    FieldId,
    Label,
    Value,
    Order,
    IsCorrect,
    Points,
}

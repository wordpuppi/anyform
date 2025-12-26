use sea_orm_migration::prelude::*;

use crate::m20250101_000002_create_steps::AsfSteps;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AsfFields::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AsfFields::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AsfFields::StepId).uuid().not_null())
                    .col(ColumnDef::new(AsfFields::Name).string_len(255).not_null())
                    .col(ColumnDef::new(AsfFields::Label).string_len(255).not_null())
                    .col(
                        ColumnDef::new(AsfFields::FieldType)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AsfFields::Order)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AsfFields::Required)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(AsfFields::Placeholder).string_len(255))
                    .col(ColumnDef::new(AsfFields::HelpText).text())
                    .col(ColumnDef::new(AsfFields::DefaultValue).text())
                    .col(ColumnDef::new(AsfFields::ValidationRules).json())
                    .col(ColumnDef::new(AsfFields::UiOptions).json())
                    // Quiz fields (Phase 3)
                    .col(ColumnDef::new(AsfFields::CorrectAnswer).text())
                    .col(ColumnDef::new(AsfFields::Points).integer())
                    .col(ColumnDef::new(AsfFields::Weight).double())
                    .col(
                        ColumnDef::new(AsfFields::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_asf_fields_step")
                            .from(AsfFields::Table, AsfFields::StepId)
                            .to(AsfSteps::Table, AsfSteps::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asf_fields_step")
                    .table(AsfFields::Table)
                    .col(AsfFields::StepId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asf_fields_order")
                    .table(AsfFields::Table)
                    .col(AsfFields::StepId)
                    .col(AsfFields::Order)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AsfFields::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AsfFields {
    Table,
    Id,
    StepId,
    Name,
    Label,
    FieldType,
    Order,
    Required,
    Placeholder,
    HelpText,
    DefaultValue,
    ValidationRules,
    UiOptions,
    CorrectAnswer,
    Points,
    Weight,
    CreatedAt,
}

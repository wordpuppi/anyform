use sea_orm_migration::prelude::*;

use super::m20250101_000001_create_forms::AsfForms;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AsfSteps::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AsfSteps::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AsfSteps::FormId).uuid().not_null())
                    .col(ColumnDef::new(AsfSteps::Name).string_len(255).not_null())
                    .col(ColumnDef::new(AsfSteps::Description).text())
                    .col(
                        ColumnDef::new(AsfSteps::Order)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(AsfSteps::Condition).json())
                    .col(
                        ColumnDef::new(AsfSteps::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_asf_steps_form")
                            .from(AsfSteps::Table, AsfSteps::FormId)
                            .to(AsfForms::Table, AsfForms::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asf_steps_form")
                    .table(AsfSteps::Table)
                    .col(AsfSteps::FormId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asf_steps_order")
                    .table(AsfSteps::Table)
                    .col(AsfSteps::FormId)
                    .col(AsfSteps::Order)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AsfSteps::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AsfSteps {
    Table,
    Id,
    FormId,
    Name,
    Description,
    Order,
    Condition,
    CreatedAt,
}

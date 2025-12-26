use sea_orm_migration::prelude::*;

use crate::m20250101_000001_create_forms::AsfForms;
use crate::m20250101_000002_create_steps::AsfSteps;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AsfSubmissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AsfSubmissions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AsfSubmissions::FormId).uuid().not_null())
                    .col(ColumnDef::new(AsfSubmissions::Data).json().not_null())
                    .col(ColumnDef::new(AsfSubmissions::Metadata).json())
                    // Multi-step tracking
                    .col(ColumnDef::new(AsfSubmissions::CurrentStepId).uuid())
                    .col(ColumnDef::new(AsfSubmissions::CompletedAt).timestamp_with_time_zone())
                    // Quiz scoring (Phase 3)
                    .col(ColumnDef::new(AsfSubmissions::Score).integer())
                    .col(ColumnDef::new(AsfSubmissions::MaxScore).integer())
                    .col(ColumnDef::new(AsfSubmissions::ResultKey).string_len(255))
                    .col(
                        ColumnDef::new(AsfSubmissions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AsfSubmissions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(AsfSubmissions::DeletedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_asf_submissions_form")
                            .from(AsfSubmissions::Table, AsfSubmissions::FormId)
                            .to(AsfForms::Table, AsfForms::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_asf_submissions_step")
                            .from(AsfSubmissions::Table, AsfSubmissions::CurrentStepId)
                            .to(AsfSteps::Table, AsfSteps::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asf_submissions_form")
                    .table(AsfSubmissions::Table)
                    .col(AsfSubmissions::FormId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asf_submissions_completed")
                    .table(AsfSubmissions::Table)
                    .col(AsfSubmissions::CompletedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AsfSubmissions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AsfSubmissions {
    Table,
    Id,
    FormId,
    Data,
    Metadata,
    CurrentStepId,
    CompletedAt,
    Score,
    MaxScore,
    ResultKey,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

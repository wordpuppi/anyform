use sea_orm_migration::prelude::*;

use crate::m20250101_000001_create_forms::AsfForms;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AsfResults::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AsfResults::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AsfResults::FormId).uuid().not_null())
                    .col(ColumnDef::new(AsfResults::Key).string_len(255).not_null())
                    .col(ColumnDef::new(AsfResults::Title).string_len(255).not_null())
                    .col(ColumnDef::new(AsfResults::Description).text())
                    .col(ColumnDef::new(AsfResults::MinScore).integer())
                    .col(ColumnDef::new(AsfResults::MaxScore).integer())
                    .col(
                        ColumnDef::new(AsfResults::Order)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_asf_results_form")
                            .from(AsfResults::Table, AsfResults::FormId)
                            .to(AsfForms::Table, AsfForms::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asf_results_form")
                    .table(AsfResults::Table)
                    .col(AsfResults::FormId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asf_results_form_key")
                    .table(AsfResults::Table)
                    .col(AsfResults::FormId)
                    .col(AsfResults::Key)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AsfResults::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AsfResults {
    Table,
    Id,
    FormId,
    Key,
    Title,
    Description,
    MinScore,
    MaxScore,
    Order,
}

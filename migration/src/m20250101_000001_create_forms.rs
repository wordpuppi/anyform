use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AsfForms::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AsfForms::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AsfForms::Name).string_len(255).not_null())
                    .col(
                        ColumnDef::new(AsfForms::Slug)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(AsfForms::Description).text())
                    .col(ColumnDef::new(AsfForms::Settings).json())
                    .col(
                        ColumnDef::new(AsfForms::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AsfForms::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(AsfForms::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asf_forms_slug")
                    .table(AsfForms::Table)
                    .col(AsfForms::Slug)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AsfForms::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AsfForms {
    Table,
    Id,
    Name,
    Slug,
    Description,
    Settings,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

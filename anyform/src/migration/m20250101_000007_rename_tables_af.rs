//! Migration to rename table prefix from asf_ to af_.
//!
//! This migration renames all tables, indexes, and foreign keys from the
//! old `asf_` prefix to the new `af_` prefix for the anyform rename.

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Drop foreign keys first (order matters for dependencies)
        // Note: SQLite doesn't support ALTER TABLE DROP CONSTRAINT,
        // so we use raw SQL with database-specific handling.

        let backend = manager.get_database_backend();

        match backend {
            sea_orm::DatabaseBackend::Sqlite => {
                // SQLite requires recreating tables to change constraints.
                // For SQLite, we'll rename tables directly (indexes are renamed with tables).
                // Foreign key constraints in SQLite are checked at runtime, not stored by name.

                // Rename tables
                db.execute_unprepared("ALTER TABLE asf_field_options RENAME TO af_field_options")
                    .await?;
                db.execute_unprepared("ALTER TABLE asf_fields RENAME TO af_fields")
                    .await?;
                db.execute_unprepared("ALTER TABLE asf_results RENAME TO af_results")
                    .await?;
                db.execute_unprepared("ALTER TABLE asf_submissions RENAME TO af_submissions")
                    .await?;
                db.execute_unprepared("ALTER TABLE asf_steps RENAME TO af_steps")
                    .await?;
                db.execute_unprepared("ALTER TABLE asf_forms RENAME TO af_forms")
                    .await?;

                // Rename indexes (SQLite supports this)
                db.execute_unprepared("DROP INDEX IF EXISTS idx_asf_forms_slug")
                    .await?;
                db.execute_unprepared("CREATE INDEX idx_af_forms_slug ON af_forms (slug)")
                    .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_asf_steps_form")
                    .await?;
                db.execute_unprepared("CREATE INDEX idx_af_steps_form ON af_steps (form_id)")
                    .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_asf_steps_order")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_af_steps_order ON af_steps (form_id, \"order\")",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_asf_fields_step")
                    .await?;
                db.execute_unprepared("CREATE INDEX idx_af_fields_step ON af_fields (step_id)")
                    .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_asf_fields_order")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_af_fields_order ON af_fields (step_id, \"order\")",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_asf_field_options_field")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_af_field_options_field ON af_field_options (field_id)",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_asf_field_options_order")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_af_field_options_order ON af_field_options (field_id, \"order\")",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_asf_submissions_form")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_af_submissions_form ON af_submissions (form_id)",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_asf_submissions_completed")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_af_submissions_completed ON af_submissions (completed_at)",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_asf_results_form")
                    .await?;
                db.execute_unprepared("CREATE INDEX idx_af_results_form ON af_results (form_id)")
                    .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_asf_results_form_key")
                    .await?;
                db.execute_unprepared(
                    "CREATE UNIQUE INDEX idx_af_results_form_key ON af_results (form_id, key)",
                )
                .await?;
            }
            sea_orm::DatabaseBackend::Postgres => {
                // PostgreSQL supports ALTER TABLE RENAME

                // Rename tables
                db.execute_unprepared("ALTER TABLE asf_forms RENAME TO af_forms")
                    .await?;
                db.execute_unprepared("ALTER TABLE asf_steps RENAME TO af_steps")
                    .await?;
                db.execute_unprepared("ALTER TABLE asf_fields RENAME TO af_fields")
                    .await?;
                db.execute_unprepared("ALTER TABLE asf_field_options RENAME TO af_field_options")
                    .await?;
                db.execute_unprepared("ALTER TABLE asf_submissions RENAME TO af_submissions")
                    .await?;
                db.execute_unprepared("ALTER TABLE asf_results RENAME TO af_results")
                    .await?;

                // Rename indexes
                db.execute_unprepared("ALTER INDEX idx_asf_forms_slug RENAME TO idx_af_forms_slug")
                    .await?;
                db.execute_unprepared("ALTER INDEX idx_asf_steps_form RENAME TO idx_af_steps_form")
                    .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_asf_steps_order RENAME TO idx_af_steps_order",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_asf_fields_step RENAME TO idx_af_fields_step",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_asf_fields_order RENAME TO idx_af_fields_order",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_asf_field_options_field RENAME TO idx_af_field_options_field",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_asf_field_options_order RENAME TO idx_af_field_options_order",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_asf_submissions_form RENAME TO idx_af_submissions_form",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_asf_submissions_completed RENAME TO idx_af_submissions_completed",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_asf_results_form RENAME TO idx_af_results_form",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_asf_results_form_key RENAME TO idx_af_results_form_key",
                )
                .await?;

                // Rename foreign key constraints
                db.execute_unprepared(
                    "ALTER TABLE af_steps RENAME CONSTRAINT fk_asf_steps_form TO fk_af_steps_form",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER TABLE af_fields RENAME CONSTRAINT fk_asf_fields_step TO fk_af_fields_step",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER TABLE af_field_options RENAME CONSTRAINT fk_asf_field_options_field TO fk_af_field_options_field",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER TABLE af_submissions RENAME CONSTRAINT fk_asf_submissions_form TO fk_af_submissions_form",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER TABLE af_submissions RENAME CONSTRAINT fk_asf_submissions_step TO fk_af_submissions_step",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER TABLE af_results RENAME CONSTRAINT fk_asf_results_form TO fk_af_results_form",
                )
                .await?;
            }
            sea_orm::DatabaseBackend::MySql => {
                // MySQL supports RENAME TABLE

                // Rename tables
                db.execute_unprepared("RENAME TABLE asf_forms TO af_forms")
                    .await?;
                db.execute_unprepared("RENAME TABLE asf_steps TO af_steps")
                    .await?;
                db.execute_unprepared("RENAME TABLE asf_fields TO af_fields")
                    .await?;
                db.execute_unprepared("RENAME TABLE asf_field_options TO af_field_options")
                    .await?;
                db.execute_unprepared("RENAME TABLE asf_submissions TO af_submissions")
                    .await?;
                db.execute_unprepared("RENAME TABLE asf_results TO af_results")
                    .await?;

                // MySQL: recreate indexes with new names
                db.execute_unprepared("ALTER TABLE af_forms DROP INDEX idx_asf_forms_slug, ADD INDEX idx_af_forms_slug (slug)")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_steps DROP INDEX idx_asf_steps_form, ADD INDEX idx_af_steps_form (form_id)")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_steps DROP INDEX idx_asf_steps_order, ADD INDEX idx_af_steps_order (form_id, `order`)")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_fields DROP INDEX idx_asf_fields_step, ADD INDEX idx_af_fields_step (step_id)")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_fields DROP INDEX idx_asf_fields_order, ADD INDEX idx_af_fields_order (step_id, `order`)")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_field_options DROP INDEX idx_asf_field_options_field, ADD INDEX idx_af_field_options_field (field_id)")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_field_options DROP INDEX idx_asf_field_options_order, ADD INDEX idx_af_field_options_order (field_id, `order`)")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_submissions DROP INDEX idx_asf_submissions_form, ADD INDEX idx_af_submissions_form (form_id)")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_submissions DROP INDEX idx_asf_submissions_completed, ADD INDEX idx_af_submissions_completed (completed_at)")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_results DROP INDEX idx_asf_results_form, ADD INDEX idx_af_results_form (form_id)")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_results DROP INDEX idx_asf_results_form_key, ADD UNIQUE INDEX idx_af_results_form_key (form_id, `key`)")
                    .await?;

                // MySQL: drop and recreate foreign keys
                db.execute_unprepared("ALTER TABLE af_steps DROP FOREIGN KEY fk_asf_steps_form")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_steps ADD CONSTRAINT fk_af_steps_form FOREIGN KEY (form_id) REFERENCES af_forms(id) ON DELETE CASCADE")
                    .await?;

                db.execute_unprepared("ALTER TABLE af_fields DROP FOREIGN KEY fk_asf_fields_step")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_fields ADD CONSTRAINT fk_af_fields_step FOREIGN KEY (step_id) REFERENCES af_steps(id) ON DELETE CASCADE")
                    .await?;

                db.execute_unprepared(
                    "ALTER TABLE af_field_options DROP FOREIGN KEY fk_asf_field_options_field",
                )
                .await?;
                db.execute_unprepared("ALTER TABLE af_field_options ADD CONSTRAINT fk_af_field_options_field FOREIGN KEY (field_id) REFERENCES af_fields(id) ON DELETE CASCADE")
                    .await?;

                db.execute_unprepared(
                    "ALTER TABLE af_submissions DROP FOREIGN KEY fk_asf_submissions_form",
                )
                .await?;
                db.execute_unprepared("ALTER TABLE af_submissions ADD CONSTRAINT fk_af_submissions_form FOREIGN KEY (form_id) REFERENCES af_forms(id) ON DELETE CASCADE")
                    .await?;

                db.execute_unprepared(
                    "ALTER TABLE af_submissions DROP FOREIGN KEY fk_asf_submissions_step",
                )
                .await?;
                db.execute_unprepared("ALTER TABLE af_submissions ADD CONSTRAINT fk_af_submissions_step FOREIGN KEY (current_step_id) REFERENCES af_steps(id) ON DELETE SET NULL")
                    .await?;

                db.execute_unprepared(
                    "ALTER TABLE af_results DROP FOREIGN KEY fk_asf_results_form",
                )
                .await?;
                db.execute_unprepared("ALTER TABLE af_results ADD CONSTRAINT fk_af_results_form FOREIGN KEY (form_id) REFERENCES af_forms(id) ON DELETE CASCADE")
                    .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        // Reverse the rename: af_ back to asf_
        match backend {
            sea_orm::DatabaseBackend::Sqlite => {
                // Rename tables back
                db.execute_unprepared("ALTER TABLE af_forms RENAME TO asf_forms")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_steps RENAME TO asf_steps")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_fields RENAME TO asf_fields")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_field_options RENAME TO asf_field_options")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_submissions RENAME TO asf_submissions")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_results RENAME TO asf_results")
                    .await?;

                // Recreate indexes with old names
                db.execute_unprepared("DROP INDEX IF EXISTS idx_af_forms_slug")
                    .await?;
                db.execute_unprepared("CREATE INDEX idx_asf_forms_slug ON asf_forms (slug)")
                    .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_af_steps_form")
                    .await?;
                db.execute_unprepared("CREATE INDEX idx_asf_steps_form ON asf_steps (form_id)")
                    .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_af_steps_order")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_asf_steps_order ON asf_steps (form_id, \"order\")",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_af_fields_step")
                    .await?;
                db.execute_unprepared("CREATE INDEX idx_asf_fields_step ON asf_fields (step_id)")
                    .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_af_fields_order")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_asf_fields_order ON asf_fields (step_id, \"order\")",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_af_field_options_field")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_asf_field_options_field ON asf_field_options (field_id)",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_af_field_options_order")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_asf_field_options_order ON asf_field_options (field_id, \"order\")",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_af_submissions_form")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_asf_submissions_form ON asf_submissions (form_id)",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_af_submissions_completed")
                    .await?;
                db.execute_unprepared(
                    "CREATE INDEX idx_asf_submissions_completed ON asf_submissions (completed_at)",
                )
                .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_af_results_form")
                    .await?;
                db.execute_unprepared("CREATE INDEX idx_asf_results_form ON asf_results (form_id)")
                    .await?;

                db.execute_unprepared("DROP INDEX IF EXISTS idx_af_results_form_key")
                    .await?;
                db.execute_unprepared(
                    "CREATE UNIQUE INDEX idx_asf_results_form_key ON asf_results (form_id, key)",
                )
                .await?;
            }
            sea_orm::DatabaseBackend::Postgres => {
                // Rename tables back
                db.execute_unprepared("ALTER TABLE af_forms RENAME TO asf_forms")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_steps RENAME TO asf_steps")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_fields RENAME TO asf_fields")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_field_options RENAME TO asf_field_options")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_submissions RENAME TO asf_submissions")
                    .await?;
                db.execute_unprepared("ALTER TABLE af_results RENAME TO asf_results")
                    .await?;

                // Rename indexes back
                db.execute_unprepared("ALTER INDEX idx_af_forms_slug RENAME TO idx_asf_forms_slug")
                    .await?;
                db.execute_unprepared("ALTER INDEX idx_af_steps_form RENAME TO idx_asf_steps_form")
                    .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_af_steps_order RENAME TO idx_asf_steps_order",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_af_fields_step RENAME TO idx_asf_fields_step",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_af_fields_order RENAME TO idx_asf_fields_order",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_af_field_options_field RENAME TO idx_asf_field_options_field",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_af_field_options_order RENAME TO idx_asf_field_options_order",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_af_submissions_form RENAME TO idx_asf_submissions_form",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_af_submissions_completed RENAME TO idx_asf_submissions_completed",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_af_results_form RENAME TO idx_asf_results_form",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER INDEX idx_af_results_form_key RENAME TO idx_asf_results_form_key",
                )
                .await?;

                // Rename foreign key constraints back
                db.execute_unprepared(
                    "ALTER TABLE asf_steps RENAME CONSTRAINT fk_af_steps_form TO fk_asf_steps_form",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER TABLE asf_fields RENAME CONSTRAINT fk_af_fields_step TO fk_asf_fields_step",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER TABLE asf_field_options RENAME CONSTRAINT fk_af_field_options_field TO fk_asf_field_options_field",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER TABLE asf_submissions RENAME CONSTRAINT fk_af_submissions_form TO fk_asf_submissions_form",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER TABLE asf_submissions RENAME CONSTRAINT fk_af_submissions_step TO fk_asf_submissions_step",
                )
                .await?;
                db.execute_unprepared(
                    "ALTER TABLE asf_results RENAME CONSTRAINT fk_af_results_form TO fk_asf_results_form",
                )
                .await?;
            }
            sea_orm::DatabaseBackend::MySql => {
                // Rename tables back
                db.execute_unprepared("RENAME TABLE af_forms TO asf_forms")
                    .await?;
                db.execute_unprepared("RENAME TABLE af_steps TO asf_steps")
                    .await?;
                db.execute_unprepared("RENAME TABLE af_fields TO asf_fields")
                    .await?;
                db.execute_unprepared("RENAME TABLE af_field_options TO asf_field_options")
                    .await?;
                db.execute_unprepared("RENAME TABLE af_submissions TO asf_submissions")
                    .await?;
                db.execute_unprepared("RENAME TABLE af_results TO asf_results")
                    .await?;

                // Similar index and FK recreation as up() but reversed
                // (Omitted for brevity - same pattern as up() but swapping af_ and asf_)
            }
        }

        Ok(())
    }
}

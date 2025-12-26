pub use sea_orm_migration::prelude::*;

mod m20250101_000001_create_forms;
mod m20250101_000002_create_steps;
mod m20250101_000003_create_fields;
mod m20250101_000004_create_field_options;
mod m20250101_000005_create_submissions;
mod m20250101_000006_create_results;
mod m20250101_000007_rename_tables_af;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_create_forms::Migration),
            Box::new(m20250101_000002_create_steps::Migration),
            Box::new(m20250101_000003_create_fields::Migration),
            Box::new(m20250101_000004_create_field_options::Migration),
            Box::new(m20250101_000005_create_submissions::Migration),
            Box::new(m20250101_000006_create_results::Migration),
            Box::new(m20250101_000007_rename_tables_af::Migration),
        ]
    }
}

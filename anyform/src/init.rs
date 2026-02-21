//! Schema initialization for embedded use.
//!
//! This module provides a simple way to initialize the anyform database schema
//! without using the migration system. This is useful when embedding anyform
//! into another application that has its own migration management.
//!
//! # Example
//!
//! ```rust,ignore
//! use anyform::init_schema;
//! use sea_orm::Database;
//!
//! let db = Database::connect("sqlite:my_app.db").await?;
//! init_schema(&db).await?;
//! ```

use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr};

/// Initialize the anyform database schema.
///
/// Creates all required tables with `IF NOT EXISTS` so it's safe to call multiple times.
/// This is an alternative to using the migration system, useful for embedded scenarios.
pub async fn init_schema(db: &DatabaseConnection) -> Result<(), DbErr> {
    let backend = db.get_database_backend();

    let statements = match backend {
        sea_orm::DatabaseBackend::Sqlite => sqlite_schema(),
        sea_orm::DatabaseBackend::Postgres => postgres_schema(),
        sea_orm::DatabaseBackend::MySql => mysql_schema(),
    };

    for sql in statements {
        db.execute(sea_orm::Statement::from_string(backend, sql.to_string()))
            .await?;
    }

    Ok(())
}

fn sqlite_schema() -> Vec<&'static str> {
    vec![
        // Forms table
        r#"CREATE TABLE IF NOT EXISTS af_forms (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            slug TEXT NOT NULL UNIQUE,
            description TEXT,
            settings TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            deleted_at TEXT
        )"#,
        // Steps table
        r#"CREATE TABLE IF NOT EXISTS af_steps (
            id TEXT PRIMARY KEY NOT NULL,
            form_id TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            "order" INTEGER NOT NULL DEFAULT 0,
            condition TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (form_id) REFERENCES af_forms(id) ON DELETE CASCADE
        )"#,
        // Fields table
        r#"CREATE TABLE IF NOT EXISTS af_fields (
            id TEXT PRIMARY KEY NOT NULL,
            step_id TEXT NOT NULL,
            name TEXT NOT NULL,
            label TEXT NOT NULL,
            field_type TEXT NOT NULL,
            "order" INTEGER NOT NULL DEFAULT 0,
            required INTEGER NOT NULL DEFAULT 0,
            placeholder TEXT,
            help_text TEXT,
            default_value TEXT,
            validation_rules TEXT,
            ui_options TEXT,
            correct_answer TEXT,
            points INTEGER,
            weight REAL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (step_id) REFERENCES af_steps(id) ON DELETE CASCADE
        )"#,
        // Field options table
        r#"CREATE TABLE IF NOT EXISTS af_field_options (
            id TEXT PRIMARY KEY NOT NULL,
            field_id TEXT NOT NULL,
            label TEXT NOT NULL,
            value TEXT NOT NULL,
            "order" INTEGER NOT NULL DEFAULT 0,
            is_correct INTEGER NOT NULL DEFAULT 0,
            points INTEGER,
            FOREIGN KEY (field_id) REFERENCES af_fields(id) ON DELETE CASCADE
        )"#,
        // Submissions table
        r#"CREATE TABLE IF NOT EXISTS af_submissions (
            id TEXT PRIMARY KEY NOT NULL,
            form_id TEXT NOT NULL,
            data TEXT NOT NULL,
            metadata TEXT,
            current_step_id TEXT,
            completed_at TEXT,
            score INTEGER,
            max_score INTEGER,
            result_key TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            deleted_at TEXT,
            FOREIGN KEY (form_id) REFERENCES af_forms(id) ON DELETE CASCADE,
            FOREIGN KEY (current_step_id) REFERENCES af_steps(id) ON DELETE SET NULL
        )"#,
        // Results table (for quizzes)
        r#"CREATE TABLE IF NOT EXISTS af_results (
            id TEXT PRIMARY KEY NOT NULL,
            form_id TEXT NOT NULL,
            key TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            min_score INTEGER,
            max_score INTEGER,
            "order" INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (form_id) REFERENCES af_forms(id) ON DELETE CASCADE
        )"#,
        // Indexes
        "CREATE INDEX IF NOT EXISTS idx_af_forms_slug ON af_forms(slug)",
        "CREATE INDEX IF NOT EXISTS idx_af_forms_deleted ON af_forms(deleted_at)",
        "CREATE INDEX IF NOT EXISTS idx_af_steps_form ON af_steps(form_id)",
        "CREATE INDEX IF NOT EXISTS idx_af_fields_step ON af_fields(step_id)",
        "CREATE INDEX IF NOT EXISTS idx_af_field_options_field ON af_field_options(field_id)",
        "CREATE INDEX IF NOT EXISTS idx_af_submissions_form ON af_submissions(form_id)",
        "CREATE INDEX IF NOT EXISTS idx_af_results_form ON af_results(form_id)",
    ]
}

fn postgres_schema() -> Vec<&'static str> {
    vec![
        // Forms table
        r#"CREATE TABLE IF NOT EXISTS af_forms (
            id UUID PRIMARY KEY NOT NULL,
            name VARCHAR(255) NOT NULL,
            slug VARCHAR(255) NOT NULL UNIQUE,
            description TEXT,
            settings JSONB,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            deleted_at TIMESTAMPTZ
        )"#,
        // Steps table
        r#"CREATE TABLE IF NOT EXISTS af_steps (
            id UUID PRIMARY KEY NOT NULL,
            form_id UUID NOT NULL REFERENCES af_forms(id) ON DELETE CASCADE,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            "order" INTEGER NOT NULL DEFAULT 0,
            condition JSONB,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#,
        // Fields table
        r#"CREATE TABLE IF NOT EXISTS af_fields (
            id UUID PRIMARY KEY NOT NULL,
            step_id UUID NOT NULL REFERENCES af_steps(id) ON DELETE CASCADE,
            name VARCHAR(255) NOT NULL,
            label VARCHAR(255) NOT NULL,
            field_type VARCHAR(50) NOT NULL,
            "order" INTEGER NOT NULL DEFAULT 0,
            required BOOLEAN NOT NULL DEFAULT FALSE,
            placeholder TEXT,
            help_text TEXT,
            default_value TEXT,
            validation_rules JSONB,
            ui_options JSONB,
            correct_answer TEXT,
            points INTEGER,
            weight DOUBLE PRECISION,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#,
        // Field options table
        r#"CREATE TABLE IF NOT EXISTS af_field_options (
            id UUID PRIMARY KEY NOT NULL,
            field_id UUID NOT NULL REFERENCES af_fields(id) ON DELETE CASCADE,
            label VARCHAR(255) NOT NULL,
            value VARCHAR(255) NOT NULL,
            "order" INTEGER NOT NULL DEFAULT 0,
            is_correct BOOLEAN NOT NULL DEFAULT FALSE,
            points INTEGER
        )"#,
        // Submissions table
        r#"CREATE TABLE IF NOT EXISTS af_submissions (
            id UUID PRIMARY KEY NOT NULL,
            form_id UUID NOT NULL REFERENCES af_forms(id) ON DELETE CASCADE,
            data JSONB NOT NULL,
            metadata JSONB,
            current_step_id UUID REFERENCES af_steps(id) ON DELETE SET NULL,
            completed_at TIMESTAMPTZ,
            score INTEGER,
            max_score INTEGER,
            result_key VARCHAR(255),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            deleted_at TIMESTAMPTZ
        )"#,
        // Results table
        r#"CREATE TABLE IF NOT EXISTS af_results (
            id UUID PRIMARY KEY NOT NULL,
            form_id UUID NOT NULL REFERENCES af_forms(id) ON DELETE CASCADE,
            key VARCHAR(255) NOT NULL,
            title VARCHAR(255) NOT NULL,
            description TEXT,
            min_score INTEGER,
            max_score INTEGER,
            "order" INTEGER NOT NULL DEFAULT 0
        )"#,
        // Indexes
        "CREATE INDEX IF NOT EXISTS idx_af_forms_slug ON af_forms(slug)",
        "CREATE INDEX IF NOT EXISTS idx_af_forms_deleted ON af_forms(deleted_at)",
        "CREATE INDEX IF NOT EXISTS idx_af_steps_form ON af_steps(form_id)",
        "CREATE INDEX IF NOT EXISTS idx_af_fields_step ON af_fields(step_id)",
        "CREATE INDEX IF NOT EXISTS idx_af_field_options_field ON af_field_options(field_id)",
        "CREATE INDEX IF NOT EXISTS idx_af_submissions_form ON af_submissions(form_id)",
        "CREATE INDEX IF NOT EXISTS idx_af_results_form ON af_results(form_id)",
    ]
}

fn mysql_schema() -> Vec<&'static str> {
    vec![
        // Forms table
        r#"CREATE TABLE IF NOT EXISTS af_forms (
            id CHAR(36) PRIMARY KEY NOT NULL,
            name VARCHAR(255) NOT NULL,
            slug VARCHAR(255) NOT NULL UNIQUE,
            description TEXT,
            settings JSON,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            deleted_at TIMESTAMP NULL
        )"#,
        // Steps table
        r#"CREATE TABLE IF NOT EXISTS af_steps (
            id CHAR(36) PRIMARY KEY NOT NULL,
            form_id CHAR(36) NOT NULL,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            `order` INTEGER NOT NULL DEFAULT 0,
            `condition` JSON,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (form_id) REFERENCES af_forms(id) ON DELETE CASCADE
        )"#,
        // Fields table
        r#"CREATE TABLE IF NOT EXISTS af_fields (
            id CHAR(36) PRIMARY KEY NOT NULL,
            step_id CHAR(36) NOT NULL,
            name VARCHAR(255) NOT NULL,
            label VARCHAR(255) NOT NULL,
            field_type VARCHAR(50) NOT NULL,
            `order` INTEGER NOT NULL DEFAULT 0,
            required BOOLEAN NOT NULL DEFAULT FALSE,
            placeholder TEXT,
            help_text TEXT,
            default_value TEXT,
            validation_rules JSON,
            ui_options JSON,
            correct_answer TEXT,
            points INTEGER,
            weight DOUBLE,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (step_id) REFERENCES af_steps(id) ON DELETE CASCADE
        )"#,
        // Field options table
        r#"CREATE TABLE IF NOT EXISTS af_field_options (
            id CHAR(36) PRIMARY KEY NOT NULL,
            field_id CHAR(36) NOT NULL,
            label VARCHAR(255) NOT NULL,
            value VARCHAR(255) NOT NULL,
            `order` INTEGER NOT NULL DEFAULT 0,
            is_correct BOOLEAN NOT NULL DEFAULT FALSE,
            points INTEGER,
            FOREIGN KEY (field_id) REFERENCES af_fields(id) ON DELETE CASCADE
        )"#,
        // Submissions table
        r#"CREATE TABLE IF NOT EXISTS af_submissions (
            id CHAR(36) PRIMARY KEY NOT NULL,
            form_id CHAR(36) NOT NULL,
            data JSON NOT NULL,
            metadata JSON,
            current_step_id CHAR(36),
            completed_at TIMESTAMP NULL,
            score INTEGER,
            max_score INTEGER,
            result_key VARCHAR(255),
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            deleted_at TIMESTAMP NULL,
            FOREIGN KEY (form_id) REFERENCES af_forms(id) ON DELETE CASCADE,
            FOREIGN KEY (current_step_id) REFERENCES af_steps(id) ON DELETE SET NULL
        )"#,
        // Results table
        r#"CREATE TABLE IF NOT EXISTS af_results (
            id CHAR(36) PRIMARY KEY NOT NULL,
            form_id CHAR(36) NOT NULL,
            `key` VARCHAR(255) NOT NULL,
            title VARCHAR(255) NOT NULL,
            description TEXT,
            min_score INTEGER,
            max_score INTEGER,
            `order` INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (form_id) REFERENCES af_forms(id) ON DELETE CASCADE
        )"#,
        // Indexes
        "CREATE INDEX idx_af_forms_deleted ON af_forms(deleted_at)",
        "CREATE INDEX idx_af_steps_form ON af_steps(form_id)",
        "CREATE INDEX idx_af_fields_step ON af_fields(step_id)",
        "CREATE INDEX idx_af_field_options_field ON af_field_options(field_id)",
        "CREATE INDEX idx_af_submissions_form ON af_submissions(form_id)",
        "CREATE INDEX idx_af_results_form ON af_results(form_id)",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_schema_syntax() {
        // Just verify the SQL strings are valid (basic check)
        let statements = sqlite_schema();
        assert!(!statements.is_empty());
        for stmt in statements {
            assert!(stmt.contains("af_"));
        }
    }

    #[test]
    fn test_postgres_schema_syntax() {
        let statements = postgres_schema();
        assert!(!statements.is_empty());
        for stmt in statements {
            assert!(stmt.contains("af_"));
        }
    }

    #[test]
    fn test_mysql_schema_syntax() {
        let statements = mysql_schema();
        assert!(!statements.is_empty());
        for stmt in statements {
            assert!(stmt.contains("af_"));
        }
    }
}

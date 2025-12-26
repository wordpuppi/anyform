//! CLI testing infrastructure for integration tests.
//!
//! Provides `TestCli` for running CLI commands against a temporary database.

use assert_cmd::Command;
use sea_orm::Database;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use anyform::MigratorTrait;

/// Test CLI wrapper for integration tests.
///
/// Creates a temporary SQLite database and provides helpers for running
/// CLI commands against it.
///
/// # Example
///
/// ```ignore
/// let cli = TestCli::new().await;
///
/// cli.cmd()
///     .args(["form", "list"])
///     .assert()
///     .success()
///     .stdout(predicates::str::contains("No forms found"));
/// ```
pub struct TestCli {
    temp_dir: TempDir,
    db_path: PathBuf,
}

impl TestCli {
    /// Creates a new CLI test environment with an initialized database.
    ///
    /// The database is created in a temporary directory and has all
    /// migrations applied.
    pub async fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let db_path = temp_dir.path().join("test.db");

        // Initialize database with migrations
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
        let db = Database::connect(&db_url)
            .await
            .expect("Failed to connect to test database");

        anyform::Migrator::up(&db, None)
            .await
            .expect("Failed to run migrations");

        // Close the connection so CLI can use it
        drop(db);

        Self { temp_dir, db_path }
    }

    /// Returns the database URL for this test environment.
    pub fn db_url(&self) -> String {
        format!("sqlite:{}?mode=rwc", self.db_path.display())
    }

    /// Returns a new database connection for this test environment.
    ///
    /// Useful for seeding data or verifying results.
    pub async fn connect(&self) -> sea_orm::DatabaseConnection {
        Database::connect(&self.db_url())
            .await
            .expect("Failed to connect to test database")
    }

    /// Returns a Command configured for this test environment.
    ///
    /// The command has `DATABASE_URL` set to the test database.
    pub fn cmd(&self) -> Command {
        let mut cmd =
            Command::cargo_bin("anyform").expect("anyform binary not found - build with --features cli");
        cmd.env("DATABASE_URL", self.db_url());
        cmd
    }

    /// Returns the path to the temporary directory.
    ///
    /// Useful for creating test files (e.g., JSON form definitions).
    pub fn temp_dir(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Returns the path to the database file.
    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    /// Creates a JSON file in the temp directory and returns its path.
    pub fn create_json_file(&self, name: &str, content: &serde_json::Value) -> PathBuf {
        let path = self.temp_dir.path().join(name);
        std::fs::write(
            &path,
            serde_json::to_string_pretty(content).expect("Failed to serialize JSON"),
        )
        .expect("Failed to write JSON file");
        path
    }

    /// Creates a directory in the temp directory and returns its path.
    pub fn create_dir(&self, name: &str) -> PathBuf {
        let path = self.temp_dir.path().join(name);
        std::fs::create_dir_all(&path).expect("Failed to create directory");
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cli_creation() {
        let cli = TestCli::new().await;

        // Verify database exists
        assert!(cli.db_path().exists());

        // Verify we can connect
        let db = cli.connect().await;
        assert!(db.ping().await.is_ok());
    }

    #[tokio::test]
    async fn test_cli_temp_dir() {
        let cli = TestCli::new().await;

        // Create a test file
        let content = serde_json::json!({"name": "test"});
        let path = cli.create_json_file("test.json", &content);

        assert!(path.exists());

        let read_content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(read_content, content);
    }
}

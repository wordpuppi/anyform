//! In-memory SQLite database setup for testing.

use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection};

/// A test database wrapper that provides an in-memory SQLite connection.
pub struct TestDb {
    pub db: DatabaseConnection,
}

impl TestDb {
    /// Creates a new in-memory SQLite database with all migrations applied.
    pub async fn new() -> Self {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to in-memory SQLite");

        migration::Migrator::up(&db, None)
            .await
            .expect("Failed to run migrations");

        Self { db }
    }

    /// Returns a reference to the database connection.
    pub fn conn(&self) -> &DatabaseConnection {
        &self.db
    }
}

/// Creates a new test database and returns the connection.
pub async fn setup_test_db() -> DatabaseConnection {
    TestDb::new().await.db
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_setup() {
        let test_db = TestDb::new().await;
        assert!(test_db.conn().ping().await.is_ok());
    }
}

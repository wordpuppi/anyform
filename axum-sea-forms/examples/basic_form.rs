//! Basic form example.
//!
//! Run with: cargo run --example basic_form

use axum::Router;
use axum_sea_forms::{FormsRouter, Migrator, MigratorTrait};
use sea_orm::Database;

#[tokio::main]
async fn main() {
    // Connect to SQLite database
    let db = Database::connect("sqlite:./example_forms.db?mode=rwc")
        .await
        .expect("Failed to connect to database");

    // Run migrations
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    println!("Database initialized.");

    // Build the router with form routes
    let app = Router::new().merge(FormsRouter::new(db));

    // Start the server
    println!("Starting server at http://localhost:3000");
    println!("Routes:");
    println!("  GET  /forms/:slug        - Render form HTML");
    println!("  GET  /forms/:slug/json   - Get form schema JSON");
    println!("  POST /forms/:slug        - Submit form");
    println!("  GET  /forms/:slug/success - Success page");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

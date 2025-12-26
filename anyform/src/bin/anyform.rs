//! anyform CLI tool.

use anyhow::Result;
use anyform::commands;
use anyform::commands::{FormAction, SubmissionAction};
use anyform::AnyFormRouter;
use axum::{routing::get, Router};
use clap::{Parser, Subcommand};
use http::{header, Method};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[derive(Parser)]
#[command(name = "anyform")]
#[command(author, version, about = "Any database. Any form. Zero hassle.", long_about = None)]
struct Cli {
    /// Database URL (or set DATABASE_URL env var)
    #[arg(short, long, env = "DATABASE_URL")]
    database_url: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize database (creates SQLite by default)
    Init {
        /// SQLite file path [default: ./anyform.db]
        #[arg(short, long, default_value = "./anyform.db")]
        path: PathBuf,

        /// Use external database URL instead of SQLite
        #[arg(long)]
        database: Option<String>,

        /// Overwrite existing database
        #[arg(long)]
        force: bool,

        /// Seed example forms after initialization
        #[arg(long)]
        seed: bool,
    },

    /// Run database migrations
    Migrate {
        /// Run pending migrations
        #[arg(long, default_value = "true")]
        up: bool,

        /// Rollback last migration
        #[arg(long)]
        down: bool,

        /// Show migration status
        #[arg(long)]
        status: bool,

        /// Rename tables from asf_ to af_ prefix
        #[arg(long)]
        rename_tables: bool,
    },

    /// Form management
    Form {
        #[command(subcommand)]
        action: FormAction,
    },

    /// Submission management
    Submissions {
        #[command(subcommand)]
        action: SubmissionAction,
    },

    /// Seed example forms into database
    Seed {
        /// Only seed the contact form
        #[arg(long)]
        contact_only: bool,

        /// Only seed the feedback form
        #[arg(long)]
        feedback_only: bool,

        /// Only seed the quiz form
        #[arg(long)]
        quiz_only: bool,

        /// Clear seeded forms instead of creating them
        #[arg(long)]
        clear: bool,
    },

    /// Start the API server
    Serve {
        /// Host to bind to
        #[arg(short = 'H', long, default_value = "0.0.0.0")]
        host: String,

        /// Port to bind to
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Disable admin routes
        #[arg(long)]
        no_admin: bool,

        /// Enable CORS for specified origin (use '*' for any)
        #[arg(long)]
        cors: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Get database URL (init command has its own handling)
    let database_url = cli
        .database_url
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .unwrap_or_else(|| "sqlite:./anyform.db?mode=rwc".to_string());

    match cli.command {
        Commands::Init {
            path,
            database,
            force,
            seed,
        } => {
            // Determine the database URL
            let db_url = if let Some(url) = database {
                url
            } else {
                // Create parent directories if needed
                if let Some(parent) = path.parent() {
                    if !parent.as_os_str().is_empty() && !parent.exists() {
                        std::fs::create_dir_all(parent)?;
                        println!("Created directory: {}", parent.display());
                    }
                }

                // Check if file exists and --force not set
                if path.exists() && !force {
                    anyhow::bail!(
                        "Database file already exists: {}\nUse --force to overwrite.",
                        path.display()
                    );
                }

                // Remove existing file if --force
                if path.exists() && force {
                    std::fs::remove_file(&path)?;
                    println!("Removed existing database: {}", path.display());
                }

                format!("sqlite:{}?mode=rwc", path.display())
            };

            println!("Initializing database...");
            let db = Database::connect(&db_url).await?;

            // Run migrations
            println!("Running migrations...");
            migration::Migrator::up(&db, None).await?;

            // Seed if requested
            if seed {
                println!("Seeding example forms...");
                anyform::seed_all(&db).await?;
                println!("Created: contact, feedback, quiz");
            }

            println!("Done! Database initialized at: {}", db_url);
        }

        Commands::Migrate {
            up,
            down,
            status,
            rename_tables,
        } => {
            let db = connect(&database_url).await?;

            if status {
                println!("Migration status:");
                let migrations = migration::Migrator::get_pending_migrations(&db).await?;
                if migrations.is_empty() {
                    println!("  All migrations applied.");
                } else {
                    println!("  Pending migrations:");
                    for m in migrations {
                        println!("    - {}", m.name());
                    }
                }
            } else if down {
                println!("Rolling back last migration...");
                migration::Migrator::down(&db, Some(1)).await?;
                println!("Done.");
            } else if rename_tables {
                // Run just the rename migration
                println!("Renaming tables from asf_ to af_...");
                migration::Migrator::up(&db, None).await?;
                println!("Done. Tables renamed to af_ prefix.");
            } else if up {
                println!("Running migrations...");
                migration::Migrator::up(&db, None).await?;
                println!("Done.");
            }
        }

        Commands::Form { action } => {
            let db = connect(&database_url).await?;
            commands::form::handle(&db, action).await?;
        }

        Commands::Submissions { action } => {
            let db = connect(&database_url).await?;
            commands::submissions::handle(&db, action).await?;
        }

        Commands::Seed {
            contact_only,
            feedback_only,
            quiz_only,
            clear,
        } => {
            let db = connect(&database_url).await?;

            if clear {
                println!("Clearing seeded forms...");
                anyform::clear_seeded_forms(&db).await?;
                println!("Done.");
            } else if contact_only {
                println!("Seeding contact form...");
                anyform::seed_contact_form(&db).await?;
                println!("Done.");
            } else if feedback_only {
                println!("Seeding feedback form...");
                anyform::seed_feedback_form(&db).await?;
                println!("Done.");
            } else if quiz_only {
                println!("Seeding quiz form...");
                anyform::seed_quiz_form(&db).await?;
                println!("Done.");
            } else {
                println!("Seeding all example forms...");
                anyform::seed_all(&db).await?;
                println!("Done. Created: contact, feedback, quiz");
            }
        }

        Commands::Serve {
            host,
            port,
            no_admin,
            cors,
        } => {
            let db = connect(&database_url).await?;

            // Run migrations first
            println!("Running migrations...");
            migration::Migrator::up(&db, None).await?;

            // Build anyform router
            let mut builder = AnyFormRouter::builder().database(db);
            if !no_admin {
                builder = builder.enable_admin(true);
            }
            let anyform_router = builder.build();

            // Health check endpoint
            let health_router = Router::new().route("/health", get(health_check));

            // Combine routers
            let mut app = Router::new().merge(anyform_router).merge(health_router);

            // Add CORS if specified
            if let Some(origin) = cors {
                let cors_layer = if origin == "*" {
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                } else {
                    CorsLayer::new()
                        .allow_origin(origin.parse::<http::HeaderValue>()?)
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                };
                app = app.layer(ServiceBuilder::new().layer(cors_layer));
                println!("CORS enabled for: {}", if origin == "*" { "any origin" } else { &origin });
            }

            let addr = format!("{host}:{port}");
            println!("Starting server at http://{addr}");
            println!("Health check: http://{addr}/health");
            if !no_admin {
                println!("Admin routes enabled at /api/admin/*");
            }
            println!("Press Ctrl+C to stop");
            let listener = TcpListener::bind(&addr).await?;
            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}

/// Health check endpoint handler.
async fn health_check() -> &'static str {
    "OK"
}

async fn connect(url: &str) -> Result<DatabaseConnection> {
    println!("Connecting to database...");
    let db = Database::connect(url).await?;
    Ok(db)
}

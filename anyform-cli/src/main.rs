//! anyform CLI tool.

use anyhow::Result;
use anyform::FormsRouter;
use clap::{Parser, Subcommand};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tokio::net::TcpListener;

mod commands;

#[derive(Parser)]
#[command(name = "asf")]
#[command(author, version, about = "anyform CLI tool", long_about = None)]
struct Cli {
    /// Database URL (or set DATABASE_URL env var)
    #[arg(short, long, env = "DATABASE_URL")]
    database_url: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
    },
}

#[derive(Subcommand)]
pub enum FormAction {
    /// List all forms
    List,

    /// Show form details
    Show {
        /// Form slug or ID
        slug: String,
    },

    /// Create a form from JSON file
    Create {
        /// Path to JSON file
        #[arg(short, long)]
        file: String,
    },

    /// Update a form from JSON file
    Update {
        /// Form slug
        slug: String,

        /// Path to JSON file
        #[arg(short, long)]
        file: String,
    },

    /// Delete a form
    Delete {
        /// Form slug
        slug: String,
    },

    /// Export a form to JSON
    Export {
        /// Form slug
        slug: String,

        /// Output format
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Render form HTML
    Render {
        /// Form slug
        slug: String,
    },

    /// Sync forms from a folder
    Sync {
        /// Path to folder containing JSON form files
        #[arg(short, long)]
        folder: String,
    },
}

#[derive(Subcommand)]
pub enum SubmissionAction {
    /// List submissions for a form
    List {
        /// Form slug
        #[arg(long)]
        form: String,

        /// Maximum number of submissions to show
        #[arg(short, long, default_value = "50")]
        limit: usize,
    },

    /// Show a specific submission
    Show {
        /// Submission ID
        id: String,
    },

    /// Delete a submission
    Delete {
        /// Submission ID
        id: String,
    },

    /// Export submissions to CSV
    Export {
        /// Form slug
        #[arg(long)]
        form: String,

        /// Output format (csv, json)
        #[arg(short, long, default_value = "csv")]
        format: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Get database URL
    let database_url = cli
        .database_url
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .unwrap_or_else(|| "sqlite:./forms.db?mode=rwc".to_string());

    match cli.command {
        Commands::Migrate { up, down, status } => {
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

        Commands::Serve { host, port } => {
            let db = connect(&database_url).await?;

            // Run migrations first
            println!("Running migrations...");
            migration::Migrator::up(&db, None).await?;

            let app = FormsRouter::builder()
                .database(db)
                .enable_admin(true)
                .build();

            let addr = format!("{host}:{port}");
            println!("Starting server at http://{addr}");
            println!("Press Ctrl+C to stop");
            let listener = TcpListener::bind(&addr).await?;
            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}

async fn connect(url: &str) -> Result<DatabaseConnection> {
    println!("Connecting to database...");
    let db = Database::connect(url).await?;
    Ok(db)
}

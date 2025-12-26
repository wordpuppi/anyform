# axum-sea-forms

Database-driven dynamic forms for Axum and SeaORM.

## Features

- **Schema-driven forms**: Define forms in the database, not code
- **Multiple output formats**: JSON, HTML, Tera templates
- **Multi-step wizards**: Progress tracking with conditional logic
- **Survey & quiz support**: Scoring, results, analytics
- **Multi-database**: SQLite, PostgreSQL, MySQL via SeaORM

## Quick Start

```bash
# Add to your Cargo.toml
cargo add axum-sea-forms
```

### Basic Usage

```rust
use axum::{Router, Extension};
use axum_sea_forms::{FormsRouter, Migrator};
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

#[tokio::main]
async fn main() {
    let db = Database::connect("sqlite:forms.db?mode=rwc").await.unwrap();

    // Run migrations
    Migrator::up(&db, None).await.unwrap();

    // Mount the forms router
    let app = Router::new()
        .merge(FormsRouter::new(db.clone()))
        .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Routes

| Method | Path | Description |
|--------|------|-------------|
| GET | `/forms/{slug}` | Render form HTML |
| GET | `/forms/{slug}/json` | Get form schema JSON |
| POST | `/forms/{slug}` | Submit form (JSON response) |
| POST | `/forms/{slug}/submit` | Submit form (redirect) |
| GET | `/forms/{slug}/success` | Success page |

## CLI Tool

```bash
# Install
cargo install axum-sea-forms-cli

# Initialize database
asf migrate --database-url "sqlite:forms.db?mode=rwc"

# List forms
asf form list

# Export form
asf form export contact --format json

# List submissions
asf submissions list --form contact

# Export to CSV
asf submissions export --form contact --format csv > leads.csv
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `json` | JSON serialization (default) |
| `tera` | Tera template context builder (default) |
| `handlers` | Pre-built Axum handlers |
| `router` | FormsRouter builder |
| `admin` | Admin CRUD routes |
| `full` | All features |

## Loco.rs Integration

```rust
use loco_rs::prelude::*;
use axum_sea_forms::{Form, FormSubmission, TeraRenderer, validate_submission};

pub async fn show(
    Path(slug): Path<String>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let form = Form::find_by_slug(&ctx.db, &slug).await?;
    let tera_ctx = TeraRenderer::context(&ctx.db, &form).await?;
    format::render().view(&v, "forms/show.html", tera_ctx)
}
```

## Database Schema

Tables created by migrations:

- `asf_forms` - Form definitions
- `asf_steps` - Multi-step form steps
- `asf_fields` - Form fields
- `asf_field_options` - Options for select/radio/checkbox
- `asf_submissions` - Form submissions
- `asf_results` - Quiz result buckets

## License

MIT OR Apache-2.0

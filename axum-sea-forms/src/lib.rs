//! # axum-sea-forms
//!
//! Database-driven dynamic forms for Axum and SeaORM.
//!
//! ## Features
//!
//! - **Schema-driven forms**: Define forms in the database, not code
//! - **Multiple output formats**: JSON, HTML, Tera templates
//! - **Multi-step wizards**: Progress tracking with conditional logic
//! - **Survey & quiz support**: Scoring, results, analytics
//! - **Multi-database**: SQLite, PostgreSQL, MySQL via SeaORM
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use axum::{Router, Extension};
//! use axum_sea_forms::FormsRouter;
//! use sea_orm::Database;
//!
//! #[tokio::main]
//! async fn main() {
//!     let db = Database::connect("sqlite:forms.db").await.unwrap();
//!
//!     let app = Router::new()
//!         .merge(FormsRouter::new(db.clone()))
//!         .layer(Extension(db));
//!
//!     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//!     axum::serve(listener, app).await.unwrap();
//! }
//! ```

pub mod entities;
pub mod error;
pub mod extractors;
pub mod response;
pub mod schema;
pub mod seed;
pub mod services;
pub mod validation;

#[cfg(feature = "json")]
pub mod render;

#[cfg(feature = "handlers")]
pub mod handlers;

#[cfg(feature = "router")]
mod router;

// Re-export migrations
pub use migration::{Migrator, MigratorTrait};

// Re-export core types
pub use entities::{
    field::{ActiveModel as FieldActiveModel, Entity as FieldEntity, Model as Field},
    field_option::{
        ActiveModel as FieldOptionActiveModel, Entity as FieldOptionEntity, Model as FieldOption,
    },
    form::{ActiveModel as FormActiveModel, Entity as FormEntity, Model as Form},
    result::{ActiveModel as ResultActiveModel, Entity as ResultEntity, Model as FormResult},
    step::{ActiveModel as StepActiveModel, Entity as StepEntity, Model as Step},
    submission::{
        ActiveModel as SubmissionActiveModel, Entity as SubmissionEntity, Model as Submission,
    },
};

// Re-export schema types
pub use schema::{
    FieldValue, FormSettings, ScaleLabels, UiOptions, ValidationRules, ValueType,
};

// Re-export condition types from core
pub use axum_sea_forms_core::{ConditionOp, ConditionRule};

// Re-export seeding functions
pub use seed::{
    clear_seeded_forms, seed_all, seed_contact_form, seed_feedback_form, seed_quiz_form,
};

// Re-export error types
pub use error::{FormError, IntoApiError, StepValidationErrors, ValidationErrors};

// Re-export validation
pub use validation::{
    is_field_visible, is_step_visible, validate_field, validate_multi_step_submission,
    validate_step, validate_submission,
};

// Re-export services
pub use services::{CreateFieldInput, CreateFormInput, CreateOptionInput, CreateStepInput, FormBuilder};

// Re-export extractors
pub use extractors::{FormSubmission, RequestId, ValidatedSubmission};

// Re-export response types
pub use response::{ApiError, ApiResponse, PaginationInfo};

// Re-export renderers
#[cfg(feature = "json")]
pub use render::{FormJson, HtmlOptions, HtmlRenderer, JsonRenderer};

#[cfg(feature = "tera")]
pub use render::TeraRenderer;

// Re-export router
#[cfg(feature = "router")]
pub use router::{FormsRouter, FormsRouterBuilder};

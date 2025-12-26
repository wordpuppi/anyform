//! CLI command handlers.

use clap::Subcommand;

pub mod form;
pub mod submissions;

/// Form subcommand actions.
#[derive(Subcommand, Clone)]
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

    /// Set form action URL (where the form submits to)
    SetAction {
        /// Form slug
        slug: String,

        /// External URL to submit form data to (empty string to clear)
        #[arg(long)]
        url: Option<String>,

        /// HTTP method (POST, PUT, PATCH)
        #[arg(long)]
        method: Option<String>,
    },
}

/// Submission subcommand actions.
#[derive(Subcommand, Clone)]
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

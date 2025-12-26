//! Typed response structs for handler responses.

use serde::{Deserialize, Serialize};

/// Response data for form creation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormCreated {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: String,
}

/// Response data for form update.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormUpdated {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub updated_at: String,
}

/// Summary of a form for list responses.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormSummary {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Response data for form list.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormList {
    pub forms: Vec<FormSummary>,
    pub count: usize,
}

/// Response data for form sync.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncResult {
    pub created: usize,
    pub updated: usize,
    pub errors: Vec<String>,
}

/// Response data for form submission.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmissionCreated {
    pub submission_id: String,
    pub message: String,
}

/// Response data for submission retrieval.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmissionData {
    pub id: String,
    pub form_id: String,
    pub data: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub completed_at: Option<String>,
    pub score: Option<i32>,
    pub max_score: Option<i32>,
    pub result_key: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Summary of a submission for list responses.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmissionSummary {
    pub id: String,
    pub data: serde_json::Value,
    pub completed_at: Option<String>,
    pub score: Option<i32>,
    pub created_at: String,
}

/// Response data for submission list.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmissionList {
    pub submissions: Vec<SubmissionSummary>,
    pub count: usize,
}

/// Response for delete operations.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deleted {
    pub message: String,
}

impl Deleted {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn form() -> Self {
        Self::new("Form deleted")
    }

    pub fn submission() -> Self {
        Self::new("Submission deleted")
    }
}

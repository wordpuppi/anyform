//! Axum extractors for form handling.

mod form_submission;
mod request_id;
mod validated_submission;

pub use form_submission::FormSubmission;
pub use request_id::RequestId;
pub use validated_submission::ValidatedSubmission;

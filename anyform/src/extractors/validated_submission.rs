//! Validated form submission extractor.

use axum::extract::{FromRequest, FromRequestParts, Path, Request};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

use crate::entities::{field, step};
use crate::error::{FormError, ValidationErrors};
use crate::schema::FieldValue;
use crate::validation::validate_submission;

use super::FormSubmission;

/// Extractor that validates form submission data against the form schema.
///
/// This extractor:
/// 1. Extracts the form slug from the URL path
/// 2. Loads the form and its fields from the database
/// 3. Parses the submission data
/// 4. Validates against the form schema
///
/// # Example
///
/// ```rust,ignore
/// use axum_sea_forms::ValidatedSubmission;
///
/// async fn handle_submit(
///     ValidatedSubmission { form_id, data, errors }: ValidatedSubmission,
/// ) -> impl IntoResponse {
///     if !errors.is_empty() {
///         // Re-render form with errors
///     }
///     // Save the submission
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ValidatedSubmission {
    /// The form ID.
    pub form_id: uuid::Uuid,
    /// The form slug.
    pub slug: String,
    /// The submitted data.
    pub data: HashMap<String, FieldValue>,
    /// Validation errors, if any.
    pub errors: ValidationErrors,
}

/// Path parameters for form submission.
#[derive(Debug, serde::Deserialize)]
struct FormPath {
    slug: String,
}

impl<S> FromRequest<S> for ValidatedSubmission
where
    S: Send + Sync,
    DatabaseConnection: FromRequestParts<S>,
{
    type Rejection = FormError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract parts for path and database
        let (mut parts, body) = req.into_parts();

        // Extract the slug from path
        let Path(FormPath { slug }) = Path::from_request_parts(&mut parts, state)
            .await
            .map_err(|_| FormError::InvalidData("Missing form slug in path".to_string()))?;

        // Get the database connection
        let db: DatabaseConnection = DatabaseConnection::from_request_parts(&mut parts, state)
            .await
            .map_err(|_| FormError::Database("Database connection not available".to_string()))?;

        // Reconstruct the request
        let req = Request::from_parts(parts, body);

        // Parse the form data
        let FormSubmission(data) = FormSubmission::from_request(req, state).await?;

        // Load the form
        let form = crate::entities::form::Entity::find_by_slug(&db, &slug)
            .await?
            .ok_or_else(|| FormError::NotFound(slug.clone()))?;

        if form.is_deleted() {
            return Err(FormError::FormDeleted);
        }

        // Load all fields for the form
        let steps = step::Entity::find_by_form(&db, form.id).await?;
        let mut all_fields = Vec::new();

        for step in &steps {
            let fields = field::Entity::find_by_step(&db, step.id).await?;
            all_fields.extend(fields);
        }

        // Validate the submission
        let errors = validate_submission(&all_fields, &data);

        Ok(Self {
            form_id: form.id,
            slug,
            data,
            errors,
        })
    }
}

impl ValidatedSubmission {
    /// Returns true if there are no validation errors.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Gets a value by field name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&FieldValue> {
        self.data.get(name)
    }

    /// Gets a string value by field name.
    #[must_use]
    pub fn get_str(&self, name: &str) -> Option<&str> {
        self.data.get(name).and_then(FieldValue::as_str)
    }
}

//! Axum route handlers for forms.

mod responses;

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::entities::{field, form, step, submission};
use crate::error::FormError;
use crate::extractors::{FormSubmission, RequestId};
use crate::render::{FormJson, HtmlOptions, HtmlRenderer, JsonRenderer};
use crate::response::ApiResponse;
#[cfg(feature = "admin")]
use crate::services::{CreateFormInput, FormBuilder};
use crate::validation::validate_submission;

pub use responses::*;

/// Gets a form by slug and returns its JSON schema.
pub async fn get_form_json(
    Path(slug): Path<String>,
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, FormError> {
    let form = form::Entity::find_by_slug(&db, &slug)
        .await?
        .ok_or_else(|| FormError::NotFound(slug))?;

    if form.is_deleted() {
        return Err(FormError::FormDeleted);
    }

    let json = JsonRenderer::render(&db, &form).await?;
    Ok(Json(json))
}

/// Gets a form by slug and returns its HTML.
pub async fn get_form_html(
    Path(slug): Path<String>,
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, FormError> {
    let form = form::Entity::find_by_slug(&db, &slug)
        .await?
        .ok_or_else(|| FormError::NotFound(slug))?;

    if form.is_deleted() {
        return Err(FormError::FormDeleted);
    }

    let html = HtmlRenderer::render(&db, &form, &HtmlOptions::new()).await?;
    Ok(Html(html))
}

/// Submits a form.
pub async fn submit_form(
    Path(slug): Path<String>,
    State(db): State<DatabaseConnection>,
    RequestId(request_id): RequestId,
    FormSubmission(data): FormSubmission,
) -> Result<ApiResponse<SubmissionCreated>, ApiResponse<()>> {
    let form = form::Entity::find_by_slug(&db, &slug)
        .await
        .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?
        .ok_or_else(|| ApiResponse::<()>::from(FormError::NotFound(slug.clone())))?;

    if form.is_deleted() {
        return Err(FormError::FormDeleted.into());
    }

    // Load all fields for validation
    let steps = step::Entity::find_by_form(&db, form.id)
        .await
        .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?;
    let mut all_fields = Vec::new();
    for s in &steps {
        let fields = field::Entity::find_by_step(&db, s.id)
            .await
            .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?;
        all_fields.extend(fields);
    }

    // Validate
    let errors = validate_submission(&all_fields, &data);
    if !errors.is_empty() {
        return Err(FormError::ValidationFailed(errors).into());
    }

    // Create submission
    let now = chrono::Utc::now().fixed_offset();
    let sub = submission::ActiveModel {
        id: Set(Uuid::new_v4()),
        form_id: Set(form.id),
        data: Set(serde_json::to_value(&data).unwrap_or_default()),
        metadata: Set(None),
        current_step_id: Set(None),
        completed_at: Set(Some(now)),
        score: Set(None),
        max_score: Set(None),
        result_key: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        deleted_at: Set(None),
    };

    let saved = sub
        .insert(&db)
        .await
        .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?;

    Ok(ApiResponse::created(SubmissionCreated {
        submission_id: saved.id.to_string(),
        message: form
            .settings()
            .success_message
            .unwrap_or_else(|| "Form submitted successfully".to_string()),
    })
    .with_request_id(request_id))
}

/// Submits a form and redirects (for SSR).
pub async fn submit_form_redirect(
    Path(slug): Path<String>,
    State(db): State<DatabaseConnection>,
    FormSubmission(data): FormSubmission,
) -> Result<impl IntoResponse, FormError> {
    let form = form::Entity::find_by_slug(&db, &slug)
        .await?
        .ok_or_else(|| FormError::NotFound(slug.clone()))?;

    if form.is_deleted() {
        return Err(FormError::FormDeleted);
    }

    // Load all fields for validation
    let steps = step::Entity::find_by_form(&db, form.id).await?;
    let mut all_fields = Vec::new();
    for s in &steps {
        let fields = field::Entity::find_by_step(&db, s.id).await?;
        all_fields.extend(fields);
    }

    // Validate
    let errors = validate_submission(&all_fields, &data);
    if !errors.is_empty() {
        // Re-render form with errors
        let html = HtmlRenderer::render_with_values(
            &db,
            &form,
            &HtmlOptions::new(),
            &data,
            &errors,
        )
        .await?;
        return Ok(Html(html).into_response());
    }

    // Create submission
    let now = chrono::Utc::now().fixed_offset();
    let submission = submission::ActiveModel {
        id: Set(Uuid::new_v4()),
        form_id: Set(form.id),
        data: Set(serde_json::to_value(&data).unwrap_or_default()),
        metadata: Set(None),
        current_step_id: Set(None),
        completed_at: Set(Some(now)),
        score: Set(None),
        max_score: Set(None),
        result_key: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        deleted_at: Set(None),
    };

    submission.insert(&db).await?;

    // Redirect to success page or custom URL
    let redirect_url = form
        .settings()
        .redirect_url
        .unwrap_or_else(|| format!("/forms/{}/success", slug));

    Ok(Redirect::to(&redirect_url).into_response())
}

/// Success page after form submission.
pub async fn form_success(
    Path(slug): Path<String>,
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, FormError> {
    let form = form::Entity::find_by_slug(&db, &slug)
        .await?
        .ok_or_else(|| FormError::NotFound(slug))?;

    let message = form
        .settings()
        .success_message
        .unwrap_or_else(|| "Thank you! Your submission has been received.".to_string());

    Ok(Html(format!(
        r#"<!DOCTYPE html>
<html>
<head><title>Success</title></head>
<body>
<h1>Success</h1>
<p>{message}</p>
<p><a href="/forms/{slug}">Submit another response</a></p>
</body>
</html>"#,
        slug = form.slug
    )))
}

// Admin handlers

/// Lists all forms (admin).
#[cfg(feature = "admin")]
pub async fn list_forms(
    State(db): State<DatabaseConnection>,
    RequestId(request_id): RequestId,
) -> Result<ApiResponse<FormList>, ApiResponse<()>> {
    let forms = form::Entity::find_active(&db)
        .await
        .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?;

    let forms_list: Vec<FormSummary> = forms
        .into_iter()
        .map(|f| FormSummary {
            id: f.id.to_string(),
            name: f.name,
            slug: f.slug,
            description: f.description,
            created_at: f.created_at.to_rfc3339(),
            updated_at: f.updated_at.to_rfc3339(),
        })
        .collect();

    let count = forms_list.len();
    Ok(ApiResponse::ok(FormList {
        forms: forms_list,
        count,
    })
    .with_request_id(request_id))
}

/// Gets a form by ID (admin).
#[cfg(feature = "admin")]
pub async fn get_form_by_id(
    Path(id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
    RequestId(request_id): RequestId,
) -> Result<ApiResponse<FormJson>, ApiResponse<()>> {
    let form = form::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?
        .ok_or_else(|| ApiResponse::<()>::from(FormError::NotFound(id.to_string())))?;

    let json = JsonRenderer::render(&db, &form)
        .await
        .map_err(|e| ApiResponse::<()>::from(e))?;
    Ok(ApiResponse::ok(json).with_request_id(request_id))
}

/// Lists submissions for a form (admin).
#[cfg(feature = "admin")]
pub async fn list_submissions(
    Path(form_id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
    RequestId(request_id): RequestId,
) -> Result<ApiResponse<SubmissionList>, ApiResponse<()>> {
    let submissions = submission::Entity::find_by_form(&db, form_id)
        .await
        .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?;

    let submissions_list: Vec<SubmissionSummary> = submissions
        .into_iter()
        .map(|s| SubmissionSummary {
            id: s.id.to_string(),
            data: s.data,
            completed_at: s.completed_at.map(|d| d.to_rfc3339()),
            score: s.score,
            created_at: s.created_at.to_rfc3339(),
        })
        .collect();

    let count = submissions_list.len();
    Ok(ApiResponse::ok(SubmissionList {
        submissions: submissions_list,
        count,
    })
    .with_request_id(request_id))
}

/// Creates a new form (admin).
#[cfg(feature = "admin")]
pub async fn create_form(
    State(db): State<DatabaseConnection>,
    RequestId(request_id): RequestId,
    Json(input): Json<CreateFormInput>,
) -> Result<ApiResponse<FormCreated>, ApiResponse<()>> {
    let form = FormBuilder::create(&db, input)
        .await
        .map_err(|e| ApiResponse::<()>::from(e))?;

    Ok(ApiResponse::created(FormCreated {
        id: form.id.to_string(),
        name: form.name,
        slug: form.slug,
        description: form.description,
        created_at: form.created_at.to_rfc3339(),
    })
    .with_request_id(request_id))
}

/// Updates an existing form (admin).
#[cfg(feature = "admin")]
pub async fn update_form(
    Path(id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
    RequestId(request_id): RequestId,
    Json(input): Json<CreateFormInput>,
) -> Result<ApiResponse<FormUpdated>, ApiResponse<()>> {
    let form = FormBuilder::update(&db, id, input)
        .await
        .map_err(|e| ApiResponse::<()>::from(e))?;

    Ok(ApiResponse::ok(FormUpdated {
        id: form.id.to_string(),
        name: form.name,
        slug: form.slug,
        description: form.description,
        updated_at: form.updated_at.to_rfc3339(),
    })
    .with_request_id(request_id))
}

/// Deletes a form (admin).
#[cfg(feature = "admin")]
pub async fn delete_form(
    Path(id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
    RequestId(request_id): RequestId,
) -> Result<ApiResponse<Deleted>, ApiResponse<()>> {
    FormBuilder::soft_delete(&db, id)
        .await
        .map_err(|e| ApiResponse::<()>::from(e))?;

    Ok(ApiResponse::ok(Deleted::form()).with_request_id(request_id))
}

/// Gets a specific submission (admin).
#[cfg(feature = "admin")]
pub async fn get_submission(
    Path((form_id, sub_id)): Path<(Uuid, Uuid)>,
    State(db): State<DatabaseConnection>,
    RequestId(request_id): RequestId,
) -> Result<ApiResponse<SubmissionData>, ApiResponse<()>> {
    // Verify form exists
    let _form = form::Entity::find_by_id(form_id)
        .one(&db)
        .await
        .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?
        .ok_or_else(|| ApiResponse::<()>::from(FormError::NotFound(form_id.to_string())))?;

    let sub = submission::Entity::find_active_by_id(&db, sub_id)
        .await
        .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?
        .ok_or_else(|| {
            ApiResponse::<()>::from(FormError::SubmissionNotFound(sub_id.to_string()))
        })?;

    // Verify submission belongs to form
    if sub.form_id != form_id {
        return Err(FormError::SubmissionNotFound(sub_id.to_string()).into());
    }

    Ok(ApiResponse::ok(SubmissionData {
        id: sub.id.to_string(),
        form_id: sub.form_id.to_string(),
        data: sub.data,
        metadata: sub.metadata,
        completed_at: sub.completed_at.map(|d| d.to_rfc3339()),
        score: sub.score,
        max_score: sub.max_score,
        result_key: sub.result_key,
        created_at: sub.created_at.to_rfc3339(),
        updated_at: sub.updated_at.to_rfc3339(),
    })
    .with_request_id(request_id))
}

/// Deletes a specific submission (admin).
#[cfg(feature = "admin")]
pub async fn delete_submission(
    Path((form_id, sub_id)): Path<(Uuid, Uuid)>,
    State(db): State<DatabaseConnection>,
    RequestId(request_id): RequestId,
) -> Result<ApiResponse<Deleted>, ApiResponse<()>> {
    // Verify form exists
    let _form = form::Entity::find_by_id(form_id)
        .one(&db)
        .await
        .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?
        .ok_or_else(|| ApiResponse::<()>::from(FormError::NotFound(form_id.to_string())))?;

    let sub = submission::Entity::find_active_by_id(&db, sub_id)
        .await
        .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?
        .ok_or_else(|| {
            ApiResponse::<()>::from(FormError::SubmissionNotFound(sub_id.to_string()))
        })?;

    // Verify submission belongs to form
    if sub.form_id != form_id {
        return Err(FormError::SubmissionNotFound(sub_id.to_string()).into());
    }

    submission::Entity::soft_delete(&db, sub_id)
        .await
        .map_err(|e| ApiResponse::<()>::from(FormError::from(e)))?;

    Ok(ApiResponse::ok(Deleted::submission()).with_request_id(request_id))
}

/// Syncs multiple forms (admin).
/// Creates new forms or updates existing ones based on slug.
#[cfg(feature = "admin")]
pub async fn sync_forms(
    State(db): State<DatabaseConnection>,
    RequestId(request_id): RequestId,
    Json(forms): Json<Vec<CreateFormInput>>,
) -> ApiResponse<SyncResult> {
    let mut created = 0;
    let mut updated = 0;
    let mut errors: Vec<String> = Vec::new();

    for input in forms {
        let slug = input.slug.clone();

        match FormBuilder::find_by_slug(&db, &slug).await {
            Ok(Some(existing)) => {
                match FormBuilder::update(&db, existing.id, input).await {
                    Ok(_) => updated += 1,
                    Err(e) => errors.push(format!("{}: {}", slug, e)),
                }
            }
            Ok(None) => {
                match FormBuilder::create(&db, input).await {
                    Ok(_) => created += 1,
                    Err(e) => errors.push(format!("{}: {}", slug, e)),
                }
            }
            Err(e) => errors.push(format!("{}: {}", slug, e)),
        }
    }

    ApiResponse::ok(SyncResult {
        created,
        updated,
        errors,
    })
    .with_request_id(request_id)
}

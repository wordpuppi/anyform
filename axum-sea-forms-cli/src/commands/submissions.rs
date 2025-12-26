//! Submissions CLI commands.

use anyhow::Result;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use axum_sea_forms::entities::{
    form::Entity as FormEntity,
    submission::Entity as SubmissionEntity,
};

use crate::SubmissionAction;

pub async fn handle(db: &DatabaseConnection, action: SubmissionAction) -> Result<()> {
    match action {
        SubmissionAction::List { form, limit } => list(db, &form, limit).await,
        SubmissionAction::Show { id } => show(db, &id).await,
        SubmissionAction::Delete { id } => delete(db, &id).await,
        SubmissionAction::Export { form, format } => export(db, &form, &format).await,
    }
}

async fn list(db: &DatabaseConnection, form_slug: &str, limit: usize) -> Result<()> {
    let form = FormEntity::find_by_slug(db, form_slug)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Form not found: {}", form_slug))?;

    let submissions = SubmissionEntity::find_completed_by_form(db, form.id).await?;

    if submissions.is_empty() {
        println!("No submissions found for form '{}'.", form_slug);
        return Ok(());
    }

    println!("Submissions for form '{}' ({}):", form.name, form.slug);
    println!(
        "{:<36}  {:<20}  {}",
        "ID", "CREATED", "COMPLETED"
    );
    println!("{}", "-".repeat(80));

    for sub in submissions.into_iter().take(limit) {
        let completed = sub
            .completed_at
            .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "In progress".to_string());

        println!(
            "{:<36}  {:<20}  {}",
            sub.id,
            sub.created_at.format("%Y-%m-%d %H:%M"),
            completed
        );
    }

    Ok(())
}

async fn export(db: &DatabaseConnection, form_slug: &str, format: &str) -> Result<()> {
    let form = FormEntity::find_by_slug(db, form_slug)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Form not found: {}", form_slug))?;

    let submissions = SubmissionEntity::find_completed_by_form(db, form.id).await?;

    match format.to_lowercase().as_str() {
        "json" => {
            let json: Vec<_> = submissions
                .iter()
                .map(|s| {
                    serde_json::json!({
                        "id": s.id.to_string(),
                        "data": s.data,
                        "completed_at": s.completed_at.map(|d| d.to_rfc3339()),
                        "created_at": s.created_at.to_rfc3339(),
                    })
                })
                .collect();

            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        "csv" => {
            // Get all unique field names from submissions
            let mut field_names: Vec<String> = Vec::new();
            for sub in &submissions {
                if let Some(obj) = sub.data.as_object() {
                    for key in obj.keys() {
                        if !field_names.contains(key) {
                            field_names.push(key.clone());
                        }
                    }
                }
            }

            field_names.sort();

            // Header
            print!("id,created_at,completed_at");
            for name in &field_names {
                print!(",{}", escape_csv(name));
            }
            println!();

            // Rows
            for sub in &submissions {
                print!(
                    "{},{},{}",
                    sub.id,
                    sub.created_at.format("%Y-%m-%d %H:%M:%S"),
                    sub.completed_at
                        .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_default()
                );

                for name in &field_names {
                    let value = sub
                        .data
                        .get(name)
                        .map(|v| match v {
                            serde_json::Value::String(s) => s.clone(),
                            serde_json::Value::Null => String::new(),
                            other => other.to_string(),
                        })
                        .unwrap_or_default();

                    print!(",{}", escape_csv(&value));
                }
                println!();
            }
        }
        _ => {
            anyhow::bail!("Unsupported format: {}. Use 'json' or 'csv'.", format);
        }
    }

    Ok(())
}

async fn show(db: &DatabaseConnection, id: &str) -> Result<()> {
    let uuid = Uuid::parse_str(id)
        .map_err(|_| anyhow::anyhow!("Invalid submission ID: {}", id))?;

    let sub = SubmissionEntity::find_active_by_id(db, uuid)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Submission not found: {}", id))?;

    let json = serde_json::json!({
        "id": sub.id.to_string(),
        "form_id": sub.form_id.to_string(),
        "data": sub.data,
        "metadata": sub.metadata,
        "completed_at": sub.completed_at.map(|d| d.to_rfc3339()),
        "score": sub.score,
        "max_score": sub.max_score,
        "result_key": sub.result_key,
        "created_at": sub.created_at.to_rfc3339(),
        "updated_at": sub.updated_at.to_rfc3339(),
    });

    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}

async fn delete(db: &DatabaseConnection, id: &str) -> Result<()> {
    let uuid = Uuid::parse_str(id)
        .map_err(|_| anyhow::anyhow!("Invalid submission ID: {}", id))?;

    let _sub = SubmissionEntity::find_active_by_id(db, uuid)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Submission not found: {}", id))?;

    SubmissionEntity::soft_delete(db, uuid).await?;

    println!("Submission '{}' deleted successfully.", id);

    Ok(())
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

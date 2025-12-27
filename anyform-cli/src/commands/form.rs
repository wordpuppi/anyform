//! Form CLI commands.

use anyhow::Result;
use sea_orm::DatabaseConnection;

use anyform::{
    entities::form::Entity as FormEntity,
    render::{HtmlOptions, HtmlRenderer, JsonRenderer},
    services::{CreateFormInput, FormBuilder},
};

use crate::FormAction;

pub async fn handle(db: &DatabaseConnection, action: FormAction) -> Result<()> {
    match action {
        FormAction::List => list(db).await,
        FormAction::Show { slug } => show(db, &slug).await,
        FormAction::Create { file } => create(db, &file).await,
        FormAction::Update { slug, file } => update(db, &slug, &file).await,
        FormAction::Delete { slug } => delete(db, &slug).await,
        FormAction::Export { slug, format } => export(db, &slug, &format).await,
        FormAction::Render { slug } => render(db, &slug).await,
        FormAction::Sync { folder } => sync(db, &folder).await,
    }
}

async fn list(db: &DatabaseConnection) -> Result<()> {
    let forms = FormEntity::find_active(db).await?;

    if forms.is_empty() {
        println!("No forms found.");
        return Ok(());
    }

    println!(
        "{:<36}  {:<20}  {:<15}  {}",
        "ID", "NAME", "SLUG", "CREATED"
    );
    println!("{}", "-".repeat(90));

    for form in forms {
        println!(
            "{:<36}  {:<20}  {:<15}  {}",
            form.id,
            truncate(&form.name, 20),
            truncate(&form.slug, 15),
            form.created_at.format("%Y-%m-%d %H:%M")
        );
    }

    Ok(())
}

async fn show(db: &DatabaseConnection, slug: &str) -> Result<()> {
    let form = FormEntity::find_by_slug(db, slug)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Form not found: {}", slug))?;

    let json = JsonRenderer::render_pretty(db, &form).await?;
    println!("{json}");

    Ok(())
}

async fn create(db: &DatabaseConnection, file: &str) -> Result<()> {
    let content = std::fs::read_to_string(file)?;
    let input: CreateFormInput = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid form JSON: {}", e))?;

    let form = FormBuilder::create(db, input).await?;

    println!("Form created successfully!");
    println!("  ID:   {}", form.id);
    println!("  Name: {}", form.name);
    println!("  Slug: {}", form.slug);

    Ok(())
}

async fn update(db: &DatabaseConnection, slug: &str, file: &str) -> Result<()> {
    let form = FormEntity::find_by_slug(db, slug)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Form not found: {}", slug))?;

    let content = std::fs::read_to_string(file)?;
    let input: CreateFormInput = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid form JSON: {}", e))?;

    let updated = FormBuilder::update(db, form.id, input).await?;

    println!("Form updated successfully!");
    println!("  ID:   {}", updated.id);
    println!("  Name: {}", updated.name);
    println!("  Slug: {}", updated.slug);

    Ok(())
}

async fn delete(db: &DatabaseConnection, slug: &str) -> Result<()> {
    let form = FormEntity::find_by_slug(db, slug)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Form not found: {}", slug))?;

    FormBuilder::soft_delete(db, form.id).await?;

    println!("Form '{}' deleted successfully.", slug);

    Ok(())
}

async fn export(db: &DatabaseConnection, slug: &str, format: &str) -> Result<()> {
    let form = FormEntity::find_by_slug(db, slug)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Form not found: {}", slug))?;

    match format.to_lowercase().as_str() {
        "json" => {
            let json = JsonRenderer::render_pretty(db, &form).await?;
            println!("{json}");
        }
        _ => {
            anyhow::bail!("Unsupported format: {}. Use 'json'.", format);
        }
    }

    Ok(())
}

async fn render(db: &DatabaseConnection, slug: &str) -> Result<()> {
    let form = FormEntity::find_by_slug(db, slug)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Form not found: {}", slug))?;

    let html = HtmlRenderer::render(db, &form, &HtmlOptions::new()).await?;
    println!("{html}");

    Ok(())
}

async fn sync(db: &DatabaseConnection, folder: &str) -> Result<()> {
    let pattern = format!("{}/*.json", folder);
    let mut created = 0;
    let mut updated = 0;
    let mut errors = 0;

    let entries: Vec<_> = glob::glob(&pattern)
        .map_err(|e| anyhow::anyhow!("Invalid glob pattern: {}", e))?
        .collect();

    if entries.is_empty() {
        println!("No JSON files found in: {}", folder);
        return Ok(());
    }

    for entry in entries {
        let path = match entry {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error reading path: {}", e);
                errors += 1;
                continue;
            }
        };

        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading {}: {}", path.display(), e);
                errors += 1;
                continue;
            }
        };

        let input: CreateFormInput = match serde_json::from_str(&content) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("Error parsing {}: {}", path.display(), e);
                errors += 1;
                continue;
            }
        };

        let slug = input.slug.clone();

        match FormBuilder::find_by_slug(db, &slug).await? {
            Some(existing) => {
                match FormBuilder::update(db, existing.id, input).await {
                    Ok(_) => {
                        println!("Updated: {} ({})", path.display(), slug);
                        updated += 1;
                    }
                    Err(e) => {
                        eprintln!("Error updating {}: {}", path.display(), e);
                        errors += 1;
                    }
                }
            }
            None => {
                match FormBuilder::create(db, input).await {
                    Ok(_) => {
                        println!("Created: {} ({})", path.display(), slug);
                        created += 1;
                    }
                    Err(e) => {
                        eprintln!("Error creating {}: {}", path.display(), e);
                        errors += 1;
                    }
                }
            }
        }
    }

    println!();
    println!("Sync complete: {} created, {} updated, {} errors", created, updated, errors);

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

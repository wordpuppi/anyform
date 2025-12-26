# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2025-12-XX

### Added

- **Product rename**: axum-sea-forms → anyform
- **Simplified crate structure**: 1 main crate + 1 WASM crate
- **Embedded SQLite support**: Zero-config database
- `anyform init` command for database initialization
- `anyform serve` command with `--cors` and `--no-admin` flags
- Health check endpoint at `/health`
- **WASM client** (`anyform-client`) for browser-side form handling:
  - `FormClient` for fetching and submitting forms
  - `FormState` for client-side state management
  - Client-side condition evaluation (step/field visibility)
  - Client-side validation (mirrors server rules)
  - Multi-step navigation with automatic hidden step skipping
  - Progress tracking for multi-step forms
  - Hydration mode for server-rendered HTML forms
- **npm package** (`anyform-js`) with TypeScript bindings
- **Distribution** via brew, curl, scoop, Docker
- GitHub Actions release workflow with cross-compilation
- Auto-migrations on server startup
- Docker Compose examples (SQLite, PostgreSQL)

### Changed

- Table prefix `asf_` → `af_`
- API routes prefixed with `/api/`
- CLI binary renamed from `asf` to `anyform`
- `FormsRouter` renamed to `AnyFormRouter`
- Crate name: `axum-sea-forms` → `anyform`

### Migration

See [Migration Guide](docs/migration-0.4.0.md) for upgrading from 0.3.x.

**Quick steps:**

1. Update Cargo.toml: `axum-sea-forms` → `anyform`
2. Update imports: `axum_sea_forms` → `anyform`
3. Rename tables: `anyform migrate --rename-tables`
4. Update API URLs: `/forms/` → `/api/forms/`

## [0.3.2] - 2025-12-XX

### Added

- Condition rules for step/field visibility
- Quiz scoring and result buckets
- Form sync from JSON files

### Fixed

- Field ordering in multi-step forms

## [0.3.1] - 2025-12-XX

### Added

- CSV export for submissions
- Bulk form operations

### Fixed

- Validation for nested field options

## [0.3.0] - 2025-12-XX

### Added

- Multi-step form support
- Conditional field visibility
- Progress tracking

### Changed

- Database schema for steps table

## [0.2.0] - 2025-11-XX

### Added

- Admin CRUD routes
- Form builder API
- Tera template integration

## [0.1.0] - 2025-10-XX

### Added

- Initial release
- Basic form rendering (JSON, HTML)
- Form submission handling
- SQLite, PostgreSQL, MySQL support

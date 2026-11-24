# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0] - 2026-11-24

### Changed

- **Breaking:** moved to SeaORM 2.0 and sea-orm-migration 2.0 (sqlx 0.9). Apps that use anyform must be on SeaORM 2 as well.
- Minimum supported Rust version is now 1.94, which SeaORM 2 requires.
- `init_schema` runs its SQL through `execute_raw`, the SeaORM 2 name for raw statements.
- `init_schema` and the `rename_tables_af` migration return `DbErr::BackendNotSupported` for a database backend other than SQLite, PostgreSQL or MySQL, since SeaORM 2's backend enum is no longer exhaustive.
- Tera stays on 1.x, so form templates written for 0.5 still render unchanged.

### Fixed

- Tests and the `basic_form` example import the migrator from `anyform` instead of the old standalone `migration` crate.
- Updated the contact-form JSON snapshot for the `send_confirmation` setting added in 0.5.2.

## [0.4.0] - 2025-12-27

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
- **npm package** (`@wordpuppi/anyform-wasm-js`) with TypeScript bindings
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

## [0.3.2] - 2025-12-15

### Added

- Condition rules for step/field visibility
- Quiz scoring and result buckets
- Form sync from JSON files

### Fixed

- Field ordering in multi-step forms

## [0.3.1] - 2025-12-08

### Added

- CSV export for submissions
- Bulk form operations

### Fixed

- Validation for nested field options

## [0.3.0] - 2025-12-01

### Added

- Multi-step form support
- Conditional field visibility
- Progress tracking

### Changed

- Database schema for steps table

## [0.2.0] - 2025-11-15

### Added

- Admin CRUD routes
- Form builder API
- Tera template integration

## [0.1.0] - 2025-10-28

### Added

- Initial release
- Basic form rendering (JSON, HTML)
- Form submission handling
- SQLite, PostgreSQL, MySQL support

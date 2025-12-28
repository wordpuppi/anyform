# Changelog

All notable changes to @wordpuppi/anyform-next will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.3] - 2025-12-27

### Added

- `AnyFormRSC` - React Server Component for form rendering
  - Fetches form schema on the server
  - Automatic client hydration with WASM validation
  - ISR support with 60-second revalidation
- `AnyFormClient` - Client component for RSC hydration
- Server-side utilities (import from `@wordpuppi/anyform-next/server`):
  - `fetchFormSchema()` - Cached server-side schema fetch using React's `cache()`
  - `fetchFormSchemaNoCache()` - Uncached variant for dynamic forms
  - `submitForm()` - Server Action for form submission
  - `validateForm()` - Server Action for server-side validation
- Environment variable support:
  - `ANYFORM_API_URL` - Default API base URL for server-side fetching
- Re-exports all of `@wordpuppi/anyform-react`:
  - `useAnyForm` hook
  - `AnyForm` component
  - `AnyFormProvider` context
  - All types and utilities

### Usage Patterns

Supports multiple integration patterns:

1. **Full RSC** - Schema fetched on server, hydrated on client
2. **Manual Schema Fetch** - Custom control over data fetching
3. **Client-Only** - Traditional client-side form loading
4. **Server Actions** - Server-side form submission

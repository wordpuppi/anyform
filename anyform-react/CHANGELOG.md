# Changelog

All notable changes to @anyform/react will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.3] - 2025-12-27

### Added

- `useAnyForm` hook for complete form state management
- `AnyForm` component with render props pattern for flexible rendering
- `AnyFormProvider` context for app-wide configuration (baseUrl, tailwind, classNames)
- Field-specific props helpers for all input types:
  - `getFieldProps()` - Generic input fields (text, email, number, etc.)
  - `getSelectProps()` - Select and multi-select dropdowns
  - `getCheckboxProps()` - Checkbox inputs (returns `checked` instead of `value`)
  - `getRadioGroupProps()` - Radio button groups with `getOptionProps()` helper
  - `getTextareaProps()` - Textarea elements
  - `getLabelProps()` - Label elements with proper `htmlFor` binding
  - `getFieldMeta()` - Field metadata (value, errors, touched, visibility)
  - `getFormProps()` - Form element with onSubmit handler
  - `getStepProps()` - Multi-step navigation controls
- Multi-step form navigation:
  - `nextStep()`, `prevStep()`, `goToStep(stepId)`
  - Progress tracking with `step.progress` tuple
  - Step state: `isFirst`, `isLast`, `canGoNext`, `canGoPrev`
- Conditional field/step visibility based on form values
- Tailwind CSS integration (opt-in via `tailwind: true` option)
- Custom class names via `classNames` option
- Full TypeScript support with comprehensive type definitions
- Unit tests with Vitest and React Testing Library

### Architecture

- **Hybrid REST + WASM approach**:
  - REST for initial schema fetch (SSR/RSC compatible)
  - WASM (via anyform-js) for client-side validation and state management
- Form schema loaded from `/api/forms/{slug}/json` endpoint
- Form submission to `/api/forms/{slug}` or custom `action_url`

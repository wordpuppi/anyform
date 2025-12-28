# @wordpuppi/anyform-core

Pure TypeScript form validation, condition evaluation, and state management for **anyform**.

## Installation

```bash
npm install @wordpuppi/anyform-core
```

> **Note:** Most users should install `@wordpuppi/anyform-react` or `@wordpuppi/anyform-next` instead, which include this package automatically.

## When to Use This Package

- Building a custom integration (Vue, Svelte, vanilla JS)
- Need direct access to the validation engine
- Creating your own form state management

For React apps, use `@wordpuppi/anyform-react`. For Next.js, use `@wordpuppi/anyform-next`.

## Quick Start

```typescript
import { FormState } from '@wordpuppi/anyform-core';

// Create form state from schema (fetched from anyform server)
const form = new FormState(schema);

// Set values
form.setValue('email', 'user@example.com');
form.setValue('name', 'John');

// Validate
const errors = form.validateField('email');
if (errors.length === 0) {
  console.log('Email is valid!');
}

// Check visibility (for conditional fields)
if (form.isFieldVisible('company')) {
  // Show company field
}

// Get all values for submission
const values = form.getValues();
```

## API

### FormState

```typescript
const form = new FormState(schema, initialValues?);
```

**Values:**
- `setValue(field, value)` - Set a field value
- `getValue(field)` - Get a field value
- `getValues()` - Get all values

**Validation:**
- `validateField(field)` - Returns array of error messages
- `validateStep(stepId)` - Validate all fields in a step
- `validateAll()` - Validate entire form
- `isValid()` - Check if form has no errors

**Visibility:**
- `isFieldVisible(field)` - Check if field should be shown
- `isStepVisible(stepId)` - Check if step should be shown
- `getVisibleFields(stepId?)` - Get visible fields for step
- `getVisibleSteps()` - Get visible steps

**Navigation (multi-step):**
- `getCurrentStep()` - Get current step
- `nextStep()` - Go to next visible step
- `prevStep()` - Go to previous visible step
- `goToStep(stepId)` - Go to specific step
- `getProgress()` - Returns `[current, total]`

## License

MIT

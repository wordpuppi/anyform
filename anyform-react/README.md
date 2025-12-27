# @anyform/react

React hooks for **anyform** — headless form state, validation, and multi-step navigation.

## Installation

```bash
npm install @anyform/react
```

## Quick Start

```tsx
import { useAnyForm } from '@anyform/react';

function ContactForm() {
  const form = useAnyForm('contact', {
    baseUrl: 'http://localhost:3000',
    tailwind: true,
    onSuccess: (result) => console.log('Submitted!', result),
  });

  if (form.isLoading) return <div>Loading...</div>;
  if (form.error) return <div>Error: {form.error}</div>;

  return (
    <form {...form.getFormProps()}>
      {form.visibleFields.map((field) => (
        <div key={field.name}>
          <label {...form.getLabelProps(field.name)} />
          <input {...form.getFieldProps(field.name)} />
          {form.errors[field.name]?.map((err, i) => (
            <span key={i} className="text-red-500">{err}</span>
          ))}
        </div>
      ))}
      <button type="submit" disabled={form.isSubmitting}>
        {form.isSubmitting ? 'Submitting...' : 'Submit'}
      </button>
    </form>
  );
}
```

## Features

- **Headless** — No styles, bring your own UI
- **WASM-powered validation** — Instant client-side feedback
- **Multi-step forms** — Built-in step navigation
- **Conditional fields** — Show/hide based on other field values
- **Tailwind optional** — Enable with `tailwind: true`
- **TypeScript** — Full type definitions included

## API Reference

### `useAnyForm(slug, options?)`

Main hook for form state management.

```tsx
const form = useAnyForm('my-form', {
  baseUrl: 'http://localhost:3000',  // API base URL
  tailwind: true,                     // Enable Tailwind classes
  initialValues: { email: '' },       // Pre-fill values
  initialSchema: schema,              // SSR hydration (skip fetch)
  validateOnChange: true,             // Validate on change (default: true)
  validateOnBlur: true,               // Validate on blur (default: true)
  onSubmit: async (values) => {},     // Custom submission handler
  onSuccess: (result) => {},          // Success callback
  onError: (error) => {},             // Error callback
});
```

### Return Value

#### State

| Property | Type | Description |
|----------|------|-------------|
| `schema` | `FormJson \| null` | Form schema from server |
| `values` | `Record<string, unknown>` | Current form values |
| `errors` | `Record<string, string[]>` | Validation errors by field |
| `touched` | `Record<string, boolean>` | Fields that have been interacted with |
| `isValid` | `boolean` | Form validity (all visible fields valid) |
| `isLoading` | `boolean` | Initial fetch in progress |
| `isSubmitting` | `boolean` | Submission in progress |
| `isSubmitted` | `boolean` | Successfully submitted |
| `error` | `string \| null` | Error message from fetch or submission |
| `step` | `StepState \| null` | Multi-step navigation state (null if single-step) |

#### Actions

| Method | Description |
|--------|-------------|
| `setValue(field, value)` | Set a field value |
| `setValues(values)` | Set multiple values at once |
| `setTouched(field)` | Mark a field as touched |
| `validateField(field)` | Validate a single field, returns errors |
| `validateAll()` | Validate all visible fields |
| `nextStep()` | Navigate to next step (validates current step first) |
| `prevStep()` | Navigate to previous step |
| `goToStep(stepId)` | Go to specific step by ID |
| `submit()` | Submit the form |
| `reset()` | Reset form to initial state |

#### Props Helpers

| Method | Returns | Description |
|--------|---------|-------------|
| `getFormProps()` | `FormProps` | Props for `<form>` element |
| `getFieldProps(name)` | `FieldProps` | Props for `<input>` element |
| `getSelectProps(name)` | `SelectProps` | Props for `<select>` element |
| `getCheckboxProps(name)` | `CheckboxProps` | Props for checkbox `<input>` |
| `getRadioGroupProps(name)` | `RadioGroupProps` | Props for radio button group |
| `getTextareaProps(name)` | `TextareaProps` | Props for `<textarea>` element |
| `getLabelProps(name)` | `LabelProps` | Props for `<label>` element |
| `getFieldMeta(name)` | `FieldMeta` | Field metadata (value, errors, touched, etc.) |
| `getStepProps()` | `StepProps` | Props for step navigation buttons |

#### Visibility Helpers

| Property/Method | Description |
|-----------------|-------------|
| `visibleFields` | Array of visible fields for current step |
| `visibleSteps` | Array of visible steps |
| `isFieldVisible(name)` | Check if a field is visible |
| `isStepVisible(stepId)` | Check if a step is visible |

## Field Type Examples

### Text Input

```tsx
<input {...form.getFieldProps('email')} />
```

### Select

```tsx
const selectProps = form.getSelectProps('country');

<select {...selectProps}>
  <option value="">Select...</option>
  {selectProps.options.map((opt) => (
    <option key={opt.value} value={opt.value}>
      {opt.label}
    </option>
  ))}
</select>
```

### Multi-Select

```tsx
const selectProps = form.getSelectProps('interests'); // multiple={true} automatically set

<select {...selectProps}>
  {selectProps.options.map((opt) => (
    <option key={opt.value} value={opt.value}>
      {opt.label}
    </option>
  ))}
</select>
```

### Checkbox

```tsx
<input {...form.getCheckboxProps('agree_terms')} />
```

### Radio Group

```tsx
const radioProps = form.getRadioGroupProps('plan');

<fieldset>
  {radioProps.options.map((opt) => (
    <label key={opt.value}>
      <input {...radioProps.getOptionProps(opt)} />
      {opt.label}
    </label>
  ))}
</fieldset>
```

### Textarea

```tsx
<textarea {...form.getTextareaProps('message')} />
```

## Multi-Step Forms

```tsx
function WizardForm() {
  const form = useAnyForm('onboarding');

  if (!form.step) return <div>Single step form</div>;

  return (
    <form {...form.getFormProps()}>
      <div>Step {form.step.progress[0]} of {form.step.progress[1]}</div>

      {form.visibleFields.map((field) => (
        <input key={field.name} {...form.getFieldProps(field.name)} />
      ))}

      <div>
        {form.step.canGoPrev && (
          <button type="button" onClick={form.prevStep}>Back</button>
        )}
        {form.step.isLast ? (
          <button type="submit">Submit</button>
        ) : (
          <button type="button" onClick={form.nextStep}>Next</button>
        )}
      </div>
    </form>
  );
}
```

## Context Provider

For app-wide configuration:

```tsx
import { AnyFormProvider } from '@anyform/react';

function App() {
  return (
    <AnyFormProvider baseUrl="https://api.example.com" tailwind>
      <ContactForm />
      <FeedbackForm />
    </AnyFormProvider>
  );
}
```

## Render Props Component

Alternative to the hook:

```tsx
import { AnyForm } from '@anyform/react';

<AnyForm slug="contact" options={{ tailwind: true }}>
  {(form) => (
    <form {...form.getFormProps()}>
      {/* Your form UI */}
    </form>
  )}
</AnyForm>
```

## Tailwind Integration

When `tailwind: true` is set, props helpers include sensible default classes:

```tsx
form.getFieldProps('email')
// className: "block w-full rounded-md border-gray-300 shadow-sm ..."

// With errors:
// className: "block w-full rounded-md border-red-300 text-red-900 ..."
```

Customize with `classNames` option:

```tsx
useAnyForm('contact', {
  tailwind: true,
  classNames: {
    input: 'my-custom-input',
    inputError: 'my-custom-error',
    label: 'my-custom-label',
  },
});
```

## TypeScript

Full type definitions are included. Key types:

```tsx
import type {
  FormJson,
  FieldJson,
  StepJson,
  UseAnyFormOptions,
  UseAnyFormReturn,
  FieldMeta,
} from '@anyform/react';
```

## Bundler Configuration

This package uses WASM (via `anyform-js`). Most modern bundlers support WASM, but may need configuration.

### Vite

```ts
// vite.config.ts
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
  plugins: [wasm()],
  optimizeDeps: {
    exclude: ['anyform-js'],
  },
});
```

Install the plugin: `npm install -D vite-plugin-wasm`

### Next.js

```js
// next.config.js
module.exports = {
  webpack: (config) => {
    config.experiments = {
      ...config.experiments,
      asyncWebAssembly: true,
    };
    return config;
  },
};
```

Or use `@anyform/next` which handles this automatically.

### Webpack 5

```js
// webpack.config.js
module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
};
```

### Create React App (CRA)

CRA doesn't support WASM out of the box. Use `craco`:

```bash
npm install @craco/craco
```

```js
// craco.config.js
module.exports = {
  webpack: {
    configure: (config) => {
      config.experiments = { asyncWebAssembly: true };
      return config;
    },
  },
};
```

Update `package.json` scripts to use `craco` instead of `react-scripts`.

## License

MIT

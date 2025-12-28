# @wordpuppi/anyform-react

React hooks for **anyform** — headless form state, validation, and multi-step navigation.

## Installation

```bash
npm install @wordpuppi/anyform-react
```

**That's it!** No bundler configuration needed. The default JS engine works everywhere.

### Optional: Enable WASM Engine

For faster validation on large forms, install the optional WASM package:

```bash
npm install @wordpuppi/anyform-wasm-js
```

```tsx
const form = useAnyForm('contact', { engine: 'wasm' });
```

See [Bundler Configuration](#bundler-configuration) for WASM setup.

## Quick Start

```tsx
import { useAnyForm, AutoFormField } from '@wordpuppi/anyform-react';

function ContactForm() {
  const form = useAnyForm('contact', {
    baseUrl: 'http://localhost:3000',
    onSuccess: (result) => console.log('Submitted!', result),
  });

  if (form.isLoading) return <div>Loading...</div>;
  if (form.error) return <div>Error: {form.error}</div>;

  return (
    <form {...form.getFormProps()}>
      {form.visibleFields.map((field) => (
        <AutoFormField key={field.name} field={field} form={form} />
      ))}
      <button type="submit" disabled={form.isSubmitting}>
        {form.isSubmitting ? 'Submitting...' : 'Submit'}
      </button>
    </form>
  );
}
```

## Features

- **Zero Config** — Works out of the box with pure JS engine
- **Headless** — No styles, bring your own UI
- **Auto Field Rendering** — `<AutoFormField />` handles all field types
- **Multi-step forms** — Built-in step navigation
- **Conditional fields** — Show/hide based on other field values
- **Optional WASM** — Faster validation with `@wordpuppi/anyform-wasm-js`
- **Tailwind optional** — Enable with `tailwind: true`
- **TypeScript** — Full type definitions included

## AutoFormField

The `<AutoFormField />` component automatically renders the correct input type based on your field's `field_type`. No more switch statements!

```tsx
import { useAnyForm, AutoFormField } from '@wordpuppi/anyform-react';

function MyForm() {
  const form = useAnyForm('my-form', { baseUrl: '...' });

  return (
    <form {...form.getFormProps()}>
      {form.visibleFields.map((field) => (
        <AutoFormField
          key={field.name}
          field={field}
          form={form}
          className="mb-4"
          errorClassName="text-red-500 text-sm"
        />
      ))}
      <button type="submit">Submit</button>
    </form>
  );
}
```

### Props

| Prop | Type | Description |
|------|------|-------------|
| `field` | `FieldJson` | Field definition from `form.visibleFields` |
| `form` | `UseAnyFormReturn` | Form state from `useAnyForm` |
| `className` | `string` | Class for wrapper div |
| `errorClassName` | `string` | Class for error messages (default: `text-red-500 text-sm`) |
| `renderField` | `(field, form) => ReactNode` | Custom render override |

### Custom Field Rendering

Override specific fields while using `AutoFormField` for the rest:

```tsx
{form.visibleFields.map((field) => (
  <AutoFormField
    key={field.name}
    field={field}
    form={form}
    renderField={
      field.name === 'custom_field'
        ? (f, form) => <MyCustomInput field={f} form={form} />
        : undefined
    }
  />
))}
```

## API Reference

### `useAnyForm(slug, options?)`

Main hook for form state management.

```tsx
const form = useAnyForm('my-form', {
  baseUrl: 'http://localhost:3000',  // API base URL
  engine: 'js',                       // 'js' (default) or 'wasm'
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

## Manual Field Rendering

If you prefer full control over field rendering, use the props helpers directly:

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
        <AutoFormField key={field.name} field={field} form={form} />
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
import { AnyFormProvider } from '@wordpuppi/anyform-react';

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
import { AnyForm } from '@wordpuppi/anyform-react';

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
  AutoFormFieldProps,
} from '@wordpuppi/anyform-react';
```

## Troubleshooting

### Form not loading / infinite loading

1. Check that `baseUrl` is correct
2. Verify the API is running: `curl {baseUrl}/api/forms/{slug}/json`
3. Check browser console for CORS errors

### "Failed to load WASM module" (only with `engine: 'wasm'`)

This only affects WASM users. The default JS engine requires no config.

**Solutions:**
1. Vite: Install `vite-plugin-wasm` (see Bundler Configuration)
2. Next.js: Add `asyncWebAssembly: true` to webpack config
3. CRA: Use craco (see Bundler Configuration)

Or just use the default JS engine (no config needed).

### Validation errors not showing

1. Ensure `form.isLoading` is `false` before interacting
2. Check that field is touched: `form.touched[fieldName]`
3. Validation runs on blur/change by default

## Bundler Configuration (WASM Only)

Only needed if using `engine: 'wasm'`. The default JS engine works without any configuration.

### Vite

```ts
// vite.config.ts
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
  plugins: [wasm()],
  optimizeDeps: {
    exclude: ['@wordpuppi/anyform-wasm-js'],
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

Or use `@wordpuppi/anyform-next` which handles this automatically.

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

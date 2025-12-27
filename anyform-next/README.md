# @anyform/next

Next.js integration for **anyform** — React Server Components and Server Actions support.

## Installation

```bash
npm install @anyform/next
```

## Quick Start (App Router)

```tsx
// app/contact/page.tsx
import { AnyFormRSC } from '@anyform/next';

export default function ContactPage() {
  return (
    <AnyFormRSC slug="contact" options={{ tailwind: true }}>
      {(form) => (
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
      )}
    </AnyFormRSC>
  );
}
```

## Features

- **React Server Components** — Form schema fetched on the server
- **Server Actions** — Optional server-side form submission
- **ISR Support** — Schema cached with revalidation
- **Re-exports @anyform/react** — All hooks and types included

## API Reference

### `<AnyFormRSC>` Component

React Server Component that fetches form schema on the server and hydrates on the client.

```tsx
import { AnyFormRSC } from '@anyform/next';

<AnyFormRSC
  slug="contact"                    // Form slug (required)
  baseUrl="http://localhost:3000"   // API base URL (optional, uses ANYFORM_API_URL env)
  options={{ tailwind: true }}      // Options passed to useAnyForm
>
  {(form) => (
    <form {...form.getFormProps()}>
      {/* Your form UI */}
    </form>
  )}
</AnyFormRSC>
```

### Server Functions

Import from `@anyform/next/server`:

```tsx
import { fetchFormSchema, submitForm } from '@anyform/next/server';
```

#### `fetchFormSchema(slug, baseUrl?)`

Cached server-side form schema fetcher. Uses React's `cache()` for deduplication.

```tsx
// app/forms/[slug]/page.tsx
import { fetchFormSchema } from '@anyform/next/server';
import { ClientForm } from './client-form';

export default async function FormPage({ params }: { params: { slug: string } }) {
  const schema = await fetchFormSchema(params.slug);

  return <ClientForm initialSchema={schema} />;
}
```

#### `submitForm(slug, data, options?)`

Server Action for form submission.

```tsx
// app/contact/actions.ts
'use server';

import { submitForm } from '@anyform/next/server';

export async function handleSubmit(values: Record<string, unknown>) {
  const result = await submitForm('contact', values);

  if (result.success) {
    // Handle success
    return { success: true, data: result.data };
  } else {
    // Handle error
    return { success: false, error: result.error };
  }
}
```

```tsx
// app/contact/form.tsx
'use client';

import { useAnyForm } from '@anyform/next';
import { handleSubmit } from './actions';

function ContactForm({ initialSchema }) {
  const form = useAnyForm('contact', {
    initialSchema,
    onSubmit: handleSubmit,
  });

  // ...
}
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `ANYFORM_API_URL` | Default API base URL for server-side fetching |

```bash
# .env.local
ANYFORM_API_URL=http://localhost:3000
```

## Usage Patterns

### Pattern 1: Full RSC (Recommended)

Best for SEO and initial load performance.

```tsx
// app/contact/page.tsx
import { AnyFormRSC } from '@anyform/next';

export default function ContactPage() {
  return (
    <AnyFormRSC slug="contact" options={{ tailwind: true }}>
      {(form) => <YourFormUI form={form} />}
    </AnyFormRSC>
  );
}
```

### Pattern 2: Manual Schema Fetch

For more control over the data fetching.

```tsx
// app/contact/page.tsx
import { fetchFormSchema } from '@anyform/next/server';
import { ContactForm } from './form';

export default async function ContactPage() {
  const schema = await fetchFormSchema('contact');

  return <ContactForm schema={schema} />;
}

// app/contact/form.tsx
'use client';

import { useAnyForm } from '@anyform/next';

export function ContactForm({ schema }) {
  const form = useAnyForm('contact', {
    initialSchema: schema,
    tailwind: true,
  });

  return <form {...form.getFormProps()}>...</form>;
}
```

### Pattern 3: Client-Only

For dynamic forms or when RSC isn't needed.

```tsx
'use client';

import { useAnyForm } from '@anyform/next';

export function ContactForm() {
  const form = useAnyForm('contact', { tailwind: true });

  if (form.isLoading) return <div>Loading...</div>;

  return <form {...form.getFormProps()}>...</form>;
}
```

### Pattern 4: With Server Actions

For server-side form submission.

```tsx
// app/contact/page.tsx
import { AnyFormRSC } from '@anyform/next';
import { submitForm } from '@anyform/next/server';

async function handleSubmit(values: Record<string, unknown>) {
  'use server';
  return submitForm('contact', values);
}

export default function ContactPage() {
  return (
    <AnyFormRSC
      slug="contact"
      options={{
        tailwind: true,
        onSubmit: handleSubmit,
      }}
    >
      {(form) => <YourFormUI form={form} />}
    </AnyFormRSC>
  );
}
```

## Multi-Step Forms

Works the same as `@anyform/react`:

```tsx
<AnyFormRSC slug="onboarding">
  {(form) => (
    <form {...form.getFormProps()}>
      {form.step && (
        <div>Step {form.step.progress[0]} of {form.step.progress[1]}</div>
      )}

      {form.visibleFields.map((field) => (
        <input key={field.name} {...form.getFieldProps(field.name)} />
      ))}

      <div>
        {form.step?.canGoPrev && (
          <button type="button" onClick={form.prevStep}>Back</button>
        )}
        {form.step?.isLast ? (
          <button type="submit">Submit</button>
        ) : (
          <button type="button" onClick={form.nextStep}>Next</button>
        )}
      </div>
    </form>
  )}
</AnyFormRSC>
```

## Re-exports

This package re-exports everything from `@anyform/react`:

```tsx
import {
  // Hooks
  useAnyForm,

  // Components
  AnyForm,
  AnyFormProvider,

  // Next.js specific
  AnyFormRSC,
  AnyFormClient,

  // Types
  type FormJson,
  type FieldJson,
  type UseAnyFormReturn,
  // ... etc
} from '@anyform/next';
```

## TypeScript

Full type definitions included:

```tsx
import type {
  AnyFormRSCProps,
  AnyFormClientProps,
} from '@anyform/next';

import type {
  SubmitFormResult,
} from '@anyform/next/server';
```

## License

MIT

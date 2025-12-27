# anyform Examples

Example implementations for `@anyform/react` and `@anyform/next`.

## Examples

### React Basic

Simple React examples using `@anyform/react`:

- **[ContactForm.tsx](./react-basic/ContactForm.tsx)** - Basic contact form with all field types
- **[MultiStepForm.tsx](./react-basic/MultiStepForm.tsx)** - Multi-step wizard with progress indicator

```bash
# Usage (in your React project)
npm install @anyform/react
```

### Next.js App Router

Next.js 14+ examples using `@anyform/next`:

- **[app/contact/page.tsx](./nextjs-app-router/app/contact/page.tsx)** - RSC-powered contact form

```bash
# Usage (in your Next.js project)
npm install @anyform/next
```

## Running Examples

These are code snippets, not runnable projects. Copy them into your own project.

### Prerequisites

1. An anyform server running at `http://localhost:3000`
2. A form created with slug `contact` or `onboarding`

### Quick Setup

```tsx
// In your React app
import { ContactForm } from './ContactForm';

function App() {
  return <ContactForm />;
}
```

```tsx
// In your Next.js app
// Just create the file at app/contact/page.tsx
```

## Form Schema Example

Your anyform server should return a schema like:

```json
{
  "id": "...",
  "name": "Contact Form",
  "slug": "contact",
  "steps": [
    {
      "id": "...",
      "name": "Contact Info",
      "fields": [
        {
          "name": "name",
          "label": "Your Name",
          "field_type": "text",
          "required": true
        },
        {
          "name": "email",
          "label": "Email",
          "field_type": "email",
          "required": true
        },
        {
          "name": "message",
          "label": "Message",
          "field_type": "textarea",
          "required": true
        }
      ]
    }
  ]
}
```

## License

MIT

# Anyform WordPress Plugin

Dynamic forms powered by JSON schemas and WebAssembly.

## Features

- **JSON-Based Forms** - Define forms using a declarative JSON schema
- **Multi-Step Forms** - Wizard-style forms with Back/Next navigation
- **Conditional Logic** - Show/hide fields and steps based on user input
- **Client-Side Validation** - Real-time validation via [@anyform/wasm-js](https://github.com/wordpuppi/anyform) WASM
- **Email Notifications** - Admin alerts and auto-replies on submission
- **Webhook Support** - Forward submissions to external services (Zapier, Make, etc.)
- **Submission Storage** - All submissions stored in `wp_af_submissions` table

## Why Anyform?

Anyform offers features that typically require paid form plugins—completely free.

### vs WPForms Lite (Free)

| Feature | Anyform | WPForms Lite |
|---------|:-------:|:------------:|
| Multi-Step Forms | ✅ | ❌ |
| Conditional Logic | ✅ | ❌ |
| Database Storage | ✅ | ❌ |
| Auto-Reply Emails | ✅ | ❌ |
| Multiple Recipients | ✅ | ❌ |
| Date/Time Fields | ✅ | ❌ |
| REST API | ✅ | ❌ |
| SendGrid/Mailgun | ✅ | ❌ |

### vs WPForms Pro ($199/year)

Anyform includes these Pro-level features for free:
- Multi-step forms with conditional logic
- Email template variables (`{name}`, `{date}`, etc.)
- Direct email API integration (SendGrid, Mailgun)
- Webhook forwarding to Zapier/Make

Anyform also offers features WPForms Pro doesn't have:
- **REST API** for headless WordPress
- **JSON schema** for portable form definitions
- **WebAssembly** client for fast validation
- **CSS variables** for easy theming

### Best For

- **Developers** who want JSON-based control
- **Headless WordPress** projects needing REST API
- **Budget-conscious** users who need Pro features free
- **Multi-step forms** with conditional logic

### Not For

- Users who need drag-and-drop builders
- File uploads or payment processing
- Pre-built templates

## Requirements

- WordPress 6.0+
- PHP 8.0+

## Browser Compatibility

Anyform uses WebAssembly for client-side validation. Supported browsers:

| Browser | Minimum Version |
|---------|-----------------|
| Chrome | 57+ |
| Firefox | 52+ |
| Safari | 11+ |
| Edge | 16+ |
| iOS Safari | 11+ |
| Android Chrome | 57+ |

**Note:** Forms will render without JavaScript, but client-side validation and multi-step navigation require WASM support. Server-side validation always runs regardless of browser capabilities.

## Installation

1. Copy `anyform-wordpress` to `wp-content/plugins/anyform`
2. Activate in WordPress admin
3. Go to **Anyform > Add New Form**

## Usage

### Creating a Form

1. Go to **Anyform > Add New Form**
2. Enter a title (used as the slug)
3. Paste your JSON schema
4. Publish

### Embedding

Use the shortcode on any page or post:

```
[anyform slug="contact"]
```

Optional attributes:
```
[anyform slug="contact" class="my-form" id="contact-form"]
```

### JSON Schema Format

```json
{
  "steps": [
    {
      "name": "Contact Information",
      "description": "Please provide your details",
      "fields": [
        {
          "name": "name",
          "label": "Full Name",
          "field_type": "text",
          "placeholder": "John Doe",
          "validation": {
            "required": true,
            "min_length": 2
          }
        },
        {
          "name": "email",
          "label": "Email Address",
          "field_type": "email",
          "validation": {
            "required": true
          }
        },
        {
          "name": "phone",
          "label": "Phone",
          "field_type": "tel"
        }
      ]
    },
    {
      "name": "Your Message",
      "fields": [
        {
          "name": "subject",
          "label": "Subject",
          "field_type": "select",
          "options": [
            {"value": "general", "label": "General Inquiry"},
            {"value": "support", "label": "Support"},
            {"value": "sales", "label": "Sales"}
          ]
        },
        {
          "name": "message",
          "label": "Message",
          "field_type": "textarea",
          "validation": {
            "required": true
          },
          "ui_options": {
            "rows": 6
          }
        }
      ]
    }
  ],
  "settings": {
    "submit_label": "Send Message"
  }
}
```

### Field Types

| Type | Description |
|------|-------------|
| `text` | Single-line text input |
| `email` | Email input with validation |
| `tel` | Phone number input |
| `url` | URL input with validation |
| `number` | Numeric input |
| `textarea` | Multi-line text |
| `select` | Dropdown menu |
| `radio` | Radio button group |
| `checkbox` | Single checkbox |
| `date` | Date picker |
| `time` | Time picker |
| `password` | Password input |
| `hidden` | Hidden field |

### Validation Rules

```json
{
  "validation": {
    "required": true,
    "min_length": 2,
    "max_length": 100,
    "min": 0,
    "max": 100,
    "pattern": "^[A-Z].*"
  }
}
```

### Conditional Logic

Show a field only when another field has a specific value:

```json
{
  "name": "company_name",
  "label": "Company Name",
  "field_type": "text",
  "condition": {
    "field": "customer_type",
    "operator": "eq",
    "value": "business"
  }
}
```

Supported operators: `eq`, `neq`, `gt`, `lt`, `gte`, `lte`, `contains`, `not_contains`

Conditional steps work the same way - the entire step is shown/hidden based on the condition.

## Customization

Forms are styled using CSS custom properties (variables), making it easy to customize colors, spacing, and borders without writing complex selectors.

### Quick Theme Override

Add to your theme's `style.css` or WordPress Customizer > Additional CSS:

```css
:root {
  --af-color-primary: #8b5cf6;      /* Purple buttons */
  --af-color-primary-hover: #7c3aed;
  --af-border-radius: 8px;          /* More rounded */
  --af-form-max-width: 500px;       /* Narrower form */
}
```

### Available CSS Variables

**Colors:**
| Variable | Default | Description |
|----------|---------|-------------|
| `--af-color-primary` | `#0073aa` | Buttons, focus rings |
| `--af-color-primary-hover` | `#005a87` | Button hover state |
| `--af-color-error` | `#dc2626` | Validation errors, required asterisk |
| `--af-color-success` | `#10b981` | Success message border |
| `--af-color-success-bg` | `#ecfdf5` | Success message background |
| `--af-color-success-text` | `#065f46` | Success message text |
| `--af-color-text-muted` | `#666` | Help text, descriptions |
| `--af-color-border` | `#ccc` | Input borders |
| `--af-color-bg-secondary` | `#e5e7eb` | Back button background |

**Spacing:**
| Variable | Default | Description |
|----------|---------|-------------|
| `--af-spacing-field` | `1rem` | Margin between fields |
| `--af-spacing-input` | `0.5rem` | Input padding |
| `--af-spacing-button` | `0.75rem 1.5rem` | Button padding |
| `--af-spacing-nav` | `1.5rem` | Space above navigation |

**Typography:**
| Variable | Default | Description |
|----------|---------|-------------|
| `--af-font-size-base` | `1rem` | Inputs, buttons |
| `--af-font-size-small` | `0.875rem` | Help text, errors |
| `--af-font-size-step-title` | `1.25rem` | Step headings |

**Borders & Layout:**
| Variable | Default | Description |
|----------|---------|-------------|
| `--af-border-radius` | `4px` | Inputs, buttons |
| `--af-border-radius-large` | `8px` | Success message |
| `--af-border-width` | `1px` | Input borders |
| `--af-form-max-width` | `600px` | Form container width |
| `--af-textarea-min-height` | `100px` | Minimum textarea height |

### Example: Dark Theme

```css
:root {
  --af-color-primary: #60a5fa;
  --af-color-primary-hover: #3b82f6;
  --af-color-text-muted: #9ca3af;
  --af-color-border: #4b5563;
  --af-color-bg-secondary: #374151;
  --af-color-bg-secondary-hover: #4b5563;
  --af-color-text-secondary: #e5e7eb;
}

.af-form {
  background: #1f2937;
  padding: 2rem;
  border-radius: 8px;
}

.af-form input,
.af-form textarea,
.af-form select {
  background: #374151;
  color: #f9fafb;
}
```

## Email Configuration

Go to **Anyform > Settings** to configure email notifications.

### Email Methods

#### WordPress wp_mail() (Default)

Uses WordPress's built-in mail function. For reliable delivery, install an SMTP plugin:
- [WP Mail SMTP](https://wordpress.org/plugins/wp-mail-smtp/)
- [Post SMTP](https://wordpress.org/plugins/post-smtp-mailer/)
- [FluentSMTP](https://wordpress.org/plugins/fluent-smtp/)

#### SendGrid API

1. Select "External API" as email method
2. Choose "SendGrid" as provider
3. Enter your [SendGrid API Key](https://app.sendgrid.com/settings/api_keys)
4. Verify your sender email in SendGrid dashboard

#### Mailgun API

1. Select "External API" as email method
2. Choose "Mailgun" as provider
3. Enter your [Mailgun API Key](https://app.mailgun.com/settings/api_security)
4. Enter your Mailgun domain in the "API Endpoint" field (e.g., `mg.yourdomain.com`)
   - If left blank, uses your WordPress site domain

#### Custom API

For other email services or custom endpoints:

1. Select "External API" as email method
2. Choose "Custom" as provider
3. Enter your API key (sent as `Authorization: Bearer {key}`)
4. Enter the full endpoint URL

**Expected payload format:**
```json
{
  "to": "recipient@example.com",
  "from": {
    "email": "noreply@yoursite.com",
    "name": "Your Site"
  },
  "subject": "Email subject",
  "html": "<p>Email body HTML</p>"
}
```

### Admin Notification

Sends an email to site admins when a form is submitted.

| Setting | Description |
|---------|-------------|
| **Enable** | Toggle admin notifications on/off |
| **To** | Recipient email(s), comma-separated for multiple |
| **Subject** | Email subject (supports template variables) |
| **From Name** | Sender name shown in email client |
| **From Email** | Sender email address (must be verified with your email provider) |

### Auto-Reply

Sends a confirmation email to the form submitter.

| Setting | Description |
|---------|-------------|
| **Enable** | Toggle auto-reply on/off |
| **Email Field** | Which form field contains the submitter's email (default: `email`) |
| **Subject** | Email subject (supports template variables) |
| **Body** | Email body text (supports template variables) |

### Template Variables

Use these placeholders in subject and body templates:

| Variable | Description | Example |
|----------|-------------|---------|
| `{form_name}` | Form title | "Contact Form" |
| `{form_slug}` | Form URL slug | "contact" |
| `{site_name}` | WordPress site name | "My Website" |
| `{site_url}` | WordPress site URL | "https://example.com" |
| `{date}` | Submission date | "December 27, 2025" |
| `{time}` | Submission time | "2:30 PM" |
| `{field_name}` | Any submitted field value | `{email}`, `{name}`, `{message}` |

**Example subject:**
```
New {form_name} submission from {name}
```

**Example auto-reply body:**
```
Hi {name},

Thank you for contacting {site_name}! We received your message on {date} and will respond soon.

Your submission:
- Email: {email}
- Subject: {subject}
- Message: {message}

Best regards,
The {site_name} Team
```

### Email Format

Admin notification emails are sent as HTML with:
- Form name as heading
- Submission date and time
- Table of all submitted fields and values
- Site name footer with link

Fields starting with underscore (e.g., `_nonce`) are automatically excluded.

## Webhook Integration

To forward submissions to an external service:

1. Edit your form
2. Enter the webhook URL in **Action URL**
3. Save

Submissions are POSTed as `application/x-www-form-urlencoded`.

## REST API

### Submit Form

```
POST /wp-json/anyform/v1/forms/{slug}
Content-Type: application/x-www-form-urlencoded

name=John&email=john@example.com&message=Hello
```

Requires valid `af_nonce` for CSRF protection.

## Database

Submissions are stored in `{prefix}_af_submissions`:

| Column | Type | Description |
|--------|------|-------------|
| id | bigint | Primary key |
| form_id | bigint | Form post ID |
| form_slug | varchar(255) | Form slug |
| data | longtext | JSON submission data |
| ip_address | varchar(45) | Submitter IP |
| user_agent | text | Browser user agent |
| created_at | datetime | Submission timestamp |

## File Structure

```
anyform/
├── anyform.php              # Main plugin file
├── uninstall.php            # Cleanup on plugin deletion
├── readme.txt               # WordPress.org readme
├── README.md                # This file
├── assets/
│   └── css/
│       ├── anyform.css      # Frontend styles
│       └── anyform-admin.css # Admin styles
└── includes/
    ├── class-af-database.php    # Database operations
    ├── class-af-email.php       # Email sending
    ├── class-af-post-type.php   # Form CPT
    ├── class-af-rest-api.php    # REST endpoints
    ├── class-af-settings.php    # Settings page
    ├── class-af-shortcode.php   # Shortcode handler
    └── class-af-submissions.php # Submissions admin page
```

## Development

The frontend uses [@anyform/wasm-js](https://github.com/wordpuppi/anyform), a WebAssembly library that hydrates the server-rendered HTML with:
- Client-side validation
- Conditional logic evaluation
- Multi-step navigation

The WASM client is loaded from jsDelivr CDN.

## License

MIT

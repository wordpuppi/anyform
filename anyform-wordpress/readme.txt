=== Anyform ===
Contributors: anyformteam
Tags: forms, contact form, multi-step form, json forms, dynamic forms
Requires at least: 6.0
Tested up to: 6.9
Requires PHP: 8.0
Stable tag: 1.0.0
License: MIT
License URI: https://opensource.org/licenses/MIT

Dynamic forms powered by JSON schemas and WebAssembly. Create multi-step forms with conditional logic, validation, and email notifications.

== Description ==

Anyform is a modern form plugin that renders forms from JSON schemas. Define your forms once in JSON and deploy them anywhere - WordPress, static sites, or custom applications.

**Key Features:**

* **JSON-Based Forms** - Define forms using a simple JSON schema
* **Multi-Step Forms** - Create wizard-style forms with progress tracking
* **Conditional Logic** - Show/hide fields and steps based on user input
* **Client-Side Validation** - Real-time validation powered by WebAssembly
* **Email Notifications** - Send admin alerts and auto-replies on submission
* **Webhook Support** - Forward submissions to external services
* **Submission Storage** - All submissions saved to database for review
* **Shortcode Embedding** - Simple `[anyform]` shortcode for any page
* **Easy Theming** - CSS variables for customizing colors, spacing, and borders

**Supported Field Types:**

* Text, Email, URL, Tel, Number
* Textarea
* Select dropdown
* Radio buttons
* Checkbox
* Date and Time pickers
* Password and Hidden fields

**Email Options:**

* WordPress wp_mail() - works with any SMTP plugin (WP Mail SMTP, etc.)
* SendGrid API - direct integration with API key
* Mailgun API - direct integration with API key
* Custom API - use any email service with Bearer token auth
* Template variables for personalized emails: `{name}`, `{email}`, `{form_name}`, `{date}`, etc.

== Installation ==

1. Upload the `anyform` folder to `/wp-content/plugins/`
2. Activate the plugin through the 'Plugins' menu
3. Go to **Anyform > Add New Form** to create your first form
4. Paste your JSON schema and publish
5. Use the shortcode `[anyform slug="your-form"]` to embed

== Frequently Asked Questions ==

= How do I create a form? =

Create a JSON schema with your form structure:

`{
  "steps": [{
    "name": "Contact Info",
    "fields": [
      {"name": "name", "label": "Your Name", "field_type": "text", "validation": {"required": true}},
      {"name": "email", "label": "Email", "field_type": "email", "validation": {"required": true}},
      {"name": "message", "label": "Message", "field_type": "textarea"}
    ]
  }],
  "settings": {
    "submit_label": "Send Message"
  }
}`

= How do I enable email notifications? =

Go to **Anyform > Settings** and configure:

1. Enable "Send email to admin on new submissions"
2. Enter recipient email address(es) - comma-separated for multiple
3. Set "From Name" and "From Email" for the sender
4. Optionally enable auto-reply for form submitters

For reliable delivery with WordPress wp_mail(), install WP Mail SMTP or similar plugin.

= Can I use SendGrid, Mailgun, or other email APIs? =

Yes! Under **Anyform > Settings**:

1. Select "External API" as email method
2. Choose your provider (SendGrid, Mailgun, or Custom)
3. Enter your API key
4. For Mailgun, enter your domain in the "API Endpoint" field

= What template variables can I use in emails? =

Use these placeholders in subject and body templates:

* `{form_name}` - Form title
* `{form_slug}` - Form URL slug
* `{site_name}` - Your WordPress site name
* `{site_url}` - Your site URL
* `{date}` - Submission date
* `{time}` - Submission time
* `{field_name}` - Any form field (e.g., `{email}`, `{name}`, `{message}`)

Example subject: "New {form_name} submission from {name}"

= How does auto-reply work? =

Auto-reply sends a confirmation email to the form submitter. Configure:

1. Enable auto-reply in settings
2. Set "Email Field" to the form field containing the submitter's email (default: `email`)
3. Write your subject and body using template variables

= Can I forward submissions to external services? =

Yes! Edit your form and enter a webhook URL in the "Action URL" field. Submissions will be POSTed to that URL in addition to being saved locally.

= How does conditional logic work? =

Add a `condition` property to any field or step:

`{
  "name": "company",
  "label": "Company Name",
  "field_type": "text",
  "condition": {
    "field": "contact_type",
    "operator": "eq",
    "value": "business"
  }
}`

= How do I customize form styling? =

Forms use CSS custom properties (variables) for easy theming. Add to your theme's CSS:

`
:root {
  --af-color-primary: #8b5cf6;  /* Custom button color */
  --af-border-radius: 8px;      /* More rounded corners */
  --af-form-max-width: 500px;   /* Narrower form */
}
`

See README.md for a full list of available CSS variables.

== Screenshots ==

1. Form editor with JSON schema input
2. Frontend form rendering
3. Email notification settings
4. Submissions list in admin

== Changelog ==

= 1.0.0 =
* Initial release
* JSON-based form rendering
* Multi-step form support
* Conditional logic for fields and steps
* Email notifications (admin + auto-reply)
* SendGrid, Mailgun, and custom API support
* Webhook forwarding
* Submission storage and management
* CSS custom properties for easy theming

== External Services ==

This plugin connects to the following third-party services when configured:

**Email APIs (Optional)**

If you configure external email delivery in Anyform > Settings, the plugin will send data to your chosen provider:

* SendGrid: https://sendgrid.com/policies/privacy/
* Mailgun: https://www.mailgun.com/legal/privacy-policy/
* Custom API: Your configured endpoint

**Webhooks (Optional)**

If you configure an Action URL for a form, submission data will be sent to that external URL.

== Upgrade Notice ==

= 1.0.0 =
Initial release of Anyform.

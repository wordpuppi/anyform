# @wordpuppi/anyform-wasm-js

Browser client for [anyform](https://github.com/wordpuppi/anyform) - form state management, validation, and multi-step navigation powered by WebAssembly.

## Installation

```bash
npm install anyform-js
```

## Quick Start

```typescript
import init, { FormClient } from 'anyform-js';

async function main() {
  // Initialize WASM
  await init();

  // Create client
  const client = new FormClient('http://localhost:3000');

  // Fetch form
  const form = await client.fetch_form('contact');

  // Set values
  form.set_value('email', 'user@example.com');
  form.set_value('name', 'John Doe');

  // Validate and submit
  if (form.is_valid()) {
    const result = await form.submit();
    console.log('Submitted:', result);
  }
}
```

## Multi-Step Forms

```typescript
const form = await client.fetch_form('wizard');

// Get visible steps (respects conditions)
const steps = form.visible_steps();

// Navigate
form.go_to_step(steps[0].id);
form.set_value('country', 'US');

if (form.validate_step(form.current_step().id).length === 0) {
  form.next_step(); // Skips hidden steps automatically
}

// Progress indicator
const [current, total] = form.progress();
console.log(`Step ${current} of ${total}`);
```

## Hydration Mode

For server-rendered forms, use automatic hydration:

```html
<script type="module">
  import init, { hydrate_all } from 'anyform-js';

  await init();
  hydrate_all(); // Hydrates all forms with data-af-form attribute
</script>
```

## API

### FormClient

- `new FormClient(base_url)` - Create client instance
- `fetch_form(slug)` - Fetch form schema and create FormState
- `submit_form(slug, data)` - Submit form data directly

### FormState

**Value Management:**
- `set_value(field, value)` - Set field value
- `get_value(field)` - Get field value
- `get_values()` - Get all values

**Validation:**
- `validate_field(field)` - Validate single field
- `validate_step(step_id)` - Validate all fields in step
- `validate_all()` - Validate entire form
- `is_valid()` - Check if form is valid
- `get_errors(field)` - Get errors for field

**Visibility:**
- `visible_steps()` - Get visible steps
- `visible_fields(step_id)` - Get visible fields
- `is_step_visible(step_id)` - Check step visibility
- `is_field_visible(field_name)` - Check field visibility

**Navigation:**
- `current_step()` - Get current step
- `next_step()` - Go to next visible step
- `prev_step()` - Go to previous visible step
- `go_to_step(step_id)` - Go to specific step
- `can_go_next()` / `can_go_prev()` - Check navigation
- `progress()` - Get [current, total] step numbers

### Hydration

- `hydrate_all()` - Hydrate all forms on page
- `hydrate(slug)` - Hydrate specific form

## License

MIT

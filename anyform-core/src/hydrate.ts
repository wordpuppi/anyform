/**
 * Browser hydration for anyform forms.
 *
 * This module provides DOM-based form hydration for server-rendered HTML,
 * similar to the WASM hydrate.rs but implemented in pure JavaScript.
 *
 * Used by WordPress and other server-rendered integrations.
 */

import { FormState } from './state';
import type {
  FormJson,
  StepJson,
  FieldJson,
  ValidationRules,
  ConditionRule,
  FieldOptionJson,
} from './types';

/**
 * Hydrates all forms on the page.
 */
export function hydrateAll(): void {
  const forms = document.querySelectorAll<HTMLFormElement>('[data-af-form]');
  forms.forEach((form) => {
    const slug = form.dataset.afForm;
    if (slug) {
      hydrateForm(form, slug);
    }
  });
}

/**
 * Hydrates a specific form by slug.
 */
export function hydrate(slug: string): void {
  const form = document.querySelector<HTMLFormElement>(
    `[data-af-form="${slug}"]`
  );
  if (form) {
    hydrateForm(form, slug);
  }
}

/**
 * Hydrates a form element.
 */
function hydrateForm(form: HTMLFormElement, slug: string): void {
  // Parse schema from data attributes
  const schema = parseFormSchema(form, slug);
  const state = new FormState(schema);

  // Bind events
  bindInputEvents(form, state);
  bindNavigationEvents(form, state);
  bindSubmitEvent(form, state);

  // Initial visibility update
  updateVisibility(form, state);

  console.log(`Anyform hydrated: ${slug}`);
}

/**
 * Parses form schema from DOM data attributes.
 */
function parseFormSchema(form: HTMLFormElement, slug: string): FormJson {
  const steps: StepJson[] = [];
  const stepElements = form.querySelectorAll('[data-af-step]');

  stepElements.forEach((stepEl, stepIndex) => {
    const stepId = stepEl.getAttribute('data-af-step-id') || `step-${stepIndex}`;
    const stepName =
      stepEl.querySelector('.af-step-title')?.textContent || `Step ${stepIndex + 1}`;
    const stepCondition = parseCondition(
      stepEl.getAttribute('data-af-condition')
    );

    const fields: FieldJson[] = [];
    const fieldElements = stepEl.querySelectorAll('[data-af-field]');

    fieldElements.forEach((fieldEl, fieldIndex) => {
      const fieldName = fieldEl.getAttribute('data-af-field') || '';
      const field = parseField(fieldEl as HTMLElement, fieldName, fieldIndex);
      if (field) {
        fields.push(field);
      }
    });

    steps.push({
      id: stepId,
      name: stepName,
      order: stepIndex,
      condition: stepCondition,
      fields,
    });
  });

  // If no steps found, treat entire form as single step
  if (steps.length === 0) {
    const fields: FieldJson[] = [];
    const fieldElements = form.querySelectorAll('[data-af-field]');

    fieldElements.forEach((fieldEl, fieldIndex) => {
      const fieldName = fieldEl.getAttribute('data-af-field') || '';
      const field = parseField(fieldEl as HTMLElement, fieldName, fieldIndex);
      if (field) {
        fields.push(field);
      }
    });

    steps.push({
      id: 'step-0',
      name: 'Form',
      order: 0,
      fields,
    });
  }

  return {
    id: form.getAttribute('data-af-form-id') || slug,
    name: form.getAttribute('data-af-form-name') || slug,
    slug,
    action_url: form.action || undefined,
    action_method: form.method?.toUpperCase() || 'POST',
    settings: {},
    steps,
  };
}

/**
 * Parses a field from DOM element.
 */
function parseField(
  fieldEl: HTMLElement,
  name: string,
  order: number
): FieldJson | null {
  const label =
    fieldEl.querySelector('label')?.textContent?.replace(/\*$/, '').trim() ||
    name;
  const input = fieldEl.querySelector<
    HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement
  >('input, select, textarea');
  if (!input) return null;

  const validationAttr = fieldEl.getAttribute('data-af-validation');
  const conditionAttr = fieldEl.getAttribute('data-af-condition');

  const validation: ValidationRules = validationAttr
    ? JSON.parse(validationAttr)
    : {};
  const condition = parseCondition(conditionAttr);

  // Determine field type
  let fieldType = 'text';
  if (input.tagName === 'SELECT') {
    fieldType = (input as HTMLSelectElement).multiple ? 'multi_select' : 'select';
  } else if (input.tagName === 'TEXTAREA') {
    fieldType = 'textarea';
  } else if (input instanceof HTMLInputElement) {
    const type = input.type.toLowerCase();
    if (type === 'checkbox') {
      fieldType = 'checkbox';
    } else if (type === 'radio') {
      fieldType = 'radio';
    } else {
      fieldType = type;
    }
  }

  // Parse options for select/radio
  const options: FieldOptionJson[] = [];
  if (input.tagName === 'SELECT') {
    const selectEl = input as HTMLSelectElement;
    Array.from(selectEl.options).forEach((opt, i) => {
      if (opt.value) {
        options.push({
          id: `${name}-opt-${i}`,
          label: opt.text,
          value: opt.value,
          order: i,
        });
      }
    });
  } else if (fieldType === 'radio') {
    const radios = fieldEl.querySelectorAll<HTMLInputElement>(
      `input[name="${name}"]`
    );
    radios.forEach((radio, i) => {
      const radioLabel =
        radio.parentElement?.textContent?.trim() || radio.value;
      options.push({
        id: `${name}-opt-${i}`,
        label: radioLabel,
        value: radio.value,
        order: i,
      });
    });
  }

  return {
    id: input.id || `field-${name}`,
    name,
    label,
    field_type: fieldType as FieldJson['field_type'],
    placeholder: 'placeholder' in input ? input.placeholder || undefined : undefined,
    validation,
    condition,
    options,
    order,
    required: validation.required || input.required,
  };
}

/**
 * Parses a condition from JSON string.
 */
function parseCondition(attr: string | null): ConditionRule | undefined {
  if (!attr) return undefined;
  try {
    return JSON.parse(attr) as ConditionRule;
  } catch {
    return undefined;
  }
}

/**
 * Binds input events for form fields.
 */
function bindInputEvents(form: HTMLFormElement, state: FormState): void {
  const inputs = form.querySelectorAll<
    HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement
  >('input, select, textarea');

  inputs.forEach((input) => {
    const fieldName = input.name;
    if (!fieldName) return;

    // Input/change event - update value
    const handleInput = () => {
      let value: unknown;

      if (input.type === 'checkbox') {
        value = (input as HTMLInputElement).checked;
      } else if (input.type === 'number' || input.type === 'range') {
        value = input.value === '' ? '' : parseFloat(input.value);
      } else if (input instanceof HTMLSelectElement && input.multiple) {
        value = Array.from(input.selectedOptions).map((opt) => opt.value);
      } else {
        value = input.value;
      }

      state.set_value(fieldName, value as never);
      updateVisibility(form, state);
      updateFieldError(form, fieldName, state);
    };

    // Blur event - mark touched
    const handleBlur = () => {
      state.mark_touched(fieldName);
      updateFieldError(form, fieldName, state);
    };

    input.addEventListener('input', handleInput);
    input.addEventListener('change', handleInput);
    input.addEventListener('blur', handleBlur);
  });
}

/**
 * Binds navigation events for multi-step forms.
 */
function bindNavigationEvents(form: HTMLFormElement, state: FormState): void {
  // Next button
  const nextBtns = form.querySelectorAll('.af-next');
  nextBtns.forEach((btn) => {
    btn.addEventListener('click', (e) => {
      e.preventDefault();
      if (state.next_step()) {
        updateStepVisibility(form, state);
        updateProgress(form, state);
      }
    });
  });

  // Previous button
  const prevBtns = form.querySelectorAll('.af-prev');
  prevBtns.forEach((btn) => {
    btn.addEventListener('click', (e) => {
      e.preventDefault();
      if (state.prev_step()) {
        updateStepVisibility(form, state);
        updateProgress(form, state);
      }
    });
  });
}

/**
 * Binds submit event.
 */
function bindSubmitEvent(form: HTMLFormElement, state: FormState): void {
  form.addEventListener('submit', (e) => {
    // Validate all fields
    const errors = state.validate_all();
    const hasErrors = Object.values(errors).some((e) => e.length > 0);

    if (hasErrors) {
      e.preventDefault();

      // Mark all visible fields as touched
      const visibleFields = state.visible_fields();
      visibleFields.forEach((field) => {
        state.mark_touched(field.name);
        updateFieldError(form, field.name, state);
      });

      return;
    }

    // Let form submit normally (or handle via AJAX if configured)
  });
}

/**
 * Updates visibility of all conditional elements.
 */
function updateVisibility(form: HTMLFormElement, state: FormState): void {
  // Update step visibility
  updateStepVisibility(form, state);

  // Update field visibility
  const fieldElements = form.querySelectorAll('[data-af-field]');
  fieldElements.forEach((fieldEl) => {
    const fieldName = fieldEl.getAttribute('data-af-field');
    if (fieldName) {
      const isVisible = state.is_field_visible(fieldName);
      (fieldEl as HTMLElement).style.display = isVisible ? '' : 'none';
      fieldEl.setAttribute('data-af-visible', String(isVisible));
    }
  });
}

/**
 * Updates step visibility for multi-step forms.
 */
function updateStepVisibility(form: HTMLFormElement, state: FormState): void {
  const currentStep = state.current_step();
  const stepElements = form.querySelectorAll('[data-af-step]');

  stepElements.forEach((stepEl) => {
    const stepIndex = parseInt(stepEl.getAttribute('data-af-step') || '0', 10);
    const visibleSteps = state.visible_steps();
    const currentIndex = state.current_step_index();

    // Check if this step should be shown
    const isCurrentStep = visibleSteps[currentIndex]?.id === stepEl.getAttribute('data-af-step-id') ||
      (currentStep && stepIndex === state.current_step_index());

    (stepEl as HTMLElement).style.display = isCurrentStep ? '' : 'none';
    stepEl.setAttribute('data-af-visible', String(isCurrentStep));
  });

  // Update navigation button states
  const prevBtns = form.querySelectorAll('.af-prev');
  const nextBtns = form.querySelectorAll('.af-next');
  const submitBtns = form.querySelectorAll('.af-submit');

  prevBtns.forEach((btn) => {
    (btn as HTMLButtonElement).disabled = !state.can_go_prev();
  });

  nextBtns.forEach((btn) => {
    (btn as HTMLElement).style.display = state.is_last_step() ? 'none' : '';
  });

  submitBtns.forEach((btn) => {
    (btn as HTMLElement).style.display = state.is_last_step() ? '' : 'none';
  });
}

/**
 * Updates progress indicator.
 */
function updateProgress(form: HTMLFormElement, state: FormState): void {
  const [current, total] = state.progress();
  const progressEl = form.querySelector('.af-progress');
  if (progressEl) {
    const progressBar = progressEl.querySelector('progress');
    if (progressBar) {
      progressBar.value = current;
      progressBar.max = total;
    }
    const progressText = progressEl.querySelector('.af-progress-text');
    if (progressText) {
      progressText.textContent = `Step ${current} of ${total}`;
    }
  }
}

/**
 * Updates error display for a field.
 */
function updateFieldError(
  form: HTMLFormElement,
  fieldName: string,
  state: FormState
): void {
  const errors = state.get_errors(fieldName);
  const isTouched = state.is_touched(fieldName);

  // Find the field container
  const fieldEl = form.querySelector(`[data-af-field="${fieldName}"]`);
  if (!fieldEl) return;

  // Find or create error element
  let errorEl = fieldEl.querySelector('.af-error-message');
  if (!errorEl) {
    errorEl = document.createElement('div');
    errorEl.className = 'af-error-message';
    errorEl.setAttribute('role', 'alert');
    errorEl.setAttribute('aria-live', 'polite');
    errorEl.id = `af-error-${fieldName}`;
    fieldEl.appendChild(errorEl);
  }

  // Update error display
  if (isTouched && errors.length > 0) {
    errorEl.textContent = errors[0];
    fieldEl.classList.add('af-error');

    // Update input aria
    const input = fieldEl.querySelector('input, select, textarea');
    if (input) {
      input.setAttribute('aria-invalid', 'true');
      input.setAttribute('aria-describedby', `af-error-${fieldName}`);
    }
  } else {
    errorEl.textContent = '';
    fieldEl.classList.remove('af-error');

    const input = fieldEl.querySelector('input, select, textarea');
    if (input) {
      input.removeAttribute('aria-invalid');
      input.removeAttribute('aria-describedby');
    }
  }
}

// Export for browser global
if (typeof window !== 'undefined') {
  (window as unknown as Record<string, unknown>).AnyformCore = {
    hydrateAll,
    hydrate,
    FormState,
  };
}

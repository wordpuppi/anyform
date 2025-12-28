/**
 * Mock for @anyform/wasm-js FormState
 *
 * Provides a JavaScript implementation of FormState for testing
 * without loading the actual WASM module.
 */

import type { FormJson, FieldJson, StepJson } from '../../types';

export class MockFormState {
  private _values: Record<string, unknown> = {};
  private _touched: Set<string> = new Set();
  private _errors: Record<string, string[]> = {};
  private _schema: FormJson;
  private _currentStepIndex: number = 0;

  constructor(schema: FormJson | unknown) {
    this._schema = schema as FormJson;

    // Initialize default values from schema
    if (this._schema.steps) {
      for (const step of this._schema.steps) {
        for (const field of step.fields) {
          if (field.default_value !== undefined) {
            this._values[field.name] = field.default_value;
          }
        }
      }
    }
  }

  // Value management
  get_value(field: string): unknown {
    return this._values[field];
  }

  set_value(field: string, value: unknown): void {
    this._values[field] = value;
  }

  get_values(): Record<string, unknown> {
    return { ...this._values };
  }

  // Touch tracking
  mark_touched(field: string): void {
    this._touched.add(field);
  }

  is_touched(field: string): boolean {
    return this._touched.has(field);
  }

  // Validation
  validate_field(fieldName: string): string[] {
    const field = this._findField(fieldName);
    if (!field) return [];

    const value = this._values[fieldName];
    const errors: string[] = [];

    // Required validation
    if (field.required && (value === undefined || value === null || value === '')) {
      errors.push(`${field.label} is required`);
    }

    // Min length validation
    if (
      field.validation?.min_length !== undefined &&
      typeof value === 'string' &&
      value.length < field.validation.min_length
    ) {
      errors.push(`${field.label} must be at least ${field.validation.min_length} characters`);
    }

    // Max length validation
    if (
      field.validation?.max_length !== undefined &&
      typeof value === 'string' &&
      value.length > field.validation.max_length
    ) {
      errors.push(`${field.label} must be at most ${field.validation.max_length} characters`);
    }

    this._errors[fieldName] = errors;
    return errors;
  }

  validate_all(): Record<string, string[]> {
    const allErrors: Record<string, string[]> = {};
    const visibleFields = this.visible_fields() as FieldJson[];

    for (const field of visibleFields) {
      const errors = this.validate_field(field.name);
      if (errors.length > 0) {
        allErrors[field.name] = errors;
      }
    }

    return allErrors;
  }

  validate_step(stepId: string): Record<string, string[]> {
    const step = this._schema.steps.find((s) => s.id === stepId);
    if (!step) return {};

    const stepErrors: Record<string, string[]> = {};
    for (const field of step.fields) {
      const errors = this.validate_field(field.name);
      if (errors.length > 0) {
        stepErrors[field.name] = errors;
      }
    }

    return stepErrors;
  }

  get_errors(field: string): string[] {
    return this._errors[field] || [];
  }

  get_all_errors(): Record<string, string[]> {
    return { ...this._errors };
  }

  is_valid(): boolean {
    return Object.values(this._errors).every((errors) => errors.length === 0);
  }

  // Visibility
  visible_fields(stepId?: string | null): FieldJson[] {
    if (stepId) {
      const step = this._schema.steps.find((s) => s.id === stepId);
      return step?.fields || [];
    }

    // Return fields for current step
    const currentStep = this._schema.steps[this._currentStepIndex];
    return currentStep?.fields || [];
  }

  visible_steps(): StepJson[] {
    return this._schema.steps || [];
  }

  is_field_visible(fieldName: string): boolean {
    const field = this._findField(fieldName);
    return field !== null;
  }

  is_step_visible(stepId: string): boolean {
    return this._schema.steps.some((s) => s.id === stepId);
  }

  // Multi-step navigation
  current_step(): StepJson | null {
    return this._schema.steps[this._currentStepIndex] || null;
  }

  current_step_index(): number {
    return this._currentStepIndex;
  }

  next_step(): boolean {
    if (this._currentStepIndex < this._schema.steps.length - 1) {
      this._currentStepIndex++;
      return true;
    }
    return false;
  }

  prev_step(): boolean {
    if (this._currentStepIndex > 0) {
      this._currentStepIndex--;
      return true;
    }
    return false;
  }

  can_go_next(): boolean {
    return this._currentStepIndex < this._schema.steps.length - 1;
  }

  can_go_prev(): boolean {
    return this._currentStepIndex > 0;
  }

  is_last_step(): boolean {
    return this._currentStepIndex === this._schema.steps.length - 1;
  }

  progress(): Uint32Array {
    return new Uint32Array([this._currentStepIndex + 1, this._schema.steps.length]);
  }

  go_to_step(stepId: string): boolean {
    const index = this._schema.steps.findIndex((s) => s.id === stepId);
    if (index !== -1) {
      this._currentStepIndex = index;
      return true;
    }
    return false;
  }

  // Schema access
  name(): string {
    return this._schema.name;
  }

  slug(): string {
    return this._schema.slug;
  }

  schema(): FormJson {
    return this._schema;
  }

  // Helper methods
  private _findField(fieldName: string): FieldJson | null {
    for (const step of this._schema.steps) {
      const field = step.fields.find((f) => f.name === fieldName);
      if (field) return field;
    }
    return null;
  }

  // For testing: allow setting errors directly
  _setErrors(errors: Record<string, string[]>): void {
    this._errors = errors;
  }

  // For testing: reset state
  _reset(): void {
    this._values = {};
    this._touched = new Set();
    this._errors = {};
    this._currentStepIndex = 0;
  }
}

/**
 * Creates a mock form schema for testing.
 */
export function createMockSchema(overrides?: Partial<FormJson>): FormJson {
  return {
    id: 'test-form-id',
    name: 'Test Form',
    slug: 'test-form',
    description: 'A test form',
    settings: {
      submit_label: 'Submit',
      success_message: 'Thank you!',
    },
    steps: [
      {
        id: 'step-1',
        name: 'Step 1',
        order: 0,
        fields: [
          {
            id: 'field-name',
            name: 'name',
            label: 'Name',
            field_type: 'text',
            order: 0,
            required: true,
            validation: {},
            ui_options: {},
            options: [],
          },
          {
            id: 'field-email',
            name: 'email',
            label: 'Email',
            field_type: 'email',
            order: 1,
            required: true,
            validation: {},
            ui_options: {},
            options: [],
          },
        ],
      },
    ],
    ...overrides,
  };
}

/**
 * Creates a multi-step mock schema for testing.
 */
export function createMultiStepSchema(): FormJson {
  return {
    id: 'multi-step-form',
    name: 'Multi-Step Form',
    slug: 'multi-step',
    settings: {},
    steps: [
      {
        id: 'step-1',
        name: 'Personal Info',
        order: 0,
        fields: [
          {
            id: 'field-name',
            name: 'name',
            label: 'Name',
            field_type: 'text',
            order: 0,
            required: true,
            validation: {},
            ui_options: {},
            options: [],
          },
        ],
      },
      {
        id: 'step-2',
        name: 'Contact Info',
        order: 1,
        fields: [
          {
            id: 'field-email',
            name: 'email',
            label: 'Email',
            field_type: 'email',
            order: 0,
            required: true,
            validation: {},
            ui_options: {},
            options: [],
          },
        ],
      },
      {
        id: 'step-3',
        name: 'Confirmation',
        order: 2,
        fields: [
          {
            id: 'field-agree',
            name: 'agree',
            label: 'I agree to terms',
            field_type: 'checkbox',
            order: 0,
            required: true,
            validation: {},
            ui_options: {},
            options: [],
          },
        ],
      },
    ],
  };
}

/**
 * Creates a schema with various field types for testing.
 */
export function createFieldTypesSchema(): FormJson {
  return {
    id: 'field-types-form',
    name: 'Field Types Form',
    slug: 'field-types',
    settings: {},
    steps: [
      {
        id: 'step-1',
        name: 'All Fields',
        order: 0,
        fields: [
          {
            id: 'field-text',
            name: 'text_field',
            label: 'Text Field',
            field_type: 'text',
            order: 0,
            required: false,
            placeholder: 'Enter text',
            validation: { min_length: 2, max_length: 100 },
            ui_options: {},
            options: [],
          },
          {
            id: 'field-email',
            name: 'email_field',
            label: 'Email Field',
            field_type: 'email',
            order: 1,
            required: true,
            validation: {},
            ui_options: {},
            options: [],
          },
          {
            id: 'field-select',
            name: 'select_field',
            label: 'Select Field',
            field_type: 'select',
            order: 2,
            required: false,
            validation: {},
            ui_options: {},
            options: [
              { id: 'opt-1', label: 'Option 1', value: 'opt1', order: 0 },
              { id: 'opt-2', label: 'Option 2', value: 'opt2', order: 1 },
              { id: 'opt-3', label: 'Option 3', value: 'opt3', order: 2 },
            ],
          },
          {
            id: 'field-multi-select',
            name: 'multi_select_field',
            label: 'Multi-Select Field',
            field_type: 'multi_select',
            order: 3,
            required: false,
            validation: {},
            ui_options: {},
            options: [
              { id: 'opt-a', label: 'Choice A', value: 'a', order: 0 },
              { id: 'opt-b', label: 'Choice B', value: 'b', order: 1 },
            ],
          },
          {
            id: 'field-checkbox',
            name: 'checkbox_field',
            label: 'Checkbox Field',
            field_type: 'checkbox',
            order: 4,
            required: false,
            validation: {},
            ui_options: {},
            options: [],
          },
          {
            id: 'field-radio',
            name: 'radio_field',
            label: 'Radio Field',
            field_type: 'radio',
            order: 5,
            required: false,
            validation: {},
            ui_options: {},
            options: [
              { id: 'radio-yes', label: 'Yes', value: 'yes', order: 0 },
              { id: 'radio-no', label: 'No', value: 'no', order: 1 },
            ],
          },
          {
            id: 'field-textarea',
            name: 'textarea_field',
            label: 'Textarea Field',
            field_type: 'textarea',
            order: 6,
            required: false,
            validation: {},
            ui_options: { rows: 4 },
            options: [],
          },
        ],
      },
    ],
  };
}

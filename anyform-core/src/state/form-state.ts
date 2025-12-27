/**
 * FormState - Client-side form state manager.
 *
 * Ported from anyform-client/src/form_state.rs
 *
 * This class manages:
 * - Field values
 * - Validation errors
 * - Touched state (user interaction tracking)
 * - Multi-step navigation
 * - Conditional visibility
 */

import type {
  FormJson,
  StepJson,
  FieldJson,
  JsonValue,
  ConditionRule,
} from '../types';
import { validateField } from '../validation';
import { evaluateCondition } from '../condition';

/**
 * Client-side form state manager.
 *
 * Provides the same API as the WASM FormState for drop-in compatibility.
 */
export class FormState {
  private _schema: FormJson;
  private _values: Map<string, JsonValue>;
  private _errors: Map<string, string[]>;
  private _touched: Set<string>;
  private _currentStepIndex: number;

  /**
   * Creates a new FormState from a form schema.
   */
  constructor(schema: FormJson) {
    this._schema = schema;
    this._values = new Map();
    this._errors = new Map();
    this._touched = new Set();
    this._currentStepIndex = 0;

    // Initialize with default values
    for (const step of schema.steps) {
      for (const field of step.fields) {
        if (field.default_value !== undefined) {
          this._values.set(field.name, field.default_value);
        }
      }
    }
  }

  // ─────────────────────────────────────────────────────────────────────────
  // Value management
  // ─────────────────────────────────────────────────────────────────────────

  /**
   * Sets a field value.
   */
  set_value(field: string, value: JsonValue): void {
    this._values.set(field, value);

    // Re-validate if field was touched
    if (this._touched.has(field)) {
      this.validateFieldInternal(field);
    }
  }

  /**
   * Gets a field value.
   */
  get_value(field: string): JsonValue | null {
    return this._values.get(field) ?? null;
  }

  /**
   * Gets all values as an object.
   */
  get_values(): Record<string, JsonValue> {
    const result: Record<string, JsonValue> = {};
    for (const [key, value] of this._values) {
      result[key] = value;
    }
    return result;
  }

  // ─────────────────────────────────────────────────────────────────────────
  // Touch tracking
  // ─────────────────────────────────────────────────────────────────────────

  /**
   * Marks a field as touched (user has interacted with it).
   */
  mark_touched(field: string): void {
    this._touched.add(field);
    // Validate on touch
    this.validateFieldInternal(field);
  }

  /**
   * Checks if a field has been touched.
   */
  is_touched(field: string): boolean {
    return this._touched.has(field);
  }

  // ─────────────────────────────────────────────────────────────────────────
  // Validation
  // ─────────────────────────────────────────────────────────────────────────

  /**
   * Validates a specific field and returns errors.
   */
  validate_field(fieldName: string): string[] {
    this.validateFieldInternal(fieldName);
    return this._errors.get(fieldName) ?? [];
  }

  /**
   * Validates all fields in a step.
   */
  validate_step(stepId: string): Record<string, string[]> {
    const stepErrors: Record<string, string[]> = {};
    const step = this._schema.steps.find((s) => s.id === stepId);

    if (step) {
      for (const field of step.fields) {
        // Skip hidden fields
        if (!this.isFieldVisibleInternal(field.name)) {
          continue;
        }

        const value = this._values.get(field.name) ?? null;
        const errors = validateField(field, value);
        if (errors.length > 0) {
          stepErrors[field.name] = errors;
          this._errors.set(field.name, errors);
        } else {
          this._errors.delete(field.name);
        }
      }
    }

    return stepErrors;
  }

  /**
   * Validates all visible fields in the form.
   */
  validate_all(): Record<string, string[]> {
    for (const step of this._schema.steps) {
      // Skip hidden steps
      if (!this.isStepVisibleInternal(step.id)) {
        continue;
      }

      for (const field of step.fields) {
        // Skip hidden fields
        if (!this.isFieldVisibleInternal(field.name)) {
          continue;
        }

        this.validateFieldInternal(field.name);
      }
    }

    const result: Record<string, string[]> = {};
    for (const [key, value] of this._errors) {
      result[key] = value;
    }
    return result;
  }

  /**
   * Returns true if the form has no validation errors.
   */
  is_valid(): boolean {
    if (this._errors.size === 0) return true;
    for (const errors of this._errors.values()) {
      if (errors.length > 0) return false;
    }
    return true;
  }

  /**
   * Gets errors for a specific field.
   */
  get_errors(field: string): string[] {
    return this._errors.get(field) ?? [];
  }

  /**
   * Gets all errors as an object.
   */
  get_all_errors(): Record<string, string[]> {
    const result: Record<string, string[]> = {};
    for (const [key, value] of this._errors) {
      result[key] = value;
    }
    return result;
  }

  // ─────────────────────────────────────────────────────────────────────────
  // Visibility (condition evaluation)
  // ─────────────────────────────────────────────────────────────────────────

  /**
   * Returns all visible steps.
   */
  visible_steps(): StepJson[] {
    return this._schema.steps.filter((s) =>
      this.isStepVisibleInternal(s.id)
    );
  }

  /**
   * Returns visible fields, optionally filtered by step.
   */
  visible_fields(stepId?: string): FieldJson[] {
    const fields: FieldJson[] = [];

    const steps = stepId
      ? this._schema.steps.filter((s) => s.id === stepId)
      : this._schema.steps;

    for (const step of steps) {
      if (!this.isStepVisibleInternal(step.id)) {
        continue;
      }
      for (const field of step.fields) {
        if (this.isFieldVisibleInternal(field.name)) {
          fields.push(field);
        }
      }
    }

    return fields;
  }

  /**
   * Checks if a step is visible.
   */
  is_step_visible(stepId: string): boolean {
    return this.isStepVisibleInternal(stepId);
  }

  /**
   * Checks if a field is visible.
   */
  is_field_visible(fieldName: string): boolean {
    return this.isFieldVisibleInternal(fieldName);
  }

  // ─────────────────────────────────────────────────────────────────────────
  // Multi-step navigation
  // ─────────────────────────────────────────────────────────────────────────

  /**
   * Returns the current step.
   */
  current_step(): StepJson | null {
    const visibleSteps = this.getVisibleStepIndices();
    const index = visibleSteps[this._currentStepIndex];
    if (index !== undefined) {
      return this._schema.steps[index] ?? null;
    }
    return null;
  }

  /**
   * Returns the current step index (0-based, among visible steps).
   */
  current_step_index(): number {
    return this._currentStepIndex;
  }

  /**
   * Moves to the next visible step. Returns true if successful.
   */
  next_step(): boolean {
    const visibleSteps = this.getVisibleStepIndices();
    if (this._currentStepIndex + 1 < visibleSteps.length) {
      this._currentStepIndex += 1;
      return true;
    }
    return false;
  }

  /**
   * Moves to the previous visible step. Returns true if successful.
   */
  prev_step(): boolean {
    if (this._currentStepIndex > 0) {
      this._currentStepIndex -= 1;
      return true;
    }
    return false;
  }

  /**
   * Navigates to a specific step by ID. Returns true if successful.
   */
  go_to_step(stepId: string): boolean {
    const visibleSteps = this.getVisibleStepIndices();
    for (let visibleIndex = 0; visibleIndex < visibleSteps.length; visibleIndex++) {
      const actualIndex = visibleSteps[visibleIndex];
      const step = this._schema.steps[actualIndex];
      if (step?.id === stepId) {
        this._currentStepIndex = visibleIndex;
        return true;
      }
    }
    return false;
  }

  /**
   * Returns true if we can advance to the next step.
   */
  can_go_next(): boolean {
    const visibleSteps = this.getVisibleStepIndices();
    return this._currentStepIndex + 1 < visibleSteps.length;
  }

  /**
   * Returns true if we can go back to the previous step.
   */
  can_go_prev(): boolean {
    return this._currentStepIndex > 0;
  }

  /**
   * Returns progress as [current, total] (1-indexed for display).
   */
  progress(): [number, number] {
    const visibleSteps = this.getVisibleStepIndices();
    return [this._currentStepIndex + 1, visibleSteps.length];
  }

  /**
   * Returns true if currently on the last step.
   */
  is_last_step(): boolean {
    const visibleSteps = this.getVisibleStepIndices();
    return this._currentStepIndex + 1 >= visibleSteps.length;
  }

  // ─────────────────────────────────────────────────────────────────────────
  // Schema access
  // ─────────────────────────────────────────────────────────────────────────

  /**
   * Returns the form slug.
   */
  slug(): string {
    return this._schema.slug;
  }

  /**
   * Returns the form name.
   */
  name(): string {
    return this._schema.name;
  }

  /**
   * Returns the form schema.
   */
  schema(): FormJson {
    return this._schema;
  }

  /**
   * Returns the action URL for form submission.
   */
  action_url(): string {
    return (
      this._schema.action_url ??
      this._schema.settings.action_url ??
      `/api/forms/${this._schema.slug}`
    );
  }

  /**
   * Returns the HTTP method for form submission.
   */
  action_method(): string {
    return (
      this._schema.action_method ??
      this._schema.settings.method ??
      'POST'
    );
  }

  // ─────────────────────────────────────────────────────────────────────────
  // Internal methods
  // ─────────────────────────────────────────────────────────────────────────

  private validateFieldInternal(fieldName: string): void {
    // Find the field in the schema
    for (const step of this._schema.steps) {
      for (const field of step.fields) {
        if (field.name === fieldName) {
          const value = this._values.get(fieldName) ?? null;
          const errors = validateField(field, value);
          if (errors.length === 0) {
            this._errors.delete(fieldName);
          } else {
            this._errors.set(fieldName, errors);
          }
          return;
        }
      }
    }
  }

  private isStepVisibleInternal(stepId: string): boolean {
    const step = this._schema.steps.find((s) => s.id === stepId);
    if (!step) return false;
    return this.evaluateCondition(step.condition);
  }

  private isFieldVisibleInternal(fieldName: string): boolean {
    for (const step of this._schema.steps) {
      // Skip if step is hidden
      if (!this.evaluateCondition(step.condition)) {
        continue;
      }

      for (const field of step.fields) {
        if (field.name === fieldName) {
          return this.evaluateCondition(field.condition);
        }
      }
    }
    return false;
  }

  private getVisibleStepIndices(): number[] {
    return this._schema.steps
      .map((step, i) => ({ step, i }))
      .filter(({ step }) => this.evaluateCondition(step.condition))
      .map(({ i }) => i);
  }

  private evaluateCondition(condition: ConditionRule | undefined): boolean {
    if (!condition) return true; // No condition = always visible
    return evaluateCondition(condition, this.get_values());
  }
}

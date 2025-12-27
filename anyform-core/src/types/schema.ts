/**
 * Schema types for anyform forms.
 *
 * These types mirror the Rust schema types from anyform-client.
 */

import type { ConditionRule } from './condition';

/**
 * JSON value type for form field values.
 */
export type JsonValue =
  | string
  | number
  | boolean
  | null
  | JsonValue[]
  | { [key: string]: JsonValue };

/**
 * Field value types.
 */
export type ValueType =
  | 'text'
  | 'textarea'
  | 'email'
  | 'url'
  | 'tel'
  | 'number'
  | 'date'
  | 'time'
  | 'datetime'
  | 'select'
  | 'multi_select'
  | 'radio'
  | 'checkbox'
  | 'file'
  | 'hidden'
  | 'password'
  | 'color'
  | 'range'
  | 'rating'
  | 'scale'
  | 'nps';

/**
 * Validation rules for a field.
 */
export interface ValidationRules {
  required?: boolean;
  min_length?: number;
  max_length?: number;
  pattern?: string;
  pattern_message?: string;
  min_value?: number;
  max_value?: number;
  min_selections?: number;
  max_selections?: number;
}

/**
 * Option for select/radio/checkbox fields.
 */
export interface FieldOptionJson {
  id: string;
  label: string;
  value: string;
  score?: number;
  order: number;
}

/**
 * UI options for field rendering.
 */
export interface UiOptions {
  disabled?: boolean;
  readonly?: boolean;
  autofocus?: boolean;
  autocomplete?: string;
  rows?: number;
  cols?: number;
  condition?: ConditionRule;
}

/**
 * Field in a form step.
 */
export interface FieldJson {
  id: string;
  name: string;
  label: string;
  field_type: ValueType;
  placeholder?: string;
  help_text?: string;
  default_value?: JsonValue;
  required?: boolean;
  validation: ValidationRules;
  condition?: ConditionRule;
  ui_options?: UiOptions;
  options: FieldOptionJson[];
  order: number;
}

/**
 * Step in a multi-step form.
 */
export interface StepJson {
  id: string;
  name: string;
  description?: string;
  order: number;
  condition?: ConditionRule;
  fields: FieldJson[];
}

/**
 * Form settings.
 */
export interface FormSettings {
  multi_step?: boolean;
  submit_button_text?: string;
  success_message?: string;
  success_redirect?: string;
  show_progress?: boolean;
  allow_save_draft?: boolean;
  action_url?: string;
  method?: string;
}

/**
 * Form JSON schema returned by the API.
 */
export interface FormJson {
  id: string;
  name: string;
  slug: string;
  description?: string;
  action_url?: string;
  action_method?: string;
  settings: FormSettings;
  steps: StepJson[];
}

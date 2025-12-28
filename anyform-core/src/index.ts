/**
 * @wordpuppi/anyform-core - Pure TypeScript form validation and state management
 *
 * This package provides the core form engine for anyform, including:
 * - FormState class for managing form values, validation, and multi-step navigation
 * - Validation functions for common field types
 * - Condition evaluation for dynamic visibility
 *
 * @example
 * ```ts
 * import { FormState } from '@wordpuppi/anyform-core';
 *
 * const state = new FormState(schema);
 * state.set_value('email', 'user@example.com');
 * const errors = state.validate_all();
 * ```
 */

// State management
export { FormState } from './state';

// Validation
export {
  validateField,
  isEmpty,
  validateRequired,
  getNumericValue,
  validateEmail,
  validateUrl,
  validateTel,
  validateDate,
  validateTime,
  validateDateTime,
  validateNumber,
  validateMinLength,
  validateMaxLength,
  validatePattern,
  validateMinValue,
  validateMaxValue,
  validateMinSelections,
  validateMaxSelections,
} from './validation';

// Condition evaluation
export {
  evaluateCondition,
  isEmptyValue,
  valuesEqual,
  compareNumeric,
  stringContains,
  stringStartsWith,
  stringEndsWith,
  valueIn,
} from './condition';

// Types
export type {
  // Schema types
  JsonValue,
  ValueType,
  ValidationRules,
  FieldOptionJson,
  UiOptions,
  FieldJson,
  StepJson,
  FormSettings,
  FormJson,
  // Condition types
  ConditionOp,
  SimpleCondition,
  AndCondition,
  OrCondition,
  ConditionRule,
} from './types';

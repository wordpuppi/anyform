/**
 * Validation module exports.
 *
 * Provides the main validateField function and individual validators.
 */

import type { FieldJson, JsonValue } from '../types';
import { isEmpty } from './validators';
import {
  validateEmail,
  validateUrl,
  validateTel,
  validateDate,
  validateTime,
  validateDateTime,
  validateNumber,
} from './type-validators';
import {
  validateMinLength,
  validateMaxLength,
  validatePattern,
  validateMinValue,
  validateMaxValue,
  validateMinSelections,
  validateMaxSelections,
} from './rules';

export { isEmpty, validateRequired, getNumericValue } from './validators';
export {
  validateEmail,
  validateUrl,
  validateTel,
  validateDate,
  validateTime,
  validateDateTime,
  validateNumber,
} from './type-validators';
export {
  validateMinLength,
  validateMaxLength,
  validatePattern,
  validateMinValue,
  validateMaxValue,
  validateMinSelections,
  validateMaxSelections,
} from './rules';

/**
 * Validates a field value against its rules.
 *
 * This function mirrors the Rust validate_field function.
 *
 * @param field - The field definition
 * @param value - The current field value
 * @returns Array of error messages (empty if valid)
 */
export function validateField(
  field: FieldJson,
  value: JsonValue | undefined
): string[] {
  const errors: string[] = [];
  const rules = field.validation;
  const label = field.label;

  // Required validation - check this first
  const isRequired = rules.required || field.required;
  if (isRequired && isEmpty(value)) {
    errors.push(`${label} is required`);
    return errors; // Skip other validations if empty and required
  }

  // Skip other validations if value is empty
  if (isEmpty(value)) {
    return errors;
  }

  // Type-specific validations
  switch (field.field_type) {
    case 'email': {
      const error = validateEmail(value!);
      if (error) errors.push(error);
      break;
    }
    case 'url': {
      const error = validateUrl(value!);
      if (error) errors.push(error);
      break;
    }
    case 'tel': {
      const error = validateTel(value!);
      if (error) errors.push(error);
      break;
    }
    case 'date': {
      const error = validateDate(value!);
      if (error) errors.push(error);
      break;
    }
    case 'time': {
      const error = validateTime(value!);
      if (error) errors.push(error);
      break;
    }
    case 'datetime': {
      const error = validateDateTime(value!);
      if (error) errors.push(error);
      break;
    }
    case 'number':
    case 'range':
    case 'rating':
    case 'scale':
    case 'nps': {
      const error = validateNumber(value!);
      if (error) errors.push(error);
      break;
    }
  }

  // Rule-based validations
  if (rules.min_length !== undefined) {
    const error = validateMinLength(value!, rules.min_length, label);
    if (error) errors.push(error);
  }

  if (rules.max_length !== undefined) {
    const error = validateMaxLength(value!, rules.max_length, label);
    if (error) errors.push(error);
  }

  if (rules.pattern !== undefined) {
    const error = validatePattern(
      value!,
      rules.pattern,
      rules.pattern_message,
      label
    );
    if (error) errors.push(error);
  }

  if (rules.min_value !== undefined) {
    const error = validateMinValue(value!, rules.min_value, label);
    if (error) errors.push(error);
  }

  if (rules.max_value !== undefined) {
    const error = validateMaxValue(value!, rules.max_value, label);
    if (error) errors.push(error);
  }

  if (rules.min_selections !== undefined) {
    const error = validateMinSelections(value!, rules.min_selections);
    if (error) errors.push(error);
  }

  if (rules.max_selections !== undefined) {
    const error = validateMaxSelections(value!, rules.max_selections);
    if (error) errors.push(error);
  }

  return errors;
}

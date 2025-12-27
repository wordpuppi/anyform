/**
 * Rule-based validators.
 *
 * Ported from anyform-client/src/validation.rs
 */

import type { JsonValue } from '../types';
import { getNumericValue } from './validators';

/**
 * Validates minimum string length.
 */
export function validateMinLength(
  value: JsonValue,
  min: number,
  label: string
): string | null {
  if (typeof value !== 'string') return null;

  if (value.length < min) {
    return `${label} must be at least ${min} characters`;
  }
  return null;
}

/**
 * Validates maximum string length.
 */
export function validateMaxLength(
  value: JsonValue,
  max: number,
  label: string
): string | null {
  if (typeof value !== 'string') return null;

  if (value.length > max) {
    return `${label} must be at most ${max} characters`;
  }
  return null;
}

/**
 * Validates a regex pattern.
 */
export function validatePattern(
  value: JsonValue,
  pattern: string,
  message: string | undefined,
  label: string
): string | null {
  if (typeof value !== 'string') return null;

  try {
    const regex = new RegExp(pattern);
    if (!regex.test(value)) {
      return message || `${label} does not match the required format`;
    }
  } catch {
    // Invalid regex pattern, skip validation
  }
  return null;
}

/**
 * Validates minimum numeric value.
 */
export function validateMinValue(
  value: JsonValue,
  min: number,
  label: string
): string | null {
  const num = getNumericValue(value);
  if (num === null) return null;

  if (num < min) {
    return `${label} must be at least ${min}`;
  }
  return null;
}

/**
 * Validates maximum numeric value.
 */
export function validateMaxValue(
  value: JsonValue,
  max: number,
  label: string
): string | null {
  const num = getNumericValue(value);
  if (num === null) return null;

  if (num > max) {
    return `${label} must be at most ${max}`;
  }
  return null;
}

/**
 * Validates minimum number of selections (for arrays).
 */
export function validateMinSelections(
  value: JsonValue,
  min: number
): string | null {
  if (!Array.isArray(value)) return null;

  if (value.length < min) {
    return `Select at least ${min} option${min === 1 ? '' : 's'}`;
  }
  return null;
}

/**
 * Validates maximum number of selections (for arrays).
 */
export function validateMaxSelections(
  value: JsonValue,
  max: number
): string | null {
  if (!Array.isArray(value)) return null;

  if (value.length > max) {
    return `Select at most ${max} option${max === 1 ? '' : 's'}`;
  }
  return null;
}

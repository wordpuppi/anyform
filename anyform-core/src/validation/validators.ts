/**
 * Core validation functions.
 *
 * Ported from anyform-client/src/validation.rs
 */

import type { JsonValue } from '../types';

/**
 * Checks if a value is considered empty.
 */
export function isEmpty(value: JsonValue | undefined): boolean {
  if (value === null || value === undefined) return true;
  if (typeof value === 'string') return value.trim() === '';
  if (Array.isArray(value)) return value.length === 0;
  if (typeof value === 'object') return Object.keys(value).length === 0;
  return false;
}

/**
 * Validates a required field.
 */
export function validateRequired(
  value: JsonValue | undefined,
  label: string
): string | null {
  if (isEmpty(value)) {
    return `${label} is required`;
  }
  return null;
}

/**
 * Extracts a numeric value from a JSON value.
 */
export function getNumericValue(value: JsonValue | undefined): number | null {
  if (value === null || value === undefined) return null;
  if (typeof value === 'number') return value;
  if (typeof value === 'string') {
    const parsed = parseFloat(value);
    return isNaN(parsed) ? null : parsed;
  }
  return null;
}

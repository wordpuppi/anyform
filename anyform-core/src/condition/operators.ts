/**
 * Condition operators for comparing field values.
 *
 * Ported from anyform/src/condition.rs and anyform-client/src/schema.rs
 */

import type { JsonValue } from '../types';

/**
 * Checks if a value is considered empty.
 */
export function isEmptyValue(value: JsonValue | undefined): boolean {
  if (value === null || value === undefined) return true;
  if (typeof value === 'string') return value === '';
  if (Array.isArray(value)) return value.length === 0;
  if (typeof value === 'object') return Object.keys(value).length === 0;
  return false;
}

/**
 * Tries to extract a number from a JSON value.
 */
function asNumber(value: JsonValue | undefined): number | null {
  if (value === null || value === undefined) return null;
  if (typeof value === 'number') return value;
  if (typeof value === 'string') {
    const parsed = parseFloat(value);
    return isNaN(parsed) ? null : parsed;
  }
  return null;
}

/**
 * Tries to extract a string from a JSON value.
 */
function asString(value: JsonValue | undefined): string | null {
  if (typeof value === 'string') return value;
  return null;
}

/**
 * Compares two values for equality with type coercion.
 *
 * Matches Rust behavior:
 * - Numeric comparison with coercion (string "18" == number 18)
 * - Boolean comparison with string coercion ("true" == true)
 * - Fallback to strict equality
 */
export function valuesEqual(a: JsonValue | undefined, b: JsonValue | undefined): boolean {
  // Handle null/undefined
  if (a === null || a === undefined) {
    return b === null || b === undefined;
  }
  if (b === null || b === undefined) {
    return false;
  }

  // Numeric comparison with coercion
  const aNum = asNumber(a);
  const bNum = asNumber(b);
  if (aNum !== null && bNum !== null) {
    return Math.abs(aNum - bNum) < Number.EPSILON;
  }

  // String comparison
  const aStr = asString(a);
  const bStr = asString(b);
  if (aStr !== null && bStr !== null) {
    return aStr === bStr;
  }

  // Boolean comparison with string coercion
  if (typeof a === 'boolean' && typeof b === 'string') {
    return (a === true && b === 'true') || (a === false && b === 'false');
  }
  if (typeof a === 'string' && typeof b === 'boolean') {
    return (a === 'true' && b === true) || (a === 'false' && b === false);
  }

  // Boolean comparison
  if (typeof a === 'boolean' && typeof b === 'boolean') {
    return a === b;
  }

  // Array comparison
  if (Array.isArray(a) && Array.isArray(b)) {
    if (a.length !== b.length) return false;
    return a.every((val, i) => valuesEqual(val, b[i]));
  }

  // Fallback to JSON equality
  return JSON.stringify(a) === JSON.stringify(b);
}

/**
 * Compares two values numerically.
 */
export function compareNumeric(
  op: 'gt' | 'gte' | 'lt' | 'lte',
  fieldValue: JsonValue | undefined,
  conditionValue: JsonValue | undefined
): boolean {
  const fieldNum = asNumber(fieldValue);
  const condNum = asNumber(conditionValue);

  if (fieldNum === null || condNum === null) return false;

  switch (op) {
    case 'gt':
      return fieldNum > condNum;
    case 'gte':
      return fieldNum >= condNum;
    case 'lt':
      return fieldNum < condNum;
    case 'lte':
      return fieldNum <= condNum;
  }
}

/**
 * Checks if a field value contains a substring or array element.
 */
export function stringContains(
  fieldValue: JsonValue | undefined,
  conditionValue: JsonValue | undefined
): boolean {
  // String contains substring
  if (typeof fieldValue === 'string' && typeof conditionValue === 'string') {
    return fieldValue.includes(conditionValue);
  }
  // Array contains value
  if (Array.isArray(fieldValue)) {
    return fieldValue.some((item) => valuesEqual(item, conditionValue));
  }
  return false;
}

/**
 * Checks if a field value is in an array.
 */
export function valueIn(
  fieldValue: JsonValue | undefined,
  conditionValue: JsonValue | undefined
): boolean {
  if (!Array.isArray(conditionValue)) return false;
  return conditionValue.some((item) => valuesEqual(fieldValue, item));
}

/**
 * Checks if a string starts with a prefix.
 */
export function stringStartsWith(
  fieldValue: JsonValue | undefined,
  conditionValue: JsonValue | undefined
): boolean {
  if (typeof fieldValue === 'string' && typeof conditionValue === 'string') {
    return fieldValue.startsWith(conditionValue);
  }
  return false;
}

/**
 * Checks if a string ends with a suffix.
 */
export function stringEndsWith(
  fieldValue: JsonValue | undefined,
  conditionValue: JsonValue | undefined
): boolean {
  if (typeof fieldValue === 'string' && typeof conditionValue === 'string') {
    return fieldValue.endsWith(conditionValue);
  }
  return false;
}

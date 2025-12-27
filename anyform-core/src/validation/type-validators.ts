/**
 * Type-specific validators.
 *
 * Ported from anyform-client/src/validation.rs
 */

import type { JsonValue } from '../types';

// Regex patterns matching Rust implementation
const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
const URL_REGEX = /^https?:\/\/[^\s]+$/;
const TEL_REGEX = /^[\d\s\-\(\)\+]+$/;
const DATE_REGEX = /^\d{4}-\d{2}-\d{2}$/;
const TIME_REGEX = /^\d{2}:\d{2}(:\d{2})?$/;
const DATETIME_REGEX = /^\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}(:\d{2})?/;

/**
 * Validates an email address.
 */
export function validateEmail(value: JsonValue): string | null {
  if (typeof value !== 'string') return null;

  if (!EMAIL_REGEX.test(value)) {
    return 'Invalid email format';
  }
  return null;
}

/**
 * Validates a URL.
 */
export function validateUrl(value: JsonValue): string | null {
  if (typeof value !== 'string') return null;

  if (!URL_REGEX.test(value)) {
    return 'Invalid URL format';
  }
  return null;
}

/**
 * Validates a telephone number.
 */
export function validateTel(value: JsonValue): string | null {
  if (typeof value !== 'string') return null;

  if (!TEL_REGEX.test(value)) {
    return 'Invalid phone number format';
  }

  // Ensure at least 7 digits (matching Rust)
  const digitCount = (value.match(/\d/g) || []).length;
  if (digitCount < 7) {
    return 'Invalid phone number format';
  }

  return null;
}

/**
 * Validates a date (YYYY-MM-DD).
 */
export function validateDate(value: JsonValue): string | null {
  if (typeof value !== 'string') return null;

  if (!DATE_REGEX.test(value)) {
    return 'Invalid date format (YYYY-MM-DD)';
  }
  return null;
}

/**
 * Validates a time (HH:MM or HH:MM:SS).
 */
export function validateTime(value: JsonValue): string | null {
  if (typeof value !== 'string') return null;

  if (!TIME_REGEX.test(value)) {
    return 'Invalid time format (HH:MM)';
  }
  return null;
}

/**
 * Validates a datetime.
 */
export function validateDateTime(value: JsonValue): string | null {
  if (typeof value !== 'string') return null;

  if (!DATETIME_REGEX.test(value)) {
    return 'Invalid date and time format';
  }
  return null;
}

/**
 * Validates a number field.
 */
export function validateNumber(value: JsonValue): string | null {
  if (value === null || value === undefined || value === '') return null;

  if (typeof value === 'number') return null;

  if (typeof value === 'string') {
    const num = parseFloat(value);
    if (isNaN(num)) {
      return 'Must be a number';
    }
  }

  return null;
}

/**
 * Condition evaluator for dynamic visibility.
 *
 * Ported from anyform/src/condition.rs
 */

import type { ConditionRule, JsonValue } from '../types';
import {
  isEmptyValue,
  valuesEqual,
  compareNumeric,
  stringContains,
  stringStartsWith,
  stringEndsWith,
  valueIn,
} from './operators';

/**
 * Evaluates a condition against form values.
 *
 * @param condition - The condition rule to evaluate
 * @param values - A map of field names to their current values
 * @returns true if the condition is satisfied, false otherwise
 */
export function evaluateCondition(
  condition: ConditionRule | undefined,
  values: Record<string, JsonValue>
): boolean {
  // No condition = always visible
  if (!condition) return true;

  // AND condition
  if ('and' in condition) {
    return condition.and.every((rule) => evaluateCondition(rule, values));
  }

  // OR condition
  if ('or' in condition) {
    return condition.or.some((rule) => evaluateCondition(rule, values));
  }

  // Simple condition
  const fieldValue = values[condition.field];
  const op = condition.op;
  const conditionValue = condition.value;

  // Normalize operator aliases
  const normalizedOp = normalizeOp(op);

  switch (normalizedOp) {
    case 'empty':
      return isEmptyValue(fieldValue);

    case 'not_empty':
      return !isEmptyValue(fieldValue);

    case 'eq':
      return valuesEqual(fieldValue, conditionValue);

    case 'neq':
      return !valuesEqual(fieldValue, conditionValue);

    case 'gt':
    case 'gte':
    case 'lt':
    case 'lte':
      return compareNumeric(normalizedOp, fieldValue, conditionValue);

    case 'contains':
      return stringContains(fieldValue, conditionValue);

    case 'not_contains':
      return !stringContains(fieldValue, conditionValue);

    case 'starts_with':
      return stringStartsWith(fieldValue, conditionValue);

    case 'ends_with':
      return stringEndsWith(fieldValue, conditionValue);

    case 'in':
      return valueIn(fieldValue, conditionValue);

    default:
      return false;
  }
}

/**
 * Normalizes operator aliases to canonical names.
 */
function normalizeOp(
  op: string
): 'eq' | 'neq' | 'gt' | 'gte' | 'lt' | 'lte' | 'contains' | 'not_contains' | 'starts_with' | 'ends_with' | 'in' | 'empty' | 'not_empty' {
  switch (op) {
    case 'ne':
      return 'neq';
    case 'is_empty':
      return 'empty';
    case 'is_not_empty':
      return 'not_empty';
    default:
      return op as ReturnType<typeof normalizeOp>;
  }
}

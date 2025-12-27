/**
 * Condition types for dynamic step/field visibility.
 *
 * Mirrors the Rust condition types from anyform/src/condition.rs
 */

import type { JsonValue } from './schema';

/**
 * Condition operators for comparing field values.
 */
export type ConditionOp =
  | 'eq'
  | 'neq'
  | 'ne' // alias for neq
  | 'gt'
  | 'gte'
  | 'lt'
  | 'lte'
  | 'contains'
  | 'not_contains'
  | 'starts_with'
  | 'ends_with'
  | 'in'
  | 'empty'
  | 'not_empty'
  | 'is_empty' // alias for empty
  | 'is_not_empty'; // alias for not_empty

/**
 * Simple comparison condition.
 *
 * @example
 * { field: 'country', op: 'eq', value: 'US' }
 */
export interface SimpleCondition {
  field: string;
  op: ConditionOp;
  value?: JsonValue;
}

/**
 * AND of multiple conditions.
 *
 * @example
 * { and: [
 *   { field: 'country', op: 'eq', value: 'US' },
 *   { field: 'age', op: 'gte', value: 18 }
 * ] }
 */
export interface AndCondition {
  and: ConditionRule[];
}

/**
 * OR of multiple conditions.
 *
 * @example
 * { or: [
 *   { field: 'plan', op: 'eq', value: 'enterprise' },
 *   { field: 'referral_code', op: 'not_empty' }
 * ] }
 */
export interface OrCondition {
  or: ConditionRule[];
}

/**
 * A condition rule that determines visibility of steps or fields.
 *
 * Can be a simple comparison, AND, or OR with arbitrary nesting.
 */
export type ConditionRule = SimpleCondition | AndCondition | OrCondition;

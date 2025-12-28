/**
 * Engine factory for form state management.
 *
 * Supports two engines:
 * - 'js': Pure TypeScript engine from @anyform/core (default, synchronous)
 * - 'wasm': WebAssembly engine from @anyform/wasm-js (lazy-loaded)
 */

import { FormState as JsFormState } from '@anyform/core';
import type { FormJson, EngineType } from '../types';

/**
 * Common interface for form engines.
 *
 * Both JS and WASM engines implement these methods.
 */
export interface IFormEngine {
  // Value management
  set_value(field: string, value: unknown): void;
  get_value(field: string): unknown;
  get_values(): Record<string, unknown>;

  // Touch tracking
  mark_touched(field: string): void;
  is_touched(field: string): boolean;

  // Validation
  validate_field(fieldName: string): string[];
  validate_step(stepId: string): Record<string, string[]>;
  validate_all(): Record<string, string[]>;
  is_valid(): boolean;
  get_errors(field: string): string[];
  get_all_errors(): Record<string, string[]>;

  // Visibility
  is_field_visible(fieldName: string): boolean;
  is_step_visible(stepId: string): boolean;
  visible_fields(stepId?: string): unknown[];
  visible_steps(): unknown[];

  // Multi-step navigation
  current_step(): unknown;
  current_step_index(): number;
  next_step(): boolean;
  prev_step(): boolean;
  go_to_step(stepId: string): boolean;
  can_go_next(): boolean;
  can_go_prev(): boolean;
  progress(): number[];
  is_last_step(): boolean;

  // Schema access
  slug(): string;
  name(): string;
  schema(): FormJson;
  action_url(): string;
  action_method(): string;
}

// WASM module cache
let wasmModule: typeof import('@anyform/wasm-js') | null = null;
let wasmInitPromise: Promise<typeof import('@anyform/wasm-js')> | null = null;

/**
 * Lazy-loads the WASM module.
 */
async function loadWasmModule(): Promise<typeof import('@anyform/wasm-js')> {
  if (wasmModule) return wasmModule;

  if (!wasmInitPromise) {
    wasmInitPromise = (async () => {
      const mod = await import('@anyform/wasm-js');
      await mod.default();
      wasmModule = mod;
      return mod;
    })();
  }

  return wasmInitPromise;
}

/**
 * Creates a form engine based on the specified type.
 *
 * @param schema - The form schema
 * @param type - Engine type: 'js' (default) or 'wasm'
 * @returns A promise that resolves to a FormEngine
 */
export async function createEngine(
  schema: FormJson,
  type: EngineType = 'js'
): Promise<IFormEngine> {
  if (type === 'wasm') {
    try {
      const wasm = await loadWasmModule();
      return new wasm.FormState(schema) as unknown as IFormEngine;
    } catch (e) {
      console.warn('WASM engine failed to load, falling back to JS:', e);
      // Fall back to JS engine
      return new JsFormState(schema as never) as unknown as IFormEngine;
    }
  }

  // Default: JS engine (synchronous)
  return new JsFormState(schema as never) as unknown as IFormEngine;
}

/**
 * Creates a JS form engine synchronously.
 *
 * Use this when you know you want the JS engine and don't need async.
 */
export function createJsEngine(schema: FormJson): IFormEngine {
  return new JsFormState(schema as never) as unknown as IFormEngine;
}

/**
 * Checks if the WASM module is already loaded.
 */
export function isWasmLoaded(): boolean {
  return wasmModule !== null;
}

// Re-export for convenience
export { JsFormState };
export type { EngineType };

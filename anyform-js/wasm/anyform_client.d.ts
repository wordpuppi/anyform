/* tslint:disable */
/* eslint-disable */

export class FormClient {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Fetches a form and returns a FormState for managing it.
   */
  fetch_form(slug: string): Promise<FormState>;
  /**
   * Submits form data directly.
   */
  submit_form(slug: string, data: any): Promise<any>;
  /**
   * Creates a new FormClient with the given base URL.
   */
  constructor(base_url: string);
  /**
   * Returns the base URL.
   */
  base_url(): string;
}

export class FormState {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Gets errors for a specific field.
   */
  get_errors(field: string): string[];
  /**
   * Gets all values as a JS object.
   */
  get_values(): any;
  /**
   * Navigates to a specific step by ID. Returns true if successful.
   */
  go_to_step(step_id: string): boolean;
  /**
   * Checks if a field has been touched.
   */
  is_touched(field: string): boolean;
  /**
   * Returns true if we can advance to the next step.
   */
  can_go_next(): boolean;
  /**
   * Returns true if we can go back to the previous step.
   */
  can_go_prev(): boolean;
  /**
   * Returns the current step.
   */
  current_step(): any;
  /**
   * Returns true if currently on the last step.
   */
  is_last_step(): boolean;
  /**
   * Marks a field as touched (user has interacted with it).
   */
  mark_touched(field: string): void;
  /**
   * Validates all visible fields in the form.
   */
  validate_all(): any;
  /**
   * Validates all fields in a step.
   */
  validate_step(step_id: string): any;
  /**
   * Returns all visible steps.
   */
  visible_steps(): any;
  /**
   * Gets all errors as a JS object.
   */
  get_all_errors(): any;
  /**
   * Validates a specific field and returns errors.
   */
  validate_field(field_name: string): string[];
  /**
   * Returns visible fields, optionally filtered by step.
   */
  visible_fields(step_id?: string | null): any;
  /**
   * Checks if a step is visible.
   */
  is_step_visible(step_id: string): boolean;
  /**
   * Checks if a field is visible.
   */
  is_field_visible(field_name: string): boolean;
  /**
   * Returns the current step index (0-based, among visible steps).
   */
  current_step_index(): number;
  /**
   * Creates a new FormState from a form schema.
   */
  constructor(schema_js: any);
  /**
   * Returns the form name.
   */
  name(): string;
  /**
   * Returns the form slug.
   */
  slug(): string;
  /**
   * Returns the form schema as JS.
   */
  schema(): any;
  /**
   * Returns true if the form has no validation errors.
   */
  is_valid(): boolean;
  /**
   * Returns progress as [current, total] (1-indexed for display).
   */
  progress(): Uint32Array;
  /**
   * Gets a field value.
   */
  get_value(field: string): any;
  /**
   * Moves to the next visible step. Returns true if successful.
   */
  next_step(): boolean;
  /**
   * Moves to the previous visible step. Returns true if successful.
   */
  prev_step(): boolean;
  /**
   * Sets a field value.
   */
  set_value(field: string, value: any): void;
}

/**
 * Hydrates a specific form by slug.
 */
export function hydrate(slug: string): void;

/**
 * Hydrates all forms on the page with `data-af-form` attribute.
 */
export function hydrate_all(): void;

/**
 * Initialize the WASM module.
 *
 * This is called automatically when the module is loaded.
 * Note: This does NOT auto-hydrate forms. Users should call hydrate_all()
 * after DOMContentLoaded to hydrate server-rendered forms.
 */
export function init(): void;

/**
 * Returns the version of anyform-client.
 */
export function version(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_formstate_free: (a: number, b: number) => void;
  readonly formstate_can_go_next: (a: number) => number;
  readonly formstate_can_go_prev: (a: number) => number;
  readonly formstate_current_step: (a: number) => any;
  readonly formstate_current_step_index: (a: number) => number;
  readonly formstate_get_all_errors: (a: number) => any;
  readonly formstate_get_errors: (a: number, b: number, c: number) => [number, number];
  readonly formstate_get_value: (a: number, b: number, c: number) => any;
  readonly formstate_get_values: (a: number) => any;
  readonly formstate_go_to_step: (a: number, b: number, c: number) => number;
  readonly formstate_is_field_visible: (a: number, b: number, c: number) => number;
  readonly formstate_is_last_step: (a: number) => number;
  readonly formstate_is_step_visible: (a: number, b: number, c: number) => number;
  readonly formstate_is_touched: (a: number, b: number, c: number) => number;
  readonly formstate_is_valid: (a: number) => number;
  readonly formstate_mark_touched: (a: number, b: number, c: number) => void;
  readonly formstate_name: (a: number) => [number, number];
  readonly formstate_new: (a: any) => [number, number, number];
  readonly formstate_next_step: (a: number) => number;
  readonly formstate_prev_step: (a: number) => number;
  readonly formstate_progress: (a: number) => [number, number];
  readonly formstate_schema: (a: number) => any;
  readonly formstate_set_value: (a: number, b: number, c: number, d: any) => void;
  readonly formstate_slug: (a: number) => [number, number];
  readonly formstate_validate_all: (a: number) => any;
  readonly formstate_validate_field: (a: number, b: number, c: number) => [number, number];
  readonly formstate_validate_step: (a: number, b: number, c: number) => any;
  readonly formstate_visible_fields: (a: number, b: number, c: number) => any;
  readonly formstate_visible_steps: (a: number) => any;
  readonly hydrate: (a: number, b: number) => void;
  readonly hydrate_all: () => void;
  readonly __wbg_formclient_free: (a: number, b: number) => void;
  readonly formclient_base_url: (a: number) => [number, number];
  readonly formclient_fetch_form: (a: number, b: number, c: number) => any;
  readonly formclient_new: (a: number, b: number) => number;
  readonly formclient_submit_form: (a: number, b: number, c: number, d: any) => any;
  readonly init: () => void;
  readonly version: () => [number, number];
  readonly wasm_bindgen__convert__closures_____invoke__h52f89e5a64ab2d27: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h1334e87dab7795ef: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h133f7f3e7844767d: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h6e98e1def914cdb3: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h41510e06c9979890: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;

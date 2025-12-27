/**
 * WASM initialization singleton
 *
 * Ensures the WASM module is initialized only once across the application.
 */

import init from 'anyform-js';

let wasmInitialized = false;
let wasmInitPromise: Promise<void> | null = null;

/**
 * Ensures the WASM module is initialized.
 * Safe to call multiple times - will only initialize once.
 */
export async function ensureWasmInit(): Promise<void> {
  if (wasmInitialized) {
    return;
  }

  if (!wasmInitPromise) {
    wasmInitPromise = init().then(() => {
      wasmInitialized = true;
    });
  }

  await wasmInitPromise;
}

/**
 * Checks if WASM is already initialized.
 */
export function isWasmInitialized(): boolean {
  return wasmInitialized;
}

/**
 * @wordpuppi/anyform-wasm-js - Browser client for anyform
 *
 * Provides form state management, validation, and multi-step navigation
 * powered by WebAssembly.
 *
 * Source: https://github.com/wordpuppi/anyform
 * License: MIT
 */

// Re-export all public WASM bindings
import init, {
  FormClient,
  FormState,
  hydrate,
  hydrate_all,
  version,
} from './wasm/anyform_client.js';

export { init, FormClient, FormState, hydrate, hydrate_all, version };
export default init;

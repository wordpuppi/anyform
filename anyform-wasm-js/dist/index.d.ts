/**
 * @wordpuppi/anyform-wasm-js - Browser client for anyform
 *
 * Provides form state management, validation, and multi-step navigation
 * powered by WebAssembly.
 *
 * @example
 * ```typescript
 * import init, { FormClient } from '@wordpuppi/anyform-wasm-js';
 *
 * await init();
 *
 * const client = new FormClient('http://localhost:3000');
 * const form = await client.fetch_form('contact');
 *
 * form.set_value('email', 'user@example.com');
 * if (form.is_valid()) {
 *   await form.submit();
 * }
 * ```
 */
import init, { FormClient, FormState, hydrate, hydrate_all, version } from '../wasm/anyform_client.js';
export { init, FormClient, FormState, hydrate, hydrate_all, version };
export default init;

//! # anyform-client
//!
//! WASM client library for anyform - browser-side form handling.
//!
//! ## Features
//!
//! - **FormClient**: Fetch forms and submit data via HTTP API
//! - **FormState**: Client-side form state management
//! - **Validation**: Mirror server-side validation rules in the browser
//! - **Conditions**: Evaluate step/field visibility conditions
//! - **Hydration**: Auto-enhance server-rendered HTML forms
//!
//! ## Quick Start (JavaScript)
//!
//! ```javascript
//! import init, { FormClient, hydrate_all } from 'anyform-client';
//!
//! async function main() {
//!     await init();
//!
//!     // Option 1: Hydrate server-rendered forms
//!     hydrate_all();
//!
//!     // Option 2: Fetch form and manage state manually
//!     const client = new FormClient('http://localhost:3000');
//!     const form = await client.fetch_form('contact');
//!
//!     form.set_value('email', 'user@example.com');
//!     form.mark_touched('email');
//!
//!     if (form.is_valid()) {
//!         const result = await client.submit_form('contact', form.get_values());
//!         console.log('Submitted:', result);
//!     }
//! }
//! ```
//!
//! ## Multi-Step Forms
//!
//! ```javascript
//! const form = await client.fetch_form('wizard');
//!
//! // Navigation
//! console.log('Current step:', form.current_step());
//! console.log('Progress:', form.progress()); // [1, 3]
//!
//! // Move through steps
//! if (form.can_go_next()) {
//!     form.next_step();
//! }
//!
//! // Validate current step
//! const errors = form.validate_step(form.current_step().id);
//! ```

pub mod api;
pub mod form_client;
pub mod form_state;
pub mod hydrate;
pub mod schema;
pub mod validation;

// Re-exports for wasm-bindgen
pub use form_client::FormClient;
pub use form_state::FormState;
pub use hydrate::{hydrate, hydrate_all};

use wasm_bindgen::prelude::*;

/// Initialize the WASM module.
///
/// This is called automatically when the module is loaded.
/// Note: This does NOT auto-hydrate forms. Users should call hydrate_all()
/// after DOMContentLoaded to hydrate server-rendered forms.
#[wasm_bindgen(start)]
pub fn init() {
    // Module initialization - currently a no-op.
    // Hydration happens when user calls hydrate_all() or hydrate(slug).
}

/// Returns the version of anyform-client.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

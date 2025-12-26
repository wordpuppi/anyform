//! High-level form client for anyform.
//!
//! FormClient provides a simple interface for fetching forms and managing
//! form state in the browser.

use crate::api;
use crate::form_state::FormState;
use wasm_bindgen::prelude::*;

/// High-level client for interacting with anyform API.
#[wasm_bindgen]
pub struct FormClient {
    base_url: String,
}

#[wasm_bindgen]
impl FormClient {
    /// Creates a new FormClient with the given base URL.
    #[wasm_bindgen(constructor)]
    pub fn new(base_url: &str) -> FormClient {
        FormClient {
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    /// Fetches a form and returns a FormState for managing it.
    pub async fn fetch_form(&self, slug: &str) -> Result<FormState, JsValue> {
        let schema = api::fetch_form(&self.base_url, slug)
            .await
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(FormState::from_schema(schema))
    }

    /// Submits form data directly.
    pub async fn submit_form(&self, slug: &str, data: JsValue) -> Result<JsValue, JsValue> {
        let json_data: serde_json::Value = serde_wasm_bindgen::from_value(data)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse data: {}", e)))?;

        let result = api::submit_form(&self.base_url, slug, &json_data)
            .await
            .map_err(|e| {
                serde_wasm_bindgen::to_value(&e).unwrap_or_else(|_| JsValue::from_str(&e.message))
            })?;

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
    }

    /// Returns the base URL.
    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }
}

impl FormClient {
    /// Submits a FormState's values.
    pub async fn submit_form_state(&self, form_state: &FormState) -> Result<JsValue, JsValue> {
        let data = serde_json::to_value(form_state.values_map())
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize: {}", e)))?;

        let result = api::submit_form(&self.base_url, &form_state.slug(), &data)
            .await
            .map_err(|e| {
                serde_wasm_bindgen::to_value(&e).unwrap_or_else(|_| JsValue::from_str(&e.message))
            })?;

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
    }
}

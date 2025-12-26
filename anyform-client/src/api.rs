//! HTTP API client for anyform.
//!
//! Provides fetch wrappers for communicating with the anyform server.

use crate::schema::FormJson;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// API response envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// API error details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Pagination information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    pub total_items: u32,
}

/// Submission response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ResultInfo>,
}

/// Quiz result information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultInfo {
    pub key: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Fetches a form schema from the API.
pub async fn fetch_form(base_url: &str, slug: &str) -> Result<FormJson, String> {
    let url = format!("{}/api/forms/{}/json", base_url.trim_end_matches('/'), slug);

    let response = fetch_json(&url).await?;
    let api_response: ApiResponse<FormJson> = serde_json::from_str(&response)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if api_response.success {
        api_response.data.ok_or_else(|| "No data in response".to_string())
    } else {
        let error_msg = api_response
            .error
            .map(|e| e.message)
            .unwrap_or_else(|| "Unknown error".to_string());
        Err(error_msg)
    }
}

/// Submits form data to the API.
pub async fn submit_form(
    base_url: &str,
    slug: &str,
    data: &serde_json::Value,
) -> Result<SubmissionResponse, ApiError> {
    let url = format!("{}/api/forms/{}", base_url.trim_end_matches('/'), slug);

    let response = post_json(&url, data).await.map_err(|e| ApiError {
        code: "NETWORK_ERROR".to_string(),
        message: e,
        details: None,
    })?;

    let api_response: ApiResponse<SubmissionResponse> =
        serde_json::from_str(&response).map_err(|e| ApiError {
            code: "PARSE_ERROR".to_string(),
            message: format!("Failed to parse response: {}", e),
            details: None,
        })?;

    if api_response.success {
        api_response.data.ok_or_else(|| ApiError {
            code: "NO_DATA".to_string(),
            message: "No data in response".to_string(),
            details: None,
        })
    } else {
        Err(api_response.error.unwrap_or_else(|| ApiError {
            code: "UNKNOWN_ERROR".to_string(),
            message: "Unknown error".to_string(),
            details: None,
        }))
    }
}

/// Performs a GET request and returns the response text.
async fn fetch_json(url: &str) -> Result<String, String> {
    let window = web_sys::window().ok_or("No window available")?;

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;

    request
        .headers()
        .set("Accept", "application/json")
        .map_err(|e| format!("Failed to set header: {:?}", e))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch failed: {:?}", e))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| "Response is not a Response object")?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let text = JsFuture::from(resp.text().map_err(|e| format!("Failed to get text: {:?}", e))?)
        .await
        .map_err(|e| format!("Failed to read response: {:?}", e))?;

    text.as_string().ok_or_else(|| "Response is not a string".to_string())
}

/// Performs a POST request with JSON body.
async fn post_json(url: &str, data: &serde_json::Value) -> Result<String, String> {
    let window = web_sys::window().ok_or("No window available")?;

    let body = serde_json::to_string(data).map_err(|e| format!("Failed to serialize: {}", e))?;

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&body));

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;

    request
        .headers()
        .set("Content-Type", "application/json")
        .map_err(|e| format!("Failed to set header: {:?}", e))?;

    request
        .headers()
        .set("Accept", "application/json")
        .map_err(|e| format!("Failed to set header: {:?}", e))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch failed: {:?}", e))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| "Response is not a Response object")?;

    let text = JsFuture::from(resp.text().map_err(|e| format!("Failed to get text: {:?}", e))?)
        .await
        .map_err(|e| format!("Failed to read response: {:?}", e))?;

    text.as_string().ok_or_else(|| "Response is not a string".to_string())
}

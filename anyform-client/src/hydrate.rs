//! Form hydration for server-rendered HTML.
//!
//! This module enables automatic hydration of server-rendered forms,
//! adding client-side interactivity (validation, step navigation, conditions).

use crate::form_state::FormState;
use crate::schema::{FieldJson, FormJson, StepJson, ValidationRules, ValueType};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, Event, HtmlFormElement, HtmlInputElement};

/// Hydrates all forms on the page with `data-af-form` attribute.
#[wasm_bindgen]
pub fn hydrate_all() {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => {
            console_log("No document available");
            return;
        }
    };

    let forms = match document.query_selector_all("[data-af-form]") {
        Ok(f) => f,
        Err(_) => {
            console_log("Failed to query forms");
            return;
        }
    };

    for i in 0..forms.length() {
        if let Some(node) = forms.get(i) {
            if let Ok(form_element) = node.dyn_into::<Element>() {
                if let Some(slug) = form_element.get_attribute("data-af-form") {
                    console_log(&format!("Hydrating form: {}", slug));
                    if let Ok(form) = form_element.dyn_into::<HtmlFormElement>() {
                        hydrate_form_element(&document, form, &slug);
                    }
                }
            }
        }
    }
}

/// Hydrates a specific form by slug.
#[wasm_bindgen]
pub fn hydrate(slug: &str) {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => {
            console_log("No document available");
            return;
        }
    };

    let selector = format!("[data-af-form=\"{}\"]", slug);
    if let Ok(Some(element)) = document.query_selector(&selector) {
        console_log(&format!("Hydrating form: {}", slug));
        if let Ok(form) = element.dyn_into::<HtmlFormElement>() {
            hydrate_form_element(&document, form, slug);
        }
    } else {
        console_log(&format!("Form not found: {}", slug));
    }
}

/// Hydrates a single form element.
fn hydrate_form_element(_document: &Document, form: HtmlFormElement, slug: &str) {
    // Parse form schema from data attributes
    let schema = match parse_form_schema(&form, slug) {
        Some(s) => s,
        None => {
            console_log("Failed to parse form schema from attributes");
            return;
        }
    };

    // Create form state
    let state = FormState::from_schema(schema);
    let state = Rc::new(RefCell::new(state));

    // Bind input events
    bind_input_events(&form, state.clone());

    // Bind navigation events
    bind_navigation_events(&form, state.clone());

    // Bind form submission
    bind_submit_event(&form, state.clone());

    // Initial visibility update
    update_visibility(&form, &state.borrow());

    console_log(&format!("Form hydrated successfully: {}", slug));
}

/// Parses form schema from HTML data attributes.
fn parse_form_schema(form: &HtmlFormElement, slug: &str) -> Option<FormJson> {
    let form_element: &Element = form.as_ref();
    let mut steps = Vec::new();

    // Find all steps
    if let Ok(step_elements) = form_element.query_selector_all(".af-step") {
        for step_idx in 0..step_elements.length() {
            if let Some(node) = step_elements.get(step_idx) {
                if let Ok(step_el) = node.dyn_into::<Element>() {
                    // Parse step condition
                    let condition = step_el
                        .get_attribute("data-af-condition")
                        .and_then(|c| serde_json::from_str(&c).ok());

                    let mut fields = Vec::new();

                    // Find fields in this step
                    if let Ok(field_elements) = step_el.query_selector_all(".af-field") {
                        for field_idx in 0..field_elements.length() {
                            if let Some(field_node) = field_elements.get(field_idx) {
                                if let Ok(field_el) = field_node.dyn_into::<Element>() {
                                    if let Some(field) = parse_field(&field_el, field_idx as i32) {
                                        fields.push(field);
                                    }
                                }
                            }
                        }
                    }

                    steps.push(StepJson {
                        id: Uuid::new_v4(),
                        name: format!("Step {}", step_idx + 1),
                        description: None,
                        order: step_idx as i32,
                        condition,
                        fields,
                    });
                }
            }
        }
    }

    // If no steps found, create a single step with all fields
    if steps.is_empty() {
        if let Ok(field_elements) = form_element.query_selector_all(".af-field") {
            let mut fields = Vec::new();

            for field_idx in 0..field_elements.length() {
                if let Some(field_node) = field_elements.get(field_idx) {
                    if let Ok(field_el) = field_node.dyn_into::<Element>() {
                        if let Some(field) = parse_field(&field_el, field_idx as i32) {
                            fields.push(field);
                        }
                    }
                }
            }

            steps.push(StepJson {
                id: Uuid::new_v4(),
                name: "Main".to_string(),
                description: None,
                order: 0,
                condition: None,
                fields,
            });
        }
    }

    Some(FormJson {
        id: Uuid::new_v4(),
        name: slug.to_string(),
        slug: slug.to_string(),
        description: None,
        settings: Default::default(),
        steps,
    })
}

/// Parses a field from an element.
fn parse_field(field_el: &Element, order: i32) -> Option<FieldJson> {
    let field_name = field_el.get_attribute("data-af-field")?;

    // Parse validation rules
    let validation: ValidationRules = field_el
        .get_attribute("data-af-validation")
        .and_then(|v| serde_json::from_str(&v).ok())
        .unwrap_or_default();

    // Parse field condition
    let field_condition = field_el
        .get_attribute("data-af-condition")
        .and_then(|c| serde_json::from_str(&c).ok());

    // Get label from label element
    let label = field_el
        .query_selector("label")
        .ok()
        .flatten()
        .and_then(|l| l.text_content())
        .unwrap_or_else(|| field_name.clone());

    // Determine field type from input type
    let field_type = determine_field_type(field_el);

    Some(FieldJson {
        id: Uuid::new_v4(),
        name: field_name,
        label,
        field_type,
        placeholder: None,
        help_text: None,
        default_value: None,
        validation,
        condition: field_condition,
        options: vec![],
        order,
    })
}

/// Determines field type from input element.
fn determine_field_type(field_el: &Element) -> ValueType {
    if let Ok(Some(input)) = field_el.query_selector("input") {
        let input_type = input.get_attribute("type").unwrap_or_default();
        match input_type.as_str() {
            "email" => ValueType::Email,
            "url" => ValueType::Url,
            "tel" => ValueType::Tel,
            "number" => ValueType::Number,
            "date" => ValueType::Date,
            "time" => ValueType::Time,
            "datetime-local" => ValueType::Datetime,
            "checkbox" => ValueType::Checkbox,
            "radio" => ValueType::Radio,
            "file" => ValueType::File,
            "hidden" => ValueType::Hidden,
            "password" => ValueType::Password,
            "color" => ValueType::Color,
            "range" => ValueType::Range,
            _ => ValueType::Text,
        }
    } else if field_el.query_selector("textarea").ok().flatten().is_some() {
        ValueType::Textarea
    } else if field_el.query_selector("select").ok().flatten().is_some() {
        ValueType::Select
    } else {
        ValueType::Text
    }
}

/// Binds input change events.
fn bind_input_events(form: &HtmlFormElement, state: Rc<RefCell<FormState>>) {
    let form_element: &Element = form.as_ref();
    let field_elements = match form_element.query_selector_all(".af-field") {
        Ok(els) => els,
        Err(_) => return,
    };

    for i in 0..field_elements.length() {
        let Some(node) = field_elements.get(i) else {
            continue;
        };
        let Ok(field_el) = node.dyn_into::<Element>() else {
            continue;
        };
        let Some(field_name) = field_el.get_attribute("data-af-field") else {
            continue;
        };

        // Find the actual input element
        let input = field_el
            .query_selector("input, textarea, select")
            .ok()
            .flatten();

        if let Some(input) = input {
            let state_clone = state.clone();
            let field_name_clone = field_name.clone();
            let form_clone = form.clone();

            let closure = Closure::wrap(Box::new(move |event: Event| {
                if let Some(target) = event.target() {
                    let value = get_input_value(&target.unchecked_into());
                    let mut state = state_clone.borrow_mut();

                    state.set_value(
                        &field_name_clone,
                        serde_wasm_bindgen::to_value(&value).unwrap_or(JsValue::NULL),
                    );
                    state.mark_touched(&field_name_clone);

                    // Update visibility after value change
                    update_visibility(&form_clone, &state);

                    // Update error display
                    let errors = state.get_errors(&field_name_clone);
                    update_field_errors(&form_clone, &field_name_clone, &errors);
                }
            }) as Box<dyn FnMut(Event)>);

            let _ = input.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref());
            let _ = input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());

            // Prevent closure from being dropped
            closure.forget();
        }
    }
}

/// Gets the value from an input element.
fn get_input_value(input: &Element) -> serde_json::Value {
    if let Ok(input_el) = input.clone().dyn_into::<HtmlInputElement>() {
        let input_type = input_el.type_();
        match input_type.as_str() {
            "checkbox" => serde_json::Value::Bool(input_el.checked()),
            "number" | "range" => {
                let value = input_el.value();
                if value.is_empty() {
                    serde_json::Value::Null
                } else {
                    value
                        .parse::<f64>()
                        .map(|n| serde_json::json!(n))
                        .unwrap_or(serde_json::Value::String(value))
                }
            }
            _ => serde_json::Value::String(input_el.value()),
        }
    } else if let Ok(textarea) = input.clone().dyn_into::<web_sys::HtmlTextAreaElement>() {
        serde_json::Value::String(textarea.value())
    } else if let Ok(select) = input.clone().dyn_into::<web_sys::HtmlSelectElement>() {
        serde_json::Value::String(select.value())
    } else {
        serde_json::Value::Null
    }
}

/// Binds navigation button events.
fn bind_navigation_events(form: &HtmlFormElement, state: Rc<RefCell<FormState>>) {
    let form_element: &Element = form.as_ref();

    // Prev button
    if let Ok(Some(prev_btn)) = form_element.query_selector(".af-prev") {
        let state_clone = state.clone();
        let form_clone = form.clone();

        let closure = Closure::wrap(Box::new(move |_: Event| {
            let mut state = state_clone.borrow_mut();
            if state.prev_step() {
                update_step_visibility(&form_clone, &state);
                update_navigation_buttons(&form_clone, &state);
            }
        }) as Box<dyn FnMut(Event)>);

        let _ = prev_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }

    // Next button
    if let Ok(Some(next_btn)) = form_element.query_selector(".af-next") {
        let state_clone = state.clone();
        let form_clone = form.clone();

        let closure = Closure::wrap(Box::new(move |_: Event| {
            let mut state = state_clone.borrow_mut();

            if state.next_step() {
                update_step_visibility(&form_clone, &state);
                update_navigation_buttons(&form_clone, &state);
            }
        }) as Box<dyn FnMut(Event)>);

        let _ = next_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }

    // Initial button state
    update_navigation_buttons(form, &state.borrow());
}

/// Binds form submission event.
fn bind_submit_event(form: &HtmlFormElement, state: Rc<RefCell<FormState>>) {
    let form_clone = form.clone();
    let state_clone = state;

    let closure = Closure::wrap(Box::new(move |event: Event| {
        let mut state = state_clone.borrow_mut();

        // Validate all fields
        state.validate_all();

        if !state.is_valid() {
            event.prevent_default();

            // Show all errors
            show_all_errors(&form_clone, &state);

            console_log("Form validation failed");
        }
        // If valid, let the form submit normally
    }) as Box<dyn FnMut(Event)>);

    let _ = form.add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Updates visibility of steps and fields based on conditions.
fn update_visibility(form: &HtmlFormElement, state: &FormState) {
    let form_element: &Element = form.as_ref();

    // Update field visibility
    if let Ok(field_elements) = form_element.query_selector_all(".af-field") {
        for i in 0..field_elements.length() {
            if let Some(node) = field_elements.get(i) {
                if let Ok(field_el) = node.dyn_into::<Element>() {
                    if let Some(field_name) = field_el.get_attribute("data-af-field") {
                        let visible = state.is_field_visible(&field_name);
                        let _ = field_el.set_attribute(
                            "data-af-visible",
                            if visible { "true" } else { "false" },
                        );
                    }
                }
            }
        }
    }

    // Update step visibility
    update_step_visibility(form, state);
}

/// Updates step visibility for multi-step forms.
fn update_step_visibility(form: &HtmlFormElement, state: &FormState) {
    let form_element: &Element = form.as_ref();
    let current_index = state.current_step_index();

    if let Ok(step_elements) = form_element.query_selector_all(".af-step") {
        for i in 0..step_elements.length() {
            if let Some(node) = step_elements.get(i) {
                if let Ok(step_el) = node.dyn_into::<Element>() {
                    let should_show = i == current_index as u32;
                    let _ = step_el.set_attribute(
                        "data-af-visible",
                        if should_show { "true" } else { "false" },
                    );
                }
            }
        }
    }
}

/// Updates navigation button states.
fn update_navigation_buttons(form: &HtmlFormElement, state: &FormState) {
    let form_element: &Element = form.as_ref();

    // Update prev button
    if let Ok(Some(prev_btn)) = form_element.query_selector(".af-prev") {
        if let Ok(btn) = prev_btn.dyn_into::<web_sys::HtmlButtonElement>() {
            btn.set_disabled(!state.can_go_prev());
        }
    }

    // Update next button visibility
    if let Ok(Some(next_btn)) = form_element.query_selector(".af-next") {
        if let Ok(el) = next_btn.dyn_into::<web_sys::HtmlElement>() {
            let display = if state.is_last_step() { "none" } else { "" };
            let _ = el.style().set_property("display", display);
        }
    }

    // Update submit button visibility
    if let Ok(Some(submit_btn)) = form_element.query_selector(".af-submit") {
        if let Ok(el) = submit_btn.dyn_into::<web_sys::HtmlElement>() {
            let display = if state.is_last_step() { "" } else { "none" };
            let _ = el.style().set_property("display", display);
        }
    }
}

/// Updates error display for a field.
fn update_field_errors(form: &HtmlFormElement, field_name: &str, errors: &[String]) {
    let form_element: &Element = form.as_ref();
    let selector = format!(".af-field[data-af-field=\"{}\"]", field_name);

    if let Ok(Some(field_el)) = form_element.query_selector(&selector) {
        // Toggle error class
        let class_list = field_el.class_list();
        if errors.is_empty() {
            let _ = class_list.remove_1("af-error");
        } else {
            let _ = class_list.add_1("af-error");
        }

        // Update error message
        if let Ok(Some(error_el)) = field_el.query_selector(".af-error-message") {
            error_el.set_text_content(Some(&errors.join(", ")));
        }
    }
}

/// Shows all validation errors.
fn show_all_errors(form: &HtmlFormElement, state: &FormState) {
    let form_element: &Element = form.as_ref();

    if let Ok(field_elements) = form_element.query_selector_all(".af-field") {
        for i in 0..field_elements.length() {
            if let Some(node) = field_elements.get(i) {
                if let Ok(field_el) = node.dyn_into::<Element>() {
                    if let Some(field_name) = field_el.get_attribute("data-af-field") {
                        let errors = state.get_errors(&field_name);
                        update_field_errors(form, &field_name, &errors);
                    }
                }
            }
        }
    }
}

/// Logs to the browser console.
fn console_log(msg: &str) {
    web_sys::console::log_1(&JsValue::from_str(msg));
}

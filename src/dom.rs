// src/dom_helpers.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue};
use web_sys::{window, HtmlElement, Element};

pub fn document() -> Result<web_sys::Document, JsValue> {
    window()
        .ok_or(JsValue::from_str("No global `window` exists"))?
        .document()
        .ok_or(JsValue::from_str("Should have a document on window"))
}

pub fn elem(id: &str) -> Result<web_sys::HtmlElement, JsValue> {

    let doc = window()
        .ok_or(JsValue::from_str("No global `window` exists"))?
        .document()
        .ok_or(JsValue::from_str("Should have a document on window"))?;
    
    let el = doc.get_element_by_id(id)
        .ok_or(JsValue::from_str("element with id not found"))?
        .dyn_into::<HtmlElement>()?;

    Ok(el)
}

pub fn baseelem(id: &str) -> Result<Element, JsValue> {
    // For non-HTML elements or generic ops
    let document = window().unwrap().document().unwrap();
    document.get_element_by_id(id)
        .ok_or(JsValue::from_str("Element not found"))
        .map(|e| e.unchecked_into::<Element>())
}

// src/dom.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue};
use web_sys::{Element, HtmlElement, window};

// pub fn document() -> Result<web_sys::Document, JsValue> {
//     window()
//         .ok_or(JsValue::from_str("No global `window` exists"))?
//         .document()
//         .ok_or(JsValue::from_str("Should have a document on window"))
// }

// pub fn elem(id: &str) -> Result<HtmlElement, JsValue> {
//     let doc = window()
//         .ok_or(JsValue::from_str("No global `window` exists"))?
//         .document()
//         .ok_or(JsValue::from_str("Should have a document on window"))?;
    
//     let el = doc.get_element_by_id(id)
//         .ok_or(JsValue::from_str("element with id not found"))?
//         .dyn_into::<HtmlElement>()?;

//     Ok(el)
// }

pub fn get_element(id: &str) -> Result<Element, JsValue> {
    // Works with HTML, SVG, and other element types
    let doc = window()
        .ok_or(JsValue::from_str("No global `window` exists"))?
        .document()
        .ok_or(JsValue::from_str("Should have a document on window"))?;
    
    doc.get_element_by_id(id)
        .ok_or(JsValue::from_str("element with id not found"))
}

// pub fn baseelem(id: &str) -> Result<Element, JsValue> {
//     // Deprecated - use get_element() instead
//     get_element(id)
// }
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

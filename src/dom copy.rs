// src/dom.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue};
use web_sys::{Element, HtmlElement, window};

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

// SVG Creation Functions
pub fn create_svg(parent_id: &str, width: u32, height: u32) -> Result<Element, JsValue> {
    let doc = document()?;
    let svg = doc.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    
    svg.set_attribute("id", "rsdomlib-svg")?;
    svg.set_attribute("width", &width.to_string())?;
    svg.set_attribute("height", &height.to_string())?;
    svg.set_attribute("xmlns", "http://www.w3.org/2000/svg")?;
    
    let parent = elem(parent_id)?;
    let _ = parent.append_child(&svg);
    
    Ok(svg)
}

pub fn create_circle(cx: f32, cy: f32, r: f32) -> Result<Element, JsValue> {
    let doc = document()?;
    let circle = doc.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    
    circle.set_attribute("cx", &cx.to_string())?;
    circle.set_attribute("cy", &cy.to_string())?;
    circle.set_attribute("r", &r.to_string())?;
    
    Ok(circle)
}

pub fn create_rect(x: f32, y: f32, width: f32, height: f32) -> Result<Element, JsValue> {
    let doc = document()?;
    let rect = doc.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
    
    rect.set_attribute("x", &x.to_string())?;
    rect.set_attribute("y", &y.to_string())?;
    rect.set_attribute("width", &width.to_string())?;
    rect.set_attribute("height", &height.to_string())?;
    
    Ok(rect)
}

pub fn create_line(id: String, x1: f32, y1: f32, x2: f32, y2: f32) -> Result<Element, JsValue> {
    let doc = document()?;
    let line = doc.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?;
    
    line.set_attribute("id", &id)?;
    line.set_attribute("x1", &x1.to_string())?;
    line.set_attribute("y1", &y1.to_string())?;
    line.set_attribute("x2", &x2.to_string())?;
    line.set_attribute("y2", &y2.to_string())?;
    
    Ok(line)
}

pub fn create_path(d: &str) -> Result<Element, JsValue> {
    let doc = document()?;
    let path = doc.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    
    path.set_attribute("d", d)?;
    
    Ok(path)
}

pub fn create_text(x: f32, y: f32, text: &str) -> Result<Element, JsValue> {
    let doc = document()?;
    let text_elem = doc.create_element_ns(Some("http://www.w3.org/2000/svg"), "text")?;
    
    text_elem.set_attribute("x", &x.to_string())?;
    text_elem.set_attribute("y", &y.to_string())?;
    text_elem.set_text_content(Some(text));
    
    Ok(text_elem)
}

pub fn create_ellipse(cx: f32, cy: f32, rx: f32, ry: f32) -> Result<Element, JsValue> {
    let doc = document()?;
    let ellipse = doc.create_element_ns(Some("http://www.w3.org/2000/svg"), "ellipse")?;
    
    ellipse.set_attribute("cx", &cx.to_string())?;
    ellipse.set_attribute("cy", &cy.to_string())?;
    ellipse.set_attribute("rx", &rx.to_string())?;
    ellipse.set_attribute("ry", &ry.to_string())?;
    
    Ok(ellipse)
}

pub fn create_polygon(points: &str) -> Result<Element, JsValue> {
    let doc = document()?;
    let polygon = doc.create_element_ns(Some("http://www.w3.org/2000/svg"), "polygon")?;
    
    polygon.set_attribute("points", points)?;
    
    Ok(polygon)
}

pub fn set_svg_attribute(element: &Element, attr: &str, value: &str) -> Result<(), JsValue> {
    element.set_attribute(attr, value)
}

pub fn append_to_svg(svg: &Element, child: &Element) -> Result<(), JsValue> {
    let _ = svg.append_child(child);
    Ok(())
}
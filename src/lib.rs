use serde::de::value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use chrono::{Local, Timelike};
use chrono_tz::Tz;
use serde_wasm_bindgen::to_value;
use serde::Serialize;

use crate::dom::{elem, get_element};

mod dom;

#[wasm_bindgen]
pub fn hello(id: String) -> Result<(), JsValue> {
    // let el: HtmlElement = dom::elem(&id)?;
    dom::elem(&id)?.set_inner_text("Hello from Rust!");
    Ok(())
}

#[wasm_bindgen]
pub fn set_message(elem_id: String, message: String) -> Result<(), JsValue> {
    // let el: HtmlElement = dom::elem(&id)?;
    dom::elem(&elem_id)?.set_inner_text(&message);
    Ok(())
}

#[derive(Serialize)]
struct Time {
    hr: f32,
    mi: f32,
    se: f32
}

#[derive(Debug,Serialize)]
struct Point {
    x: f32,
    y: f32,
    val: i32,
    fontwt: i32,
    fontsize: i32,
    color: String
}

#[derive(Debug,Serialize)]
struct ClockData {
    hr_angle: f32,
    mi_angle: f32,
    se_angle: f32,
    datetime: String
}

#[wasm_bindgen]
pub fn get_time() -> JsValue {
    let now = Local::now();
    to_value(&Time {
        hr: now.hour() as f32,
        mi: now.minute() as f32,
        se: now.second() as f32,
    }).unwrap()
}
#[wasm_bindgen]
pub fn get_clock_marks(num_points: i32, radius: i32, step: i32, normal_color: String, hilite_color: String,
        small_font_wt: i32, big_font_wt: i32, small_font_size: i32, big_font_size: i32
    ) -> JsValue {
    let angle:f32 = 360.0 / num_points as f32;
    let mut marks: Vec<Point> = vec![];
    for i in 0..num_points {
        let step_angle = (i as f32 * angle) - 90.0;
        let x = radius as f32 * step_angle.to_radians().cos();
        let y = radius as f32 * step_angle.to_radians().sin();
        let mut fontwt = small_font_wt;
        let mut fontsize  = small_font_size; 
        let mut color = normal_color.clone();
        if i % step == 0 {
            fontwt = big_font_wt;
            fontsize = big_font_size;
            color = hilite_color.clone();
        }
        let val = i;
        marks.push(Point{x,y,val,fontsize,fontwt,color}); 
    }
    to_value(&marks).unwrap()
}

#[wasm_bindgen]
pub fn hr_angle() -> f32 {
    let now = Local::now();  // Fresh every call!
    let hr = now.hour() as f32;
    let mi = now.minute() as f32;
    (hr * 30.0 + mi / 2.0) - 90.0  // Your Swiss formula!
}

#[wasm_bindgen]
pub fn mi_angle() -> f32 {
    let now = Local::now();
    let mi = now.minute() as f32;
    let se = now.second() as f32;
    (mi * 6.0 + se / 10.0) - 90.0
}

#[wasm_bindgen]
pub fn se_angle() -> f32 {
    let now = Local::now();
    let se = now.second() as f32;
    (se * 6.0) - 90.0
}

#[wasm_bindgen]
pub fn full_datetime() -> String {
    let now = Local::now();
    now.format("%A, %B %d, %Y at %I:%M:%S %p %Z").to_string()
}

#[wasm_bindgen]
pub fn short_datetime() -> String {
    let now = Local::now();
    now.format("%H:%M:%S %Z").to_string()
}

#[wasm_bindgen]
pub fn set_elem_value(elem_id: String, value: String) -> Result<(), JsValue> {
    elem(&elem_id)?.set_inner_html(&value);
    Ok(())
}


#[wasm_bindgen]
pub fn set_zone_time(elem_id: String, zone_str: String) -> Result<(), JsValue> {
    if zone_str.is_empty() {
        return set_message(elem_id, "Please select a timezone".to_string());
    }
    
    match zone_str.parse::<Tz>() {
        Ok(tz) => {
            let zone_now = Local::now().with_timezone(&tz);
            let hr_angle = (zone_now.hour() as f32 * 30.0) + (zone_now.minute() as f32 / 2.0) - 90.0;
            let mi_angle = (zone_now.minute() as f32 * 6.0 + zone_now.second() as f32 / 10.0) - 90.0;
            let datetime = zone_now.format("%A %d %B %Y %I:%M:%S %p %Z").to_string();
            
            // Set the time display
            if let Err(e) = set_elem_value(elem_id.clone(), format!("{}: {}", zone_str, datetime)) {
                web_sys::console::log_1(&format!("Error setting value: {:?}", e).into());
                return Err(e);
            }
            
            // Set hour hand rotation
            match get_element("thr") {
                Ok(el) => {
                    if let Err(e) = el.set_attribute("transform", &format!("rotate({})", hr_angle)) {
                        web_sys::console::log_1(&format!("Error setting thr transform: {:?}", e).into());
                    }
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Element thr not found: {:?}", e).into());
                }
            }
            
            // Set minute hand rotation
            match elem("tmi") {
                Ok(el) => {
                    if let Err(e) = el.set_attribute("transform", &format!("rotate({})", mi_angle)) {
                        web_sys::console::log_1(&format!("Error setting tmi transform: {:?}", e).into());
                    }
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Element tmi not found: {:?}", e).into());
                }
            }
        }
        Err(_) => {
            let _ = set_message(elem_id, "Invalid timezone".to_string());
        }
    }

    Ok(())
}

#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate snowhash;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct CanvasPoint(f64, f64);

#[wasm_bindgen]
pub fn generate(hash: &str) -> String {
    let points: Vec<CanvasPoint> = snowhash::generate(hash)
        .iter()
        .map(|point| hex_to_cartesian(point.x() as f64, point.y() as f64))
        .map(|(x, y)| CanvasPoint(x, y))
        .collect();
    serde_json::to_string(&points).unwrap()
}

fn hex_to_cartesian(hex_x: f64, hex_y: f64) -> (f64, f64) {
    (
        3f64.sqrt() * hex_x + 3f64.sqrt() / 2.0 * hex_y,
        3.0 / 2.0 * hex_y,
    )
}

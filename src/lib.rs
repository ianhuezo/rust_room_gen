mod structures;

use structures::globals::Position;
use structures::rooms::Room;
use structures::universe::Universe;

extern crate web_sys;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[wasm_bindgen]
pub fn universe() -> Result<JsValue, JsValue> {
    let mut universe = Universe::new(64);
    universe.generate_rooms(10);
    Ok(serde_wasm_bindgen::to_value(&universe)?)
}

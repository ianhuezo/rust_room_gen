mod structures;

use structures::globals::Position;
use structures::rooms::Room;
use structures::universe::Universe;

extern crate web_sys;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn send_example_to_js() -> JsValue {
    // let mut universe = Universe::new(64);
    // universe.generate_rooms(12);
    let mut universe = Universe::new(64);
    universe.generate_rooms(5);
    JsValue::from_serde(&universe.universe_size).unwrap()
}

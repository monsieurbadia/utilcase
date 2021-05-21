use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_group(c: char) -> bool {
  utilcase::charcase::groupcase::is_group(c)
}

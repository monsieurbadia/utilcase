use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_lowercase(c: char) -> bool {
  utilcase::charcase::lowercase::is_lowercase(c)
}

#[wasm_bindgen]
pub fn to_lowercase(c: char) -> char {
  utilcase::charcase::lowercase::to_lowercase(c)
}

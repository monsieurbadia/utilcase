use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_uppercase(c: char) -> bool {
  utilcase::charcase::uppercase::is_uppercase(c)
}

#[wasm_bindgen]
pub fn to_uppercase(c: char) -> char {
  utilcase::charcase::uppercase::to_uppercase(c)
}

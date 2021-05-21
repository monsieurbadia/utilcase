use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_underscore(c: char) -> bool {
  utilcase::charcase::underscorecase::is_underscore(c)
}

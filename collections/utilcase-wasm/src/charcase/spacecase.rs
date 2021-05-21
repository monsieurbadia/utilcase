use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_whitespace(c: char) -> bool {
  utilcase::charcase::spacecase::is_whitespace(c)
}

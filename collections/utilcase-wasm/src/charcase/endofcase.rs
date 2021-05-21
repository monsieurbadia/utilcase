use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_end_of_file(c: char) -> bool {
  utilcase::charcase::endofcase::is_end_of_file(c)
}

#[wasm_bindgen]
pub fn is_end_of_line(c: char) -> bool {
  utilcase::charcase::endofcase::is_end_of_line(c)
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_number(c: char) -> bool {
  utilcase::charcase::is_numbercase::is_number(c)
}

#[wasm_bindgen]
pub fn is_number_zero(c: char) -> bool {
  utilcase::charcase::is_numbercase::is_number_zero(c)
}

#[wasm_bindgen]
pub fn is_number_continue(c: char) -> bool {
  utilcase::charcase::is_numbercase::is_number_continue(c)
}

#[wasm_bindgen]
pub fn is_number_hex(c: char) -> bool {
  utilcase::charcase::is_numbercase::is_number_hex(c)
}

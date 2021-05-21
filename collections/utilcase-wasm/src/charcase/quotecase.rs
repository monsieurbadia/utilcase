use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_quote(c: char) -> bool {
  utilcase::charcase::quotecase::is_quote(c)
}

#[wasm_bindgen]
pub fn is_quote_single(c: char) -> bool {
  utilcase::charcase::quotecase::is_quote_single(c)
}

#[wasm_bindgen]
pub fn is_quote_double(c: char) -> bool {
  utilcase::charcase::quotecase::is_quote_double(c)
}

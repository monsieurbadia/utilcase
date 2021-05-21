use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_ident(c: char) -> bool {
  utilcase::charcase::identcase::is_ident(c)
}

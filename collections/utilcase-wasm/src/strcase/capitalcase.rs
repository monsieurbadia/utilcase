use unitest::testing::{must, test, unit};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_capitalcase(s: &str) -> bool {
  utilcase::strcase::capitalcase::is_capitalcase(s)
}

#[wasm_bindgen]
pub fn to_capitalcase(s: &str) -> String {
  utilcase::strcase::capitalcase::is_capitalcase(s)
}

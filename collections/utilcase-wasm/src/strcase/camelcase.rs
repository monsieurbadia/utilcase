use unitest::testing::{must, test, unit};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_camelcase(s: &str) -> bool {
  utilcase::strcase::camelcase::is_camelcase(s)
}

#[wasm_bindgen]
pub fn to_camelcase(s: &str) -> String {
  utilcase::strcase::camelcase::to_camelcase(s)
}

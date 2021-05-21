use unitest::testing::{must, test, unit};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_lowercase(s: &str) -> bool {
  utilcase::strcase::lowercase::is_lowercase(s)
}

#[wasm_bindgen]
pub fn to_lowercase(s: &str) -> String {
  utilcase::strcase::lowercase::to_lowercase(s)
}

#[wasm_bindgen]
pub fn to_lowercase_first(s: &str) -> String {
  utilcase::strcase::lowercase::to_lowercase_first(s)
}

use unitest::testing::{must, test, unit};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_uppercase(s: &str) -> bool {
  utilcase::strcase::uppercase::is_uppercase(s)
}

#[wasm_bindgen]
pub fn to_uppercase(s: &str) -> String {
  utilcase::strcase::uppercase::to_uppercase(s)
}

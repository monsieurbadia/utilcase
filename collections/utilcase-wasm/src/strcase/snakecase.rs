use unitest::testing::{must, test, unit};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_snakecase(s: &str) -> bool {
  utilcase::strcase::snakecase::is_snakecase(s)
}

#[wasm_bindgen]
pub fn to_snakecase(s: &str) -> String {
  utilcase::strcase::snakecase::to_snakecase(s)
}

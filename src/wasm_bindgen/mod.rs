use wasm_bindgen::prelude::*;

use super::core::{get_features,get_keys};

#[wasm_bindgen]
pub fn wasm_bindgen_get_features(game_history: &str) -> Vec<u8> {
    get_features(game_history)
}

#[wasm_bindgen]
pub fn wasm_bindgen_get_keys() -> String {
    get_keys()
}
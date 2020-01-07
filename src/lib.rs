use wasm_bindgen::prelude::*;

mod app;
mod umlaut_replacer;

use app::Model;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<Model>();
    Ok(())
}
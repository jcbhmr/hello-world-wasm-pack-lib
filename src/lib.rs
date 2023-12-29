use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen(start)]
fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn greet(name: String) {
    let message = format!("Hello {}!", name);
    console::log_1(&JsValue::from(message));
}

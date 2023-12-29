use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen(start)]
fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn greet(name: String) {
    console::log_1(&format!("Hello {}!", name).into());
}

#[wasm_bindgen]
pub struct Calculator {
    value: f64,
}
#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        return Calculator { value: 0.0 };
    }

    pub fn add(&mut self, n: f64) {
        self.value += n;
    }

    pub fn sub(&mut self, n: f64) {
        self.value -= n;
    }

    pub fn mult(&mut self, n: f64) {
        self.value *= n;
    }

    pub fn div(&mut self, n: f64) {
        self.value /= n;
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> f64 {
        return self.value;
    }

    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, n: f64) {
        self.value = n;
    }
}

#[wasm_bindgen(inspectable)]
pub struct Report {
    pub dog_count: u32,
    pub bouncy_castles: u32,
    pub fun_percent: f64,
}

#[wasm_bindgen]
pub fn get_report() -> Report {
    return Report {
        dog_count: 5,
        bouncy_castles: 3,
        fun_percent: 76.8,
    };
}

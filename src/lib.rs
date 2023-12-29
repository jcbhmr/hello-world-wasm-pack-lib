use wasm_bindgen::prelude::*;

mod wasm_pack_better_panic {
    use std::panic;
    use std::sync::Once;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/js/wasm-pack-better-panic.js")]
    extern "C" {
        #[wasm_bindgen]
        fn hook(a: String);
    }

    #[inline]
    pub fn set_hook_once() {
        static SET_HOOK: Once = Once::new();
        SET_HOOK.call_once(|| {
            panic::set_hook(Box::new(|info| hook(info.to_string())));
        });
    }
}

#[wasm_bindgen(start)]
fn start() {
    wasm_pack_better_panic::set_hook_once();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-world-wasm-pack-lib!");
    todo!("Add more code here.");
}

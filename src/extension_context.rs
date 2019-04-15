use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = vscode)]
extern "C" {
    pub type ExtensionContext;

    #[wasm_bindgen(method, getter)]
    pub fn subscriptions(this: &ExtensionContext) -> Array;
}

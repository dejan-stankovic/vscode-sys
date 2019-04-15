use js_sys::{JsString, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = vscode)]
extern "C" {
    pub type Window;

    pub static window: Window;

    #[wasm_bindgen(method, js_name = "showInformationMessage")]
    pub fn show_information_message(this: &Window, msg: JsString) -> Promise;
}

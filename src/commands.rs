use crate::disposable::Disposable;
use js_sys::{Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = vscode)]
extern "C" {
    pub type Commands;

    pub static commands: Commands;

    #[wasm_bindgen(method, js_name = "registerCommand")]
    #[must_use]
    pub fn register_command(this: &Commands, cmd: JsString, cb: &Function) -> Disposable;
}

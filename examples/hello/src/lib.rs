use js_sys::{Array, Promise};
use vscode_sys;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
pub fn activate(context: &vscode_sys::ExtensionContext) -> Promise {
    console_error_panic_hook::set_once();
    Promise::new(&mut |resolve, _reject| {
        let tag = "extension.helloWASM".into();
        let clo = Closure::wrap(Box::new(|| {
            vscode_sys::window.show_information_message("Hello from Rust!".into());
        }) as Box<dyn FnMut()>);
        let disposable = vscode_sys::commands.register_command(tag, clo.as_ref().unchecked_ref());
        clo.forget(); // NOTE: leak the closure here so that it remains active
        context.subscriptions().push(disposable.as_ref());
        resolve.apply(&JsValue::null(), &Array::new()).unwrap();
    })
}

#[wasm_bindgen]
pub fn deactivate() {
    // empty
}

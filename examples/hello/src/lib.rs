#![feature(async_await)]
#![feature(await_macro)]
#![feature(futures_api)]
#![feature(impl_trait_in_bindings)]

use futures::{prelude::*, FutureExt};
use futures_util::compat::Compat;
use js_sys::{Array, Promise};
use vscode_sys::ExtensionContext;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::future_to_promise;

#[allow(non_snake_case)]
async fn register_command_helloWASM(context: ExtensionContext) -> Result<JsValue, JsValue> {
    use vscode_sys::{commands, window};
    // invoked when the command "extension.helloWASM" is invoked
    let clo = Closure::<Fn()>::new(|| {
        // invoked when the "show_information_message" API call (below) finishes
        let clo = Closure::<FnMut(JsValue)>::new(|res| {
            let args = Array::new();
            args.push(&"should be `undefined`:".into());
            args.push(&res);
            web_sys::console::log(&args);
        });
        window
            .show_information_message("Hello from Rust!".into())
            .then(&clo);
        clo.forget();
    });
    context.subscriptions().push({
        let tag = "extension.helloWASM".into();
        let fun = clo.as_ref().unchecked_ref();
        &commands.register_command(tag, fun)
    });
    clo.forget();
    Ok(JsValue::undefined())
}

async fn activate_future(context: ExtensionContext) -> Result<JsValue, JsValue> {
    await!(future::join_all(vec![register_command_helloWASM(context)]));
    Ok(JsValue::undefined())
}

#[wasm_bindgen]
pub fn activate(context: ExtensionContext) -> Promise {
    console_error_panic_hook::set_once();
    let future = activate_future(context);
    let pinned = future.boxed();
    let compat = Compat::new(pinned);
    future_to_promise(compat) // FIXME: wasm-bindgen-futures still uses old API
}

#[wasm_bindgen]
pub fn deactivate() {
    // empty
}

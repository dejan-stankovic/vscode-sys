#![feature(async_await)]
#![feature(await_macro)]
#![feature(futures_api)]
#![feature(impl_trait_in_bindings)]

use futures::{prelude::*, FutureExt};
use futures_util::compat::Compat;
use js_sys::Promise;
use vscode_sys::ExtensionContext;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::future_to_promise;

#[allow(non_snake_case)]
async fn register_command_helloWASM(context: ExtensionContext) -> Result<JsValue, JsValue> {
    use vscode_sys::{commands, window};
    let tag = "extension.helloWASM".into();
    let clo = Closure::<Fn()>::new(|| {
        let msg = "Hello from Rust!".into();
        window.show_information_message(msg);
    });
    let fun = clo.as_ref().unchecked_ref();
    context
        .subscriptions()
        .push(&commands.register_command(tag, fun));
    clo.forget();
    Ok(JsValue::undefined())
}

async fn register_commands(context: ExtensionContext) -> Result<JsValue, JsValue> {
    await!(future::join_all(vec![register_command_helloWASM(context)]));
    Ok(JsValue::undefined())
}

#[wasm_bindgen]
pub fn activate(context: ExtensionContext) -> Promise {
    console_error_panic_hook::set_once();
    let future = register_commands(context);
    let pinned = future.boxed();
    let compat = Compat::new(pinned);
    future_to_promise(compat) // FIXME: wasm-bindgen-futures still uses old API
}

#[wasm_bindgen]
pub fn deactivate() {
    // empty
}

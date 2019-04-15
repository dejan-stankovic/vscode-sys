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
fn command_helloWASM() {
    vscode_sys::window.show_information_message("Hello from Rust!".into());
}

#[allow(non_snake_case)]
async fn register_command_helloWASM(context: ExtensionContext) -> Result<JsValue, JsValue> {
    let tag = "extension.helloWASM".into();
    let clo = Closure::wrap(Box::new(command_helloWASM) as Box<dyn Fn()>);
    let fun = clo.as_ref().unchecked_ref();
    context
        .subscriptions()
        .push(&vscode_sys::commands.register_command(tag, fun));
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

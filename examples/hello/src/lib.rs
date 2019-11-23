use futures::prelude::*;
use js_sys::{Array, Promise};
use std::pin::Pin;
use vscode_sys::ExtensionContext;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::future_to_promise;

#[allow(non_snake_case)]
async fn register_command_helloWASM(context: &ExtensionContext) -> Result<JsValue, JsValue> {
    use vscode_sys::{commands, window};
    // invoked when the command "extension.helloWASM" is invoked
    let clo = Closure::wrap(Box::new(|| {
        // invoked when the "show_information_message" API call (below) finishes
        let clo = Closure::wrap(Box::new(|res| {
            let args = Array::new();
            args.push(&"should be `undefined`:".into());
            args.push(&res);
            web_sys::console::log(&args);
        }) as Box<dyn FnMut(JsValue)>);
        window
            .show_information_message("Hello from Rust!".into())
            .then(&clo);
        clo.forget();
    }) as Box<dyn Fn()>);
    context.subscriptions().push({
        let tag = "extension.helloWASM".into();
        let fun = clo.as_ref().unchecked_ref();
        &commands.register_command(tag, fun)
    });
    clo.forget();
    Ok(JsValue::undefined())
}

async fn activate_future(context: ExtensionContext) -> Result<JsValue, JsValue> {
    let mut futures: Vec<Pin<Box<dyn Future<Output = Result<_, _>>>>> = Vec::new();
    futures.push(Box::pin(register_command_helloWASM(&context)));
    future::try_join_all(futures).await?;
    Ok(JsValue::undefined())
}

#[wasm_bindgen]
pub fn activate(context: ExtensionContext) -> Promise {
    console_error_panic_hook::set_once();
    future_to_promise(Box::pin(activate_future(context))) // FIXME: wasm-bindgen-futures still uses old API
}

#[wasm_bindgen]
pub fn deactivate() {
    // empty
}

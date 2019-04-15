use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Disposable;

    #[wasm_bindgen(method)]
    pub fn dispose(this: &Disposable);
}

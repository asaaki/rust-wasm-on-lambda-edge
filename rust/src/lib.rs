// NOTE: We need to allow some dead code, because this example project
//       doesn't use all fields of every struct, but end users might.
#![allow(dead_code)]
// The wasm_bindgen attribute macros seem to trigger this lint
#![allow(clippy::unused_unit)]

#[cfg(target_arch = "wasm32")]
use lol_alloc::FreeListAllocator;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: FreeListAllocator = FreeListAllocator::new();

mod types;

use std::panic;
use types::cloudfront as cf;
use wasm_bindgen::{intern, prelude::*};
use web_sys::console;

type JsValueResult = Result<JsValue, JsValue>;

// convenient debug log helper
#[allow(unused_macros)]
macro_rules! debug_log {
    ( $( $t:tt )* ) => {
        web_sys::console::dir_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(start, skip_typescript)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console::log_1(&intern("(wasm module start)").into());
}

#[wasm_bindgen]
pub async fn handler(event: JsValue, _context: JsValue) -> JsValueResult {
    console::log_1(&intern("(wasm handler request call)").into());

    // debug_log!("event: {:#?}", cf::Event::from_js(event.clone()));

    let request = cf::Event::request_from_event(event)?;

    // TODO: Fancy biz logic here ...

    request.to_js()
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod types;

use std::panic;
use types::cloudfront as cf;
use wasm_bindgen::{intern, prelude::*};
use web_sys::console;

type JsValueResult = Result<JsValue, JsValue>;

#[wasm_bindgen(start, final)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console::log_1(&intern("(wasm module start)").into());
}

#[wasm_bindgen(final)]
pub async fn handler(event: JsValue, _context: JsValue) -> JsValueResult {
    console::log_1(&intern("(wasm handler request call)").into());
    // console::log_2(&intern("context:").into(), &context);
    let request = cf::Event::request_from_event(event)?;

    // TODO: Fancy biz logic here ...

    request.to_js()
}

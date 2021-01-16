extern crate wasm_bindgen;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::panic;
use wasm_bindgen::{intern, prelude::*};
use web_sys::console;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Records {
    #[serde(rename = "Records")]
    records: Vec<Record>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Record {
    cf: CloudFrontRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum CloudFrontRecord {
    CloudFrontResponse {
        config: Config,
        request: Request,
        response: Value,
    },
    CloudFrontRequest {
        config: Config,
        request: Request,
    },
}
use CloudFrontRecord::*;

impl CloudFrontRecord {
    fn request(&self) -> &Request {
        match self {
            CloudFrontResponse { ref request, .. } => request,
            CloudFrontRequest { ref request, .. } => request,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    distribution_domain_name: Option<String>,
    distribution_id: Option<String>,
    event_type: Option<String>,
    request_id: Option<String>, // b64 binary blob actually
}

/*
from TS @types/aws-lambda:

export interface CloudFrontRequest {
    body?: {
        action: 'read-only' | 'replace';
        data: string;
        encoding: 'base64' | 'text';
        readonly inputTruncated: boolean;
    };
    readonly clientIp: string;
    readonly method: string;
    uri: string;
    querystring: string;
    headers: CloudFrontHeaders;
    origin?: CloudFrontOrigin;
}

export interface CloudFrontHeaders {
    [name: string]: Array<{
        key?: string;
        value: string;
    }>;
}

*/
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Request {
    method: String,
    uri: String,
    querystring: String,
    headers: Value,
    client_ip: String,
    #[serde(skip_serializing)]
    origin: Option<Value>,
    #[serde(skip_serializing)]
    body: Option<Value>,
}

#[wasm_bindgen(start, final)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console::log_1(&intern("(wasm module start)").into());
}

#[wasm_bindgen(final)]
pub fn handler(event: JsValue, _context: JsValue) -> JsValue {
    console::log_1(&intern("(wasm handler request call)").into());
    // console::log_2(&intern("context:").into(), &context);

    let event_json: Records = JsValue::into_serde(&event).unwrap();
    let request = event_json.records.first().unwrap().cf.request();

    // TODO: Fancy biz logic here ...

    // let request_value = JsValue::from_serde(&request).unwrap();
    // console::log_2(&intern("request:").into(),&request_value);

    // We could go through the pain of making Request the exchange object,
    // but the generic JsValue avoids the TS shenanigans pretty well
    JsValue::from_serde(&request).unwrap()
}

/*
Note:
- valid return for request triggers are also:

export interface CloudFrontResultResponse {
    status: string;
    statusDescription?: string;
    headers?: CloudFrontHeaders;
    bodyEncoding?: 'text' | 'base64';
    body?: string;
}
*/

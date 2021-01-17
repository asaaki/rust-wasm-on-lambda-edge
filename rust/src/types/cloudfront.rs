/*
  References:
  - https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/lambda-event-structure.html
  - @types/aws-lambda/common/cloudfront.d.ts
*/

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;
use std::collections::BTreeMap;
use wasm_bindgen::{intern, prelude::*};

pub type Event = CloudFrontRecords;

type JsResult<T> = Result<T, JsValue>;
type JsValueResult = Result<JsValue, JsValue>;

#[derive(Debug, Clone, Deserialize)]
pub struct CloudFrontRecords {
    #[serde(rename = "Records")]
    pub records: Vec<CloudFrontRecordContainer>,
}

impl CloudFrontRecords {
    pub fn from_js(input: JsValue) -> JsResult<Self> {
        deserialize_js(input)
    }

    pub fn get_record(&self) -> JsResult<&CloudFrontRecord> {
        let first = self
            .records
            .first()
            .ok_or_else(|| intern("cannot retreive CloudFront record data"))?;
        Ok(&first.cf)
    }

    pub fn request_from_event(input: JsValue) -> JsResult<CloudFrontRequest> {
        let event = Self::from_js(input)?;
        let record = event.get_record()?;
        let request = record.clone_request();
        Ok(request)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloudFrontRecordContainer {
    pub cf: CloudFrontRecord,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloudFrontRecord {
    pub config: CloudFrontConfig,
    pub request: CloudFrontRequest,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<CloudFrontResponse>,
}

impl CloudFrontRecord {
    pub fn clone_request(&self) -> CloudFrontRequest {
        self.request.clone()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudFrontConfig {
    pub distribution_domain_name: String,
    pub distribution_id: String,
    pub event_type: String, // 'origin-request' | 'origin-response' | 'viewer-request' | 'viewer-response';
    pub request_id: String, // base64 encoded binary blob actually ;-)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudFrontRequest {
    method: String,
    uri: String,
    querystring: String,
    headers: CloudFrontHeadersMap,
    client_ip: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<CloudFrontOrigin>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<CloudFrontRequestBody>,
}

impl CloudFrontRequest {
    pub fn to_js(&self) -> JsValueResult {
        serialize_return(self)
    }
}

pub type CloudFrontHeadersMap = BTreeMap<String, CloudFrontHeaders>;
pub type CloudFrontHeaders = Vec<CloudFrontHeader>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFrontHeader {
    key: String,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CloudFrontOrigin {
    #[serde(rename = "s3", rename_all = "camelCase")]
    CloudFrontS3Origin {
        auth_method: String, // 'origin-access-identity' | 'none'
        custom_headers: CloudFrontHeadersMap,
        domain_name: String,
        path: String,
        region: String,
    },

    #[serde(rename = "custom", rename_all = "camelCase")]
    CloudFrontCustomOrigin {
        custom_headers: CloudFrontHeadersMap,
        domain_name: String,
        keepalive_timeout: u32,
        path: String,
        port: u16,
        protocol: String, // 'http' | 'https'
        read_timeout: u32,
        ssl_protocols: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudFrontRequestBody {
    action: String, // 'read-only' | 'replace'
    data: String,
    encoding: String, // 'base64' | 'text'
    input_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudFrontResponse {
    status: String, // the only required field
    status_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<CloudFrontHeadersMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body_encoding: Option<String>, // 'text' | 'base64'
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
}

// UTILITIES

#[inline]
fn deserialize_js<T>(input: JsValue) -> JsResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let deserialized = serde_wasm_bindgen::from_value(input)?;
    Ok(deserialized)
}

#[inline]
fn serialize_return<T>(input: &T) -> JsValueResult
where
    T: serde::ser::Serialize,
{
    // Note: might be not a good idea, because collections are turned into
    // ES2015 Maps, which are not JSON stringify'able out of the box
    // serde_wasm_bindgen::to_value(input)
    //     .map_err(|_| intern("could not serialize request struct").into() )

    JsValue::from_serde(input).map_err(|_| intern("could not serialize request struct").into())
}

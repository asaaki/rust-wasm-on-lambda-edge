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

pub(crate) type Event = CloudFrontRecords;

type JsResult<T> = Result<T, JsValue>;
type JsValueResult = Result<JsValue, JsValue>;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CloudFrontRecords {
    #[serde(rename = "Records")]
    pub(crate) records: Vec<CloudFrontRecordContainer>,
}

impl CloudFrontRecords {
    pub(crate) fn from_js(input: JsValue) -> JsResult<Self> {
        deserialize_js(input)
    }

    pub(crate) fn get_record(&self) -> JsResult<&CloudFrontRecord> {
        let first = self
            .records
            .first()
            .ok_or_else(|| intern("cannot retreive CloudFront record data"))?;
        Ok(&first.cf)
    }

    pub(crate) fn request_from_event(input: JsValue) -> JsResult<CloudFrontRequest> {
        let event = Self::from_js(input)?;
        let record = event.get_record()?;
        let request = record.clone_request();
        Ok(request)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CloudFrontRecordContainer {
    pub(crate) cf: CloudFrontRecord,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CloudFrontRecord {
    pub(crate) config: CloudFrontConfig,
    pub(crate) request: CloudFrontRequest,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) response: Option<CloudFrontResponse>,
}

impl CloudFrontRecord {
    pub(crate) fn clone_request(&self) -> CloudFrontRequest {
        self.request.clone()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CloudFrontConfig {
    pub(crate) distribution_domain_name: String,
    pub(crate) distribution_id: String,
    pub(crate) event_type: EventType,
    pub(crate) request_id: RequestId,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum EventType {
    OriginRequest,
    OriginResponse,
    ViewerRequest,
    ViewerResponse,
}

// base64 encoded binary blob actually ;-)
// but I also do not know if and what it encodes
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct RequestId(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudFrontRequest {
    method: String, // TODO: can be enum'ified
    uri: String,
    querystring: String,
    headers: CloudFrontHeadersMap,
    client_ip: String, // TODO: use IP type (probably a v4 + v6 compatible)

    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<CloudFrontOrigin>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<CloudFrontRequestBody>,
}

impl CloudFrontRequest {
    pub(crate) fn to_js(&self) -> JsValueResult {
        serialize_return(self)
    }
}

pub type CloudFrontHeadersMap = BTreeMap<String, CloudFrontHeaders>;
pub type CloudFrontHeaders = Vec<CloudFrontHeader>;

// TODO: no header validation; check if there is a crate with serde support
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
        auth_method: AuthMethod, // 'origin-access-identity' | 'none'
        custom_headers: CloudFrontHeadersMap,
        domain_name: String, // s3 bucket domain name, max 128, lowercase
        path: String,        // should start with /, but not end with it, max 255
        region: String,      // region of S3 bucket, only needed with OAI auth
    },

    #[serde(rename = "custom", rename_all = "camelCase")]
    CloudFrontCustomOrigin {
        custom_headers: CloudFrontHeadersMap,
        domain_name: String,   // domain name, no IPs, max 253
        keepalive_timeout: u8, // 1-60 (seconds)
        path: String,          // should start with /, but not end with it, max 255
        port: u16,
        protocol: Protocol,
        read_timeout: u8, // 4–60
        ssl_protocols: Vec<SslProtocol>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthMethod {
    None,
    OriginAccessIdentity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Protocol {
    Https,
    Http,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SslProtocol {
    // TLS v1.3 not present yet, but AWS should, it's 2021!
    #[serde(rename = "TLSv1.3")]
    TlsV13,
    #[serde(rename = "TLSv1.2")]
    TlsV12,
    // sad reality, yet versions below shouldn't be used anymore, please
    #[serde(rename = "TLSv1.1")]
    TlsV11,
    #[serde(rename = "TLSv1")]
    TlsV10,
    #[serde(rename = "SSLv3")]
    SslV3,
}

// TODO: struct should be only constructible via methods,
//       so we can parse/validate its fields
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudFrontRequestBody {
    action: Action,
    data: TextOrBase64,
    encoding: Encoding,
    input_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Action {
    ReadOnly,
    Replace,
}

// TODO: struct should be only constructible via methods,
//       so we can parse/validate its fields
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudFrontResponse {
    status: Status,
    status_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<CloudFrontHeadersMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body_encoding: Option<Encoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<TextOrBase64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Status(String); // TODO: create impl with constraints (100-599)

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Encoding {
    Base64,
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TextOrBase64(String);

impl TextOrBase64 {
    // consumes the struct and returns the inner value
    #[allow(dead_code)]
    pub(crate) fn take(self, encoding: Encoding) -> JsResult<String> {
        use Encoding::*;

        match encoding {
            Text => Ok(self.0),
            Base64 => {
                let data = base64::decode(self.0)
                    .map_err(|_| intern("cannot decode base64 encoded string"))?;
                let s = String::from_utf8(data)
                    .map_err(|_| intern("(b64 string) cannot convert to utf8 string"))?;
                Ok(s)
            }
        }
    }
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
    serde_wasm_bindgen::to_value(input).map_err(|_| intern("could not serialize request struct").into())
}

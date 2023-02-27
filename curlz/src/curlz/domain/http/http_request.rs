use serde::{Deserialize, Serialize};

use crate::domain::http::{HttpBody, HttpHeaders, HttpMethod, HttpUri, HttpVersion};
use crate::template::variables::Placeholder;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct HttpRequest {
    pub url: HttpUri,
    pub method: HttpMethod,
    pub version: HttpVersion,
    pub headers: HttpHeaders,
    pub body: HttpBody,
    pub curl_params: Vec<String>,
    pub placeholders: Vec<Placeholder>,
}

impl HttpRequest {
    pub fn update(&self, update_fn: impl Fn(&mut Self)) -> Self {
        let mut req = self.clone();
        update_fn(&mut req);

        req
    }
}

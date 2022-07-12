use serde::{Deserialize, Serialize};

use crate::data::{HttpHeaders, HttpMethod};
use crate::variables::Placeholder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub headers: HttpHeaders,
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

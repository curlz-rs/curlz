use serde::{Deserialize, Serialize};

use crate::data::{HttpHeaders, HttpMethod};
use crate::TemplateSlots;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub headers: HttpHeaders,
    pub curl_params: Vec<String>,
    pub placeholders: Vec<TemplateSlots>,
}

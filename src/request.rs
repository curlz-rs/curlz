use serde::{Deserialize, Serialize};

use crate::TemplateSlots;

#[derive(Debug, Serialize, Deserialize)]
pub struct Param(String, String);

#[derive(Debug, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub url: String,
    pub method: HttpMethod,
    pub curl_params: Vec<Param>,
    pub placeholders: Vec<TemplateSlots>,
}

mod invoke_curl;

use super::IssueRequest;
use crate::domain::request::RequestContext;
pub use invoke_curl::*;

pub trait HttpBackend {
    fn issue(&self, request: &IssueRequest, context: &RequestContext) -> crate::Result<()>;
}

use super::backend::*;
use crate::domain::environment::Environment;
use crate::domain::http::HttpRequest;
use crate::domain::request::RequestContext;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum Verbosity {
    Silent,
    Verbose,
}

pub struct IssueRequest<'r> {
    pub request: &'r HttpRequest,
    pub verbosity: Verbosity,
}

impl<'r> IssueRequest<'r> {
    pub fn new(request: &'r HttpRequest, verbosity: Verbosity) -> Self {
        Self { request, verbosity }
    }
}

/// issues a request with a given `backend`
pub fn issue_request(
    req: IssueRequest<'_>,
    backend: &impl HttpBackend,
    env: &Environment,
) -> crate::Result<()> {
    let ctx = RequestContext::new(env);

    backend.issue(&req, &ctx)
}

/// issues a request with the via curl
pub fn issue_request_with_curl(req: IssueRequest<'_>, env: &Environment) -> crate::Result<()> {
    let backend = InvokeCurlBackend::default();

    issue_request(req, &backend, env)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::sample_requests;
    use log::debug;

    #[derive(Default)]
    struct MockBackend;
    impl HttpBackend for MockBackend {
        fn issue(&self, _request: &IssueRequest, _context: &RequestContext) -> crate::Result<()> {
            debug!("MockBackend issues a request");
            Ok(())
        }
    }

    #[test]
    fn how_to_issue_a_request() {
        let env = Environment::default();
        let req = sample_requests::post_request();
        let req = IssueRequest::new(&req, Verbosity::Verbose);
        let backend = MockBackend::default();
        let res = issue_request(req, &backend, &env);

        assert!(res.is_ok())
    }
}

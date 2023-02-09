use crate::cli::HeaderArgs;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct HttpHeaders(Vec<(String, String)>);

impl HttpHeaders {
    /// reverses the order of headers
    pub fn reverse(&mut self) {
        self.0.reverse();
    }
}

impl HttpHeaders {
    /// adds a new header in the form of key, value
    pub fn push(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
        self.0
            .push((key.as_ref().to_string(), value.as_ref().to_string()));
    }

    pub fn merge(&mut self, other: &HttpHeaders) {
        self.0.extend(other.0.iter().cloned());
    }
}

impl AsRef<[(String, String)]> for HttpHeaders {
    fn as_ref(&self) -> &[(String, String)] {
        self.0.as_slice()
    }
}

impl From<HeaderArgs> for HttpHeaders {
    fn from(raw_headers: HeaderArgs) -> Self {
        raw_headers.as_ref().into()
    }
}

impl From<&[String]> for HttpHeaders {
    fn from(headers: &[String]) -> Self {
        Self(
            headers
                .iter()
                .map(|value| {
                    let (key, value) = value.split_once(':').unwrap();
                    (key.trim().to_string(), value.trim().to_string())
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_the_header_arg_flag_away() {
        let args = vec![
            "-H",
            "foo: bar",
            "--header",
            "Accept: application/json",
            "--header",
            r#"Authorization: Baerer {{ jwt({"foo": "bar"}) }}"#,
            "http://example.com",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();

        let headers: HeaderArgs = (&args).into();
        let headers: HttpHeaders = headers.into();

        assert_eq!(
            headers.as_ref(),
            &[
                ("foo".to_string(), "bar".to_string()),
                ("Accept".to_string(), "application/json".to_string()),
                (
                    "Authorization".to_string(),
                    r#"Baerer {{ jwt({"foo": "bar"}) }}"#.to_string()
                )
            ]
        );
    }
}

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpVersion {
    #[serde(rename = "HTTP/1.1")]
    Http11,
    #[serde(rename = "HTTP/2")]
    Http2,
    #[serde(rename = "HTTP/3")]
    Http3,
}

impl From<&HttpVersion> for String {
    fn from(version: &HttpVersion) -> Self {
        serde_yaml::to_string(version)
            .unwrap()
            .trim_end()
            .to_string()
    }
}

impl FromStr for HttpVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s.to_uppercase().as_str())
            .map_err(|_| anyhow!("Unsupported HTTP version: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_str_to_http_method_gracefully() {
        assert_eq!(
            "HTTP/1.1".parse::<HttpVersion>().unwrap(),
            HttpVersion::Http11
        );
        assert_eq!(
            "http/1.1".parse::<HttpVersion>().unwrap(),
            HttpVersion::Http11
        );
    }

    #[test]
    #[should_panic(expected = "Unsupported HTTP version: HTTP/1.0")]
    fn should_throw_unsupported_methods() {
        "HTTP/1.0".parse::<HttpVersion>().unwrap();
    }

    #[test]
    fn should_convert_to_uppercase_string() {
        let version: String = (&HttpVersion::Http11).into();
        assert_eq!(version, "HTTP/1.1");
    }
}

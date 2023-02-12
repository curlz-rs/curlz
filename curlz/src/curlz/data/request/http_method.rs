use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Trace,
    Connect,
    Patch,
}

impl From<&HttpMethod> for String {
    fn from(method: &HttpMethod) -> Self {
        format!("{:?}", method).to_uppercase()
    }
}

impl FromStr for HttpMethod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s.to_uppercase().as_str())
            .map_err(|_| anyhow!("Unsupported HTTP method: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_str_to_http_method_gracefully() {
        assert_eq!("Get".parse::<HttpMethod>().unwrap(), HttpMethod::Get);
        assert_eq!("GET".parse::<HttpMethod>().unwrap(), HttpMethod::Get);
        assert_eq!("get".parse::<HttpMethod>().unwrap(), HttpMethod::Get);
    }

    #[test]
    #[should_panic(expected = "Unsupported HTTP method: Pal")]
    fn should_throw_unsupported_methods() {
        "Pal".parse::<HttpMethod>().unwrap();
    }

    #[test]
    fn should_convert_to_uppercase_string() {
        let method: String = (&HttpMethod::Get).into();
        assert_eq!(method, "GET");
    }
}

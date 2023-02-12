use crate::data::HttpHeaders;

pub struct HeaderArgs(Vec<String>);

impl AsRef<[String]> for HeaderArgs {
    fn as_ref(&self) -> &[String] {
        self.0.as_slice()
    }
}

impl From<&Vec<String>> for HeaderArgs {
    fn from(raw_headers: &Vec<String>) -> Self {
        let mut headers = vec![];
        let mut i = 0_usize;
        while i < raw_headers.len() {
            match raw_headers.get(i).unwrap().as_str() {
                "-H" | "--header" => {
                    headers.push(raw_headers.get(i + 1).unwrap().to_string());
                    i += 2;
                }
                _ => {
                    i += 1;
                }
            }
        }

        HeaderArgs(headers)
    }
}

impl From<HeaderArgs> for HttpHeaders {
    fn from(raw_headers: HeaderArgs) -> Self {
        raw_headers.as_ref().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_the_header_arg_flag_away() {
        let mut args = vec![
            "-H",
            "foo: bar",
            "--header",
            "Accept: application/json",
            "--header",
            r#"Authorization: Baerer {{ jwt({"foo": "bar"}) }}"#,
            "http://example.com",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        let headers: HeaderArgs = (&args).into();

        assert_eq!(
            headers.as_ref(),
            &[
                "foo: bar".to_string(),
                "Accept: application/json".to_string(),
                r#"Authorization: Baerer {{ jwt({"foo": "bar"}) }}"#.to_string()
            ]
        );
        assert_eq!(args.len(), 7);
        assert_eq!(args.pop(), Some("http://example.com".to_string()));
    }
}

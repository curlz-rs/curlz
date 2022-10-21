pub struct HeaderArgs(Vec<String>);

impl AsRef<[String]> for HeaderArgs {
    fn as_ref(&self) -> &[String] {
        self.0.as_slice()
    }
}

impl From<&Vec<String>> for HeaderArgs {
    fn from(raw_headers: &Vec<String>) -> Self {
        let headers = raw_headers
            .iter()
            .enumerate()
            .step_by(2)
            .zip(raw_headers.clone().iter().enumerate().skip(1).step_by(2))
            .filter_map(|((_, key), (_, value))| match key.as_str() {
                "-H" | "--header" => Some(value.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>();

        HeaderArgs(headers)
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

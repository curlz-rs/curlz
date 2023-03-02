use base64::{alphabet, engine, Engine};
use minijinja::{Error, State};

/// generates a basic auth token, without the usual literal `Basic`
pub(super) fn basic(_state: &State, user: &str, password: &str) -> Result<String, Error> {
    let engine = engine::GeneralPurpose::new(&alphabet::URL_SAFE, engine::general_purpose::PAD);
    let token = engine.encode(format!("{user}:{password}"));

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::RenderBuilder;

    #[test]
    fn should_encode_base64() {
        let e = RenderBuilder::new().with_function("basic", basic);

        assert_eq!(
            &e.render(r#"{{ basic("bob", "secret") }}"#),
            "Ym9iOnNlY3JldA==",
            "token should be issued now-ish"
        );
    }
}

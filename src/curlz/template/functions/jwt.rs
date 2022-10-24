use chrono::{Duration, Timelike, Utc};
use jsonwebtoken::{EncodingKey, Header};
use minijinja::value::Value;
use minijinja::{Error, ErrorKind, State};
use std::collections::HashMap;
use std::ops::{Add, Not};

const EXPIRY: &str = "exp";
const ISSUED_AT: &str = "iat";

/// generates a jwt token based on some given claims
pub fn jwt(state: &State, claims: Value, signing_key: Option<Value>) -> Result<String, Error> {
    let mut claims: HashMap<String, serde_json::Value> =
        serde_json::from_str(claims.to_string().as_str()).unwrap();

    // in case expiry is missing, we expire it in 15min
    if claims.contains_key(EXPIRY).not() {
        let expire_in = Utc::now()
            .add(Duration::minutes(15))
            .with_second(0)
            .unwrap()
            .timestamp();
        claims.insert(EXPIRY.to_string(), serde_json::Value::from(expire_in));
    }

    claims.insert(
        ISSUED_AT.to_string(),
        serde_json::Value::from(Utc::now().timestamp()),
    );

    let signing_key = signing_key
        .or_else(|| state.lookup("jwt_signing_key"))
        .ok_or_else(|| {
            Error::new(
                ErrorKind::MissingArgument,
                "The variable `jwt_signing_key` was not defined.",
            )
        })?;

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(signing_key.as_bytes().unwrap()),
    );

    token.map_err(|e| {
        Error::new(
            ErrorKind::UndefinedError,
            "jsonwebtoken failed to encode the token.",
        )
        .with_source(e)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::RenderBuilder;
    use chrono::{Duration, Timelike};
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use rstest::rstest;
    use serde::Deserialize;

    #[test]
    #[should_panic(expected = "The variable `jwt_signing_key` was not defined.")]
    fn should_throw_when_signing_key_is_missing() {
        RenderBuilder::new()
            .with_function("jwt", jwt)
            .render(r#"Bearer {{ jwt({"sub": "b@b.com"}) }}"#);
    }

    #[rstest]
    #[case(
        r#"Bearer {{ jwt({"sub": "b@b.com"}, "000") }}"#, 
        RenderBuilder::new().with_function("jwt", jwt)
    )]
    #[case(
        r#"Bearer {{ jwt({"sub": "b@b.com"}) }}"#, 
        RenderBuilder::new().with_function("jwt", jwt).with_env_var("jwt_signing_key", "000")
    )]
    #[case(
        r#"Bearer {{ jwt({"sub": "b@b.com", "iat": 666}) }}"#, 
        RenderBuilder::new().with_function("jwt", jwt).with_env_var("jwt_signing_key", "000")
    )]
    fn should_set_expiry_when_missing(#[case] token: &str, #[case] builder: RenderBuilder) {
        let s = builder.render(token);

        #[derive(Deserialize)]
        struct Claims {
            sub: String,
            exp: i64,
            iat: i64,
        }
        let token = s.as_str().split(' ').last().unwrap();
        let token_message = decode::<Claims>(
            token,
            &DecodingKey::from_secret(b"000"),
            &Validation::new(Algorithm::HS256),
        )
        .unwrap();

        assert_eq!(token_message.claims.sub, "b@b.com".to_string());
        assert_eq!(
            token_message.claims.exp,
            Utc::now()
                .add(Duration::minutes(15))
                .with_second(0)
                .unwrap()
                .timestamp(),
            "token should expire in 15min"
        );
        dbg!(token_message.claims.iat);
        assert_eq!(
            token_message.claims.iat,
            Utc::now()
                // .add(Duration::minutes(15))
                // .with_second(0)
                // .unwrap()
                .timestamp(),
            "token should be issued now-ish"
        );
    }
}

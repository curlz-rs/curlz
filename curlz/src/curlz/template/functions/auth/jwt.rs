use chrono::{Duration, Timelike, Utc};
use humantime::parse_duration;
use jsonwebtoken::{EncodingKey, Header};
use minijinja::value::ValueKind;
use minijinja::{value::Value, Error, ErrorKind, State};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Add;

const CLAIM_EXPIRY: &str = "exp";
const CLAIM_ISSUED_AT: &str = "iat";

type ClaimsMap = HashMap<String, Value>;

/// generates a jwt token based on some given claims
pub(super) fn jwt(
    state: &State,
    claims: Value,
    jwt_signing_key: Option<Value>,
) -> Result<String, Error> {
    let mut claims_map = ClaimsMap::new();
    for key in claims.try_iter()? {
        let value = claims.get_item(&key)?;

        if value.is_undefined() {
            // we should give it a try as json string
            let claims: HashMap<String, serde_json::Value> =
                serde_json::from_str(claims.to_string().as_str()).map_err(|_| Error::new(
                    ErrorKind::CannotUnpack,
                    "The variable `claims` was not a valid json map nor consists of valid key-value arguments",
                ))?;
            claims.into_iter().for_each(|(k, v)| {
                claims_map.entry(k).or_insert(Value::from_serializable(&v));
            });

            break;
        }

        claims_map.insert(format!("{key}"), value);
    }

    prepare_claim_exp(&mut claims_map)?;
    prepare_claim_iat(&mut claims_map);

    let jwt_signing_key = jwt_signing_key
        .or_else(|| state.lookup("jwt_signing_key"))
        .or_else(|| claims_map.get("jwt_signing_key").map(|v| v.to_owned()))
        .ok_or_else(|| {
            Error::new(
                ErrorKind::MissingArgument,
                "The variable `jwt_signing_key` was not defined.",
            )
        })?;
    // in any case we want the signing key never in the claims list
    claims_map.remove("jwt_signing_key");

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims_map,
        &EncodingKey::from_secret(jwt_signing_key.as_bytes().ok_or(Error::new(
            ErrorKind::BadSerialization,
            "`jwt_signing_key` had an invalid datatype, it should be a string.",
        ))?),
    );

    token.map_err(|e| {
        Error::new(
            ErrorKind::UndefinedError,
            "jsonwebtoken failed to encode the token.",
        )
        .with_source(e)
    })
}

fn prepare_claim_iat(claims_map: &mut HashMap<String, Value>) {
    claims_map.insert(
        CLAIM_ISSUED_AT.to_string(),
        Value::from(Utc::now().with_second(0).unwrap().timestamp()),
    );
}

/// in case expiry is missing, the following values are supported:
/// - strings parsable as human date format like "15min"
/// - number of non-leap seconds since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp")
fn prepare_claim_exp(claims_map: &mut ClaimsMap) -> Result<(), Error> {
    let default_ts = || {
        Utc::now()
            .add(Duration::minutes(15))
            .with_second(0)
            .unwrap()
            .timestamp()
    };

    match claims_map.entry(CLAIM_EXPIRY.to_string()) {
        Entry::Occupied(mut exp) => {
            let exp_ts: i64 = match exp.get().kind() {
                ValueKind::Number => exp.get().clone().try_into().unwrap_or(default_ts()),
                ValueKind::String => {
                    parse_duration(exp.get().as_str().unwrap()).map_err(
                    |_| Error::new(
                        ErrorKind::CannotUnpack,
                        "claim `exp` has an invalid format it must be either a numerical duration or a duration string like '15min'",
                    )
                    ).map(|std_duration| {
                    // todo: this should not fail, except for some number overruns `i64` vs `u64`
                    Duration::from_std(std_duration).map(|duration|{
                        Utc::now().add(duration).with_second(0).unwrap().timestamp()
                    }).unwrap_or(default_ts())
                })?
                }
                _ => default_ts(),
            };

            exp.insert(Value::from(exp_ts));
        }
        Entry::Vacant(vac) => {
            vac.insert(Value::from(default_ts()));
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::RenderBuilder;
    use chrono::{Duration, Timelike};
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use rstest::rstest;
    use serde::Deserialize;

    const JWT_SECRET_KEY: &str = "000";

    mod unhappy_path {
        use super::*;

        #[test]
        #[should_panic(expected = "The variable `jwt_signing_key` was not defined.")]
        fn should_throw_when_signing_key_is_missing() {
            RenderBuilder::new()
                .with_function("jwt", jwt)
                .render(r#"Bearer {{ jwt(sub="b@b.com") }}"#);
        }

        #[test]
        #[should_panic(
            expected = "`jwt_signing_key` had an invalid datatype, it should be a string."
        )]
        fn should_throw_when_signing_key_is_of_invalid_type() {
            RenderBuilder::new()
                .with_function("jwt", jwt)
                .render(r#"Bearer {{ jwt(sub="b@b.com", jwt_signing_key=12345) }}"#);
        }

        #[test]
        #[should_panic(
            expected = "The variable `claims` was not a valid json map nor consists of valid key-value arguments"
        )]
        fn should_throw_when_invalid_claims_are_provided() {
            RenderBuilder::new()
                .with_function("jwt", jwt)
                .render(r#"Bearer {{ jwt("b@b.com") }}"#);
        }

        #[test]
        #[should_panic(
            expected = "claim `exp` has an invalid format it must be either a numerical duration or a duration string like '15min'"
        )]
        fn should_throw_when_invalid_expire_date_are_provided() {
            RenderBuilder::new()
                .with_function("jwt", jwt)
                .with_env_var("key", JWT_SECRET_KEY)
                .render(r#"Bearer {{ jwt(exp="should not be string", jwt_signing_key=key) }}"#);
        }
    }

    #[rstest]
    #[case(
        r#"Bearer {{ jwt(sub="b@b.com", jwt_signing_key="000") }}"#,
        RenderBuilder::new().with_function("jwt", jwt)
    )]
    #[case(
        r#"Bearer {{ jwt(sub="b@b.com") }}"#,
        RenderBuilder::new().with_function("jwt", jwt)
            .with_env_var("jwt_signing_key", JWT_SECRET_KEY)
    )]
    #[case(
        r#"Bearer {{ jwt(sub="b@b.com", iat=666) }}"#,
        RenderBuilder::new().with_function("jwt", jwt)
            .with_env_var("jwt_signing_key", JWT_SECRET_KEY)
    )]
    #[case(
        r#"Bearer {{ jwt(jwt_claims) }}"#,
        RenderBuilder::new().with_function("jwt", jwt)
            .with_env_var("jwt_signing_key", JWT_SECRET_KEY)
            .with_env_var("jwt_claims", r#"{"sub": "b@b.com", "iat": 666}"#)
    )]
    fn should_set_expiry_when_missing(#[case] token: &str, #[case] builder: RenderBuilder) {
        let now = Utc::now();
        let jwt = builder.render(token);

        let token_message = {
            #[derive(Deserialize)]
            struct Claims {
                sub: String,
                exp: i64,
                iat: i64,
            }

            let jwt = jwt.as_str().split(' ').last().unwrap();
            decode::<Claims>(
                jwt,
                &DecodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
                &Validation::new(Algorithm::HS256),
            )
            .expect("decode claims failed")
        };

        assert_eq!(token_message.claims.sub.as_str(), "b@b.com");
        assert_eq!(
            token_message.claims.exp,
            now.add(Duration::minutes(15))
                .with_second(0)
                .unwrap()
                .timestamp(),
            "token should expire in 15min"
        );
        assert_eq!(
            token_message.claims.iat,
            now.with_second(0).unwrap().timestamp(),
            "token should be issued now-ish"
        );
    }

    mod test_prepare_claim_exp {
        use super::*;

        #[test]
        fn test_default_value() {
            let mut hm = ClaimsMap::default();
            prepare_claim_exp(&mut hm).unwrap();
            assert_eq!(
                hm["exp"],
                Value::from(
                    Utc::now()
                        .add(Duration::minutes(15))
                        .with_second(0)
                        .unwrap()
                        .timestamp()
                )
            );
        }

        #[test]
        fn test_default_value_on_invalid() {
            let mut hm = ClaimsMap::default();
            hm.insert("exp".to_string(), Value::from(vec!["a", "b"]));
            prepare_claim_exp(&mut hm).unwrap();
            assert_eq!(
                hm["exp"],
                Value::from(
                    Utc::now()
                        .add(Duration::minutes(15))
                        .with_second(0)
                        .unwrap()
                        .timestamp()
                )
            );
        }

        #[test]
        fn test_string_parsing() {
            let mut hm = ClaimsMap::default();
            hm.insert("exp".to_string(), "1h".into());
            prepare_claim_exp(&mut hm).unwrap();
            assert_eq!(
                hm["exp"],
                Value::from(
                    Utc::now()
                        .add(Duration::hours(1))
                        .with_second(0)
                        .unwrap()
                        .timestamp()
                )
            );
        }

        #[test]
        fn test_numerical_value() {
            let mut hm = ClaimsMap::default();
            hm.insert("exp".to_string(), 1679575260_i64.into());
            prepare_claim_exp(&mut hm).unwrap();
            assert_eq!(hm["exp"], Value::from(1679575260_i64));
        }

        #[test]
        fn test_numerical_value_of_other_type() {
            let mut hm = ClaimsMap::default();
            hm.insert("exp".to_string(), 1679575260_u32.into());
            prepare_claim_exp(&mut hm).unwrap();
            assert_eq!(hm["exp"], Value::from(1679575260_i64));
        }
    }
}

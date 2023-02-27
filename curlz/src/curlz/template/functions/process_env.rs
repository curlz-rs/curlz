use minijinja::value::Value;
use minijinja::{Error, ErrorKind, State};
use std::env::VarError;

pub fn process_env(_state: &State, var_name: &Value) -> Result<String, Error> {
    let var_name = var_name.as_str().ok_or_else(|| {
        Error::new(
            ErrorKind::MissingArgument,
            "The argument `var_name` for function `processEnv(var_name)` is missing.",
        )
    })?;

    std::env::var(var_name).map_err(|e| match e {
        VarError::NotPresent => Error::new(
            ErrorKind::NonKey,
            format!("The process env variable `{var_name}` is not defined."),
        ),
        VarError::NotUnicode(_) => Error::new(
            ErrorKind::UndefinedError,
            format!("The process env variable `{var_name}` has an invalid unicode value."),
        ),
    })
}

#[cfg(test)]
mod tests {
    use crate::test_utils::RenderBuilder;
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn should_throw_when_var_name_is_missing() {
        assert_eq!(
            RenderBuilder::new()
                .with_function("processEnv", process_env)
                .render(r#"{{ processEnv("USER") }}"#),
            std::env::var("USER").unwrap()
        );
    }

    #[test]
    fn should_provide_process_env_var_via_env_object() {
        // TODO: this is just a toy around case
        let mut hm = HashMap::new();
        for (key, value) in std::env::vars() {
            println!("{key}: {value}");
            hm.insert(key, value);
        }
        assert!(hm.contains_key("USER"));
        let b = RenderBuilder::new().with_env_var("env", hm);

        assert_eq!(
            b.render(r#"{{ env.USER }}"#),
            std::env::var("USER").unwrap()
        );
    }
}

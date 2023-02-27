use minijinja::value::{StructObject, Value};
use minijinja::{Error, State};

pub struct ProcessEnv;
impl StructObject for ProcessEnv {
    fn get_field(&self, field: &str) -> Option<Value> {
        // std::env::var(var_name)
        //     .map(|v| Value::from_safe_string(v))
        //     .map_err(|e| match e {
        //         VarError::NotPresent => Error::new(
        //             ErrorKind::NonKey,
        //             format!("The process env variable `{var_name}` is not defined."),
        //         ),
        //         VarError::NotUnicode(_) => Error::new(
        //             ErrorKind::UndefinedError,
        //             format!("The process env variable `{var_name}` has an invalid unicode value."),
        //         ),
        //     })
        std::env::var(field).map(Value::from_safe_string).ok()
    }
}

/// function for minijinja
pub fn process_env(_: &State, var_name: &str) -> Result<Value, Error> {
    Ok(ProcessEnv.get_field(var_name).unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use crate::domain::environment::Environment;
    use crate::template::Renderer;
    use crate::test_utils::RenderBuilder;
    use std::collections::HashMap;

    use super::*;

    #[test]
    #[cfg(not(windows))]
    fn should_throw_when_var_name_is_missing() {
        assert_eq!(
            RenderBuilder::new()
                .with_function("processEnv", process_env)
                .render(r#"{{ processEnv("USER") }}"#),
            std::env::var("USER").unwrap()
        );
    }

    #[test]
    #[cfg(windows)]
    fn should_throw_when_var_name_is_missing() {
        assert_eq!(
            RenderBuilder::new()
                .with_function("processEnv", process_env)
                .render(r#"{{ processEnv("USERNAME") }}"#),
            std::env::var("USERNAME").unwrap()
        );
    }

    #[test]
    fn should_resolve_lazy_via_env_virtuell_object() {
        let mut r = Renderer::new(&Environment::default());
        #[cfg(not(windows))]
        assert_eq!(
            r.render(r#"{{ env.USER }}"#, "template").unwrap(),
            std::env::var("USER").unwrap()
        );
        #[cfg(windows)]
        assert_eq!(
            r.render(r#"{{ env.USERNAME }}"#, "template").unwrap(),
            std::env::var("USERNAME").unwrap()
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
        #[cfg(not(windows))]
        assert!(hm.contains_key("USER"));
        #[cfg(windows)]
        assert!(hm.contains_key("USERNAME"));

        let b = RenderBuilder::new().with_env_var("env", hm);

        #[cfg(not(windows))]
        assert_eq!(
            b.render(r#"{{ env.USER }}"#),
            std::env::var("USER").unwrap()
        );
        #[cfg(windows)]
        assert_eq!(
            b.render(r#"{{ env.USERNAME }}"#),
            std::env::var("USERNAME").unwrap()
        );
    }
}

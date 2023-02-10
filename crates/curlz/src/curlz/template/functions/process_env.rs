use minijinja::{Error, State};

#[allow(dead_code)]
pub fn process_env(_state: &State, var_name: &str) -> Result<String, Error> {
    dbg!(var_name);
    // let var_name = var_name.as_str().ok_or_else(|| {
    //     Error::new(
    //         ErrorKind::MissingArgument,
    //         "The argument `var_name` for function `processEnv` was not defined.",
    //     )
    // })?;

    // Ok(var_name.to_string())
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::test_utils::RenderBuilder;

    use super::*;

    #[test]
    #[ignore]
    #[should_panic(expected = "The argument `var_name` for function `processEnv` was not defined.")]
    fn should_throw_when_var_name_is_missing() {
        RenderBuilder::new()
            .with_function("processEnv", process_env)
            .render(r#"GET https://httpbin.org/get?user={{ processEnv("USERNAME") }}"#);
    }
}

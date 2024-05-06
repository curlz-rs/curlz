use dialoguer::{Input, Password};
use minijinja::value::Value;
use minijinja::{Environment, Error, ErrorKind};

pub fn register_functions(env: &mut Environment) {
    env.add_function("prompt_password", prompt_password);
    env.add_function("prompt_for", prompt_for);
}

/// prompt for a password, to be used in a minijinja template
fn prompt_password(_state: &minijinja::State) -> Result<String, Error> {
    Password::new()
        .with_prompt("Password")
        .allow_empty_password(true)
        .interact()
        .map_err(|e| {
            Error::new(ErrorKind::UndefinedError, "cannot read password from stdin").with_source(e)
        })
}

/// prompt for something that has a name, to be used in a minijinja template
fn prompt_for(_state: &minijinja::State, prompt: Value) -> Result<String, Error> {
    let prompt = prompt.to_string();
    Input::new()
        .with_prompt(prompt)
        .allow_empty(true)
        .interact()
        .map_err(|e| {
            Error::new(ErrorKind::UndefinedError, "cannot read prompt from stdin").with_source(e)
        })
}

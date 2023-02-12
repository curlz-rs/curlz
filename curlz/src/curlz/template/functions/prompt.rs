use dialoguer::{Input, Password};
use minijinja::value::Value;
use minijinja::{Error, ErrorKind};

/// prompt for a password, to be used in a minijinja template
pub fn prompt_password(_state: &minijinja::State, _args: Vec<Value>) -> Result<String, Error> {
    Password::new()
        .with_prompt("Password")
        .allow_empty_password(true)
        .interact()
        .map_err(|e| {
            Error::new(ErrorKind::UndefinedError, "cannot read password from stdin").with_source(e)
        })
}

/// prompt for something that has a name, to be used in a minijinja template
pub fn prompt_for(_state: &minijinja::State, prompt: Value) -> Result<String, Error> {
    let prompt = prompt.to_string();
    Input::new()
        .with_prompt(prompt)
        .allow_empty(true)
        .interact()
        .map_err(|e| {
            Error::new(ErrorKind::UndefinedError, "cannot read prompt from stdin").with_source(e)
        })
}

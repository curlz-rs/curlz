use crate::Result;

use dialoguer::{Input, Password};
use minijinja::value::Value;
use minijinja::{Error, ErrorKind};

pub fn user_question(prompt: &str, default: &Option<String>) -> Result<String> {
    let mut i = Input::<String>::new();
    i.with_prompt(prompt.to_string());
    if let Some(s) = default {
        i.default(s.to_owned());
    }
    i.interact().map_err(Into::<anyhow::Error>::into)
}

/// prompt for a password, to be used in a minijinja template
pub fn prompt_password(
    _state: &minijinja::State,
    _args: Vec<Value>,
) -> std::result::Result<String, Error> {
    Password::new()
        .with_prompt("Password")
        .allow_empty_password(true)
        .interact()
        .map_err(|e| {
            Error::new(
                ErrorKind::ImpossibleOperation,
                "cannot read password from stdin",
            )
            .with_source(e)
        })
}

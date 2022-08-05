use crate::cli::{Cli, Commands};
use crate::ops::{MutOperation, OperationContext};
use crate::variables::Placeholder;
use crate::workspace::Environment;

use clap::Parser;
use std::path::Path;

pub fn exec() -> crate::Result<()> {
    match Cli::parse().command {
        Commands::Request(r) => {
            let r = &r;
            let placeholders = r.parse_define_as_placeholders();
            let env = create_environment(&r.env_file, &placeholders)?;
            let mut ctx = OperationContext::new(env)?;

            r.execute(&mut ctx)
        }
        Commands::Bookmark(_b) => {
            todo!()
        }
    }
}

/// creates an [`Environment`] from a `.env` file or a `.yaml` file
/// If the file does not exist, an empty [`Environment`] is returned.
///
/// ## Fallible
/// If `env_file` is not a `.env` or `.yaml` file, an error is returned.
/// If `env_file` is a directory, an error is returned.
fn create_environment(
    env_file: impl AsRef<Path>,
    placeholders: &[Placeholder],
) -> crate::Result<Environment> {
    Environment::try_from(env_file.as_ref()).map(|mut env| {
        placeholders
            .iter()
            .map(|placeholder| {
                let Placeholder {
                    name,
                    value,
                    default,
                    ..
                } = placeholder;
                (name, value.as_ref().or(default.as_ref()).unwrap())
            })
            .for_each(|(k, v)| env.insert(k, v));
        env
    })
}

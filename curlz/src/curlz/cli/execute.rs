use super::commands::*;

use crate::ops::{MutOperation, OperationContext};
use crate::variables::Placeholder;
use crate::workspace::Environment;
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Target;
use std::path::Path;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None, arg_required_else_help(true))]
#[clap(propagate_version = true)]
#[clap(name = "curlz")]
pub struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    #[clap(subcommand)]
    pub command: Commands,
}

pub fn execute() -> crate::Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .target(Target::Stderr)
        .init();
    let verbosity = if args.verbose.is_silent() {
        crate::ops::Verbosity::Silent
    } else {
        crate::ops::Verbosity::Verbose
    };

    match args.command {
        Commands::Request(r) => {
            let r = &r;
            let placeholders = r.parse_define_as_placeholders();
            let env = create_environment(&r.env_file, &placeholders)?;
            let mut ctx = OperationContext::new(env, verbosity)?;

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

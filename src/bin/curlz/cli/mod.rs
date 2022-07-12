pub mod commands;

use clap::Parser;

pub use commands::*;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None, arg_required_else_help(true))]
#[clap(propagate_version = true)]
#[clap(name = "curlz")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

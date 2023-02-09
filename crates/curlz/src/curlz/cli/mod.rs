pub mod commands;
mod header_args;
pub mod main;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

pub use commands::*;
pub use header_args::HeaderArgs;

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

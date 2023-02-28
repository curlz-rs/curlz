use super::sub_commands::*;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Target;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None, arg_required_else_help(true))]
#[clap(propagate_version = true)]
#[clap(name = "curlz")]
pub struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    #[clap(subcommand)]
    pub command: SubCommands,
}

pub fn execute() -> crate::Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .target(Target::Stderr)
        .init();

    match args.command {
        SubCommands::Request(ref r) => r.execute(),
        SubCommands::Bookmark(_b) => {
            todo!()
        }
        #[cfg(feature = "x-http-lang")]
        SubCommands::HttpFile(ref hf) => hf.execute(),
    }
}

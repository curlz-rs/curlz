use super::sub_commands::*;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Target;

#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = None )]
#[clap(subcommand_required = true, arg_required_else_help = true)]
#[command(propagate_version = true)]
#[command(name = "curlz")]
pub struct Cli {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
    #[command(subcommand)]
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

use super::sub_commands::*;

use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, generator::Generator, Shell};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Target;

#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = None )]
#[command(arg_required_else_help = true)]
#[command(propagate_version = true)]
#[command(name = "curlz")]
pub struct Cli {
    /// increased verbosity will show more debug output of curlz itself
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    /// Generate a SHELL completion script and print to stdout
    #[clap(long, short, value_name = "SHELL")]
    pub completions: Option<Shell>,

    #[command(subcommand)]
    pub cmd: Option<SubCommands>,
}

pub fn execute() -> crate::Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .target(Target::Stderr)
        .init();

    if let Some(shell) = args.completions {
        let mut cmd = Cli::command();
        print_completions(shell, &mut cmd);
        std::process::exit(0);
    }

    if let Some(cmd) = &args.cmd {
        match cmd {
            SubCommands::Request(ref r) => r.execute(),
            SubCommands::Bookmark(_b) => {
                todo!()
            }
            #[cfg(feature = "x-http-lang")]
            SubCommands::HttpFile(ref hf) => hf.execute(),
        }
    } else {
        Ok(())
    }
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

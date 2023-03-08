use clap::{Args, Subcommand};

#[derive(Clone, Debug, Args)]
#[command(arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
pub struct BookmarkCli {
    #[clap(subcommand)]
    pub command: BookmarkCommands,
}

#[derive(Clone, Debug, Subcommand)]
pub enum BookmarkCommands {
    List,
    Rename {
        #[clap(value_parser)]
        name: String,
        #[clap(value_parser)]
        new_name: String,
    },
    Remove {
        #[clap(value_parser)]
        name: String,
    },
    Show {
        #[clap(value_parser)]
        name: String,
    },
}

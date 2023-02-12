use clap::Subcommand;

mod bookmark;
mod request;

pub use bookmark::*;
pub use request::*;

#[derive(Clone, Debug, Subcommand)]
pub enum SubCommands {
    #[clap(alias("r"))]
    Request(RequestCli),
    #[clap(alias("b"))]
    /// similar to git remote, we want to support `list`, `add`, `rename`, `remove` and `show`
    Bookmark(BookmarkCli),
}

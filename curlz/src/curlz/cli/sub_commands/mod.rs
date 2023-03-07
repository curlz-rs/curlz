use clap::Subcommand;

mod bookmark;
mod request;

pub use bookmark::*;
pub use request::*;

#[cfg(feature = "x-http-lang")]
mod http_file;
#[cfg(feature = "x-http-lang")]
pub use http_file::*;

#[derive(Clone, Debug, Subcommand)]
pub enum SubCommands {
    #[command(alias("r"))]
    Request(RequestCli),
    #[command(alias("b"))]
    /// similar to git remote, we want to support `list`, `add`, `rename`, `remove` and `show`
    Bookmark(BookmarkCli),
    #[cfg(feature = "x-http-lang")]
    HttpFile(HttpFileCli),
}

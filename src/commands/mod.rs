mod run_curl;
mod save_bookmark;

pub use run_curl::*;
pub use save_bookmark::*;

use crate::CommandContext;

pub trait Execute {
    type Output;

    fn execute(self, context: &CommandContext) -> crate::Result<Self::Output>;
}

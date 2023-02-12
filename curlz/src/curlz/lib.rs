pub mod cli;
pub mod data;
pub mod interactive;
pub mod ops;
pub mod template;
pub mod utils;
pub mod variables;
pub mod workspace;

mod http_file;
#[cfg(test)]
pub mod test_utils;

#[macro_use]
extern crate pest_derive;
extern crate core;

pub type Result<T> = anyhow::Result<T>;

pub mod prelude {}

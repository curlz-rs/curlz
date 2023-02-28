pub mod cli;
pub mod template;
pub mod utils;

// the main domain logic
pub mod domain;

#[cfg(feature = "x-http-lang")]
#[macro_use]
extern crate pest_derive;

#[cfg(test)]
pub mod test_utils;

extern crate core;

pub type Result<T> = anyhow::Result<T>;

pub mod prelude {}

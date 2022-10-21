pub mod cli;
pub mod data;
pub mod interactive;
pub mod ops;
pub mod template;
pub mod utils;
pub mod variables;
pub mod workspace;

#[cfg(test)]
pub mod test_utils;

pub type Result<T> = anyhow::Result<T>;

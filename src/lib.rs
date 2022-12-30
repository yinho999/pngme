pub mod args;
mod chunk;
mod chunk_type;
pub mod commands;
mod png;

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

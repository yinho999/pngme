use anyhow::{self, Ok};
use args::{Cli, Commands};
use clap::{Command, Parser};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.commands {
        Commands::Encode(encode_args) => commands::encode(encode_args)?,
        Commands::Decode(decode_args) => commands::decode(decode_args)?,
        Commands::Remove(remove_args) => commands::remove(remove_args)?,
        Commands::Print(print_args) => commands::print_chunks(print_args)?,
    }
    Ok(())
}

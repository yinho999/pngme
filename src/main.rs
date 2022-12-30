use anyhow::{self, Ok};
use clap::Parser;
use pngme::{
    args::{Cli, Commands},
    commands,
};

fn main() -> pngme::Result<()> {
    let cli = Cli::parse();
    match cli.commands {
        Commands::Encode(encode_args) => commands::encode(encode_args)?,
        Commands::Decode(decode_args) => commands::decode(decode_args)?,
        Commands::Remove(remove_args) => commands::remove(remove_args)?,
        Commands::Print(print_args) => commands::print_chunks(print_args)?,
    }
    Ok(())
}

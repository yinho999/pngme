use std::{path::PathBuf, str::FromStr};

use clap::{Args, Parser, Subcommand};

use crate::chunk_type::ChunkType;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encode the png file with the hidden message with chunk type
    Encode(EncodeArgs),

    /// Decode the png file with the hidden message using chunk type
    Decode(DecodeArgs),

    /// Remove the hidden message using the chunk type
    Remove(RemoveArgs),

    Print(PrintArgs),
}
#[derive(Debug, Args)]
pub struct EncodeArgs {
    /// File path for Png file
    pub file_path: PathBuf,

    /// Chunk type
    #[clap(value_parser = chunk_parser)]
    pub chunk_type: ChunkType,

    /// Message
    pub message: String,

    /// Write the output PNG file to specific location
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    /// File path for Png file
    pub file_path: PathBuf,

    /// Chunk type
    #[clap(value_parser = chunk_parser)]
    pub chunk_type: ChunkType,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// File path for Png file
    pub file_path: PathBuf,

    /// Chunk type
    #[clap(value_parser = chunk_parser)]
    pub chunk_type: ChunkType,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    /// File path for Png file
    pub file_path: PathBuf,
}

fn chunk_parser(s: &str) -> Result<ChunkType, String> {
    match ChunkType::from_str(s) {
        Ok(chunk_type) => Ok(chunk_type),
        Err(e) => Err(e.to_string()),
    }
}

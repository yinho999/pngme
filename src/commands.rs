use std::convert::TryFrom;
use std::fs;

use anyhow::bail;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, Png};
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let EncodeArgs {
        file_path,
        chunk_type,
        message,
        output_file,
    } = args;

    let png_file = fs::read(file_path.clone())?;

    let mut png = Png::try_from(png_file.as_slice())?;

    let chunk = Chunk::new(chunk_type.clone(), message.as_bytes().to_vec());

    png.append_chunk(chunk);

    let outputdir = match output_file {
        Some(path) => path,
        None => file_path.clone(),
    };
    fs::write(outputdir, png.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let DecodeArgs {
        file_path,
        chunk_type,
    } = args;
    let png_file = fs::read(file_path)?;
    let png = Png::try_from(png_file.as_slice())?;
    match png.chunk_by_type(&chunk_type.to_string()) {
        Some(chunk) => {
            println!("The chunk is: {}", chunk.to_string());
            Ok(())
        }
        None => bail!("Not found"),
    }
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let RemoveArgs {
        file_path,
        chunk_type,
    } = args;
    let png_file = fs::read(file_path.clone())?;
    let mut png = Png::try_from(png_file.as_slice())?;
    png.remove_chunk(&chunk_type.to_string())?;

    fs::write(file_path, png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let PrintArgs { file_path } = args;
    let png_file = fs::read(file_path.clone())?;
    let png = Png::try_from(png_file.as_slice())?;
    let mut count = 0;

    // Print all message in png
    for chunk in png.chunks() {
        if let Ok(msg) = chunk.data_as_string() {
            if msg.trim() != "" {
                count += 1;
                println!(
                    "{}: Chunk Type - {}, Msg: {}",
                    count,
                    chunk.chunk_type(),
                    msg
                )
            }
        }
    }
    println!("{} results in total", count);
    Ok(())
}

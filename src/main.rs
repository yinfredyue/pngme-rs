use clap::{Parser, ValueEnum};
use std::{io::Write, path::PathBuf, str::FromStr};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
enum Command {
    Encode,
    Decode,
    Remove,
    Print,
}

#[derive(clap::Parser, Debug)]
#[command(name = "")]
struct Args {
    /// command to run
    #[arg(value_enum)]
    command: Command,

    /// input/output file
    #[arg(value_name = "FILE")]
    file_path: PathBuf,

    /// chunk type, ignore for print
    #[arg(value_name = "CHUNK")]
    chunk_type: Option<String>,

    /// message to encode, ignored for other commands
    #[arg(value_name = "MESSAGE")]
    message: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file_content = std::fs::read(&args.file_path).unwrap();
    let mut png = Png::try_from(&file_content[..]).unwrap();

    let mut overwrite_file = false;
    match args.command {
        Command::Encode => {
            let chunk_type = ChunkType::from_str(&args.chunk_type.unwrap()).unwrap();
            let new_chunk = Chunk::new(chunk_type, args.message.unwrap().as_bytes().to_vec());
            png.append_chunk(new_chunk);
            overwrite_file = true;
        }
        Command::Decode => {
            let chunk = png.chunk_by_type(&args.chunk_type.unwrap()).unwrap();
            println!("{}", chunk);
        }
        Command::Remove => {
            let removed = png.remove_chunk(&args.chunk_type.unwrap()).unwrap();
            println!("Removed: {}", removed);
            overwrite_file = true;
        }
        Command::Print => {
            println!("{}", png);
        }
    }

    if overwrite_file {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&args.file_path)?;

        f.write_all(&png.as_bytes()[..]).unwrap();
        f.flush().unwrap();
    }

    Ok(())
}

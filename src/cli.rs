use std::path::PathBuf;

use anyhow::{Result};
use clap::{Parser, Subcommand};

use crate::commands::{decode, encode, print, remove};

#[derive(Debug, Parser)]
#[command(name = "pngpong")]
#[command(about = "CLI commands to hide and retrieve messages in PNG files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encodes a message inside a PGN file
    #[command(arg_required_else_help = true)]
    Encode {
        #[arg(required = true)]
        file_path: PathBuf,
        #[arg(required = true)]
        chunk_type: String,
        #[arg(required = true)]
        message: String,
        #[arg(value_name = "OUTPUT")]
        output_path: Option<PathBuf>,
    },
    /// Decodes message hidden in PGN file
    Decode {
        #[arg(required = true)]
        file_path: PathBuf,
        #[arg(required = true)]
        chunk_type: String,
    },
    /// Removes first chunk of specified type from PNG file
    #[command(arg_required_else_help = true)]
    Remove {
        #[arg(required = true)]
        file_path: PathBuf,
        #[arg(required = true)]
        chunk_type: String,
    },
    /// Prints desired file to CLI
    #[command(arg_required_else_help = true)]
    Print {
        #[arg(required = true)]
        file_path: PathBuf,
    },
}

pub fn parse_cli() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_path,
        } => encode(file_path, &chunk_type, &message, output_path),
        Commands::Decode {
            file_path,
            chunk_type,
        } => decode(file_path, &chunk_type),
        Commands::Remove {
            file_path,
            chunk_type,
        } => remove(file_path, &chunk_type),
        Commands::Print { file_path } => print(file_path),
    }
}

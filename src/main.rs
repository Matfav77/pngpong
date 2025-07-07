#![allow(dead_code)]
use crate::cli::parse_cli;

use anyhow::{Result};

mod cli;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() -> Result<()> {
    parse_cli()?;
    Ok(())
}

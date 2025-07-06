use crate::cli::parse_cli;

mod cli;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    parse_cli()?;
    Ok(())
}

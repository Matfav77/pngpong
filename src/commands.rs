use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{Result, anyhow};

use crate::png::Png;
use crate::{chunk::Chunk, chunk_type::ChunkType};

pub fn encode(
    path: PathBuf,
    chunk_type: &str,
    message: &str,
    output_path: Option<PathBuf>,
) -> Result<()> {
    let data: Vec<u8> = fs::read(&path)?;
    let mut png = Png::try_from(&data[..])?;
    let chunk_type = ChunkType::from_str(chunk_type)?;
    let new_chunk = Chunk::new(chunk_type, message.bytes().collect());
    png.append_chunk(new_chunk);
    match output_path {
        Some(out) => fs::write(&out, png.as_bytes())?,
        None => fs::write(&path, png.as_bytes())?,
    }
    Ok(())
}

pub fn decode(path: PathBuf, chunk_type: &str) -> Result<()> {
    let data: Vec<u8> = fs::read(path)?;
    let png = Png::try_from(&data[..])?;
    if let Some(chunk) = png.chunk_by_type(chunk_type) {
        let string = chunk.data_as_string()?;
        println!("{string}");
    } else {
        return Err(anyhow!("no chunk of the specified type found"));
    }
    Ok(())
}

pub fn remove(path: PathBuf, chunk_type: &str) -> Result<()> {
    let data: Vec<u8> = fs::read(&path)?;
    let mut png = Png::try_from(&data[..])?;
    png.remove_first_chunk(chunk_type)?;
    fs::write(&path, png.as_bytes())?;
    Ok(())
}

pub fn print(path: PathBuf) -> Result<()> {
    let data: Vec<u8> = fs::read(path)?;
    let png = Png::try_from(&data[..])?;
    println!("{png}");
    Ok(())
}

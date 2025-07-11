use anyhow::{Error, Result, anyhow};
use std::fmt::Display;
use std::io::{BufReader, Read};
use std::str;

use crate::chunk_type::ChunkType;
use crc::crc32;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Self::calculate_crc(&chunk_type, &data);
        Chunk {
            length: data.len() as u32,
            chunk_type: chunk_type,
            data: data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let parsed = str::from_utf8(&self.data)?;
        Ok(parsed.to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .copied()
            .chain(self.chunk_type.bytes().iter().copied())
            .chain(self.data.iter().copied())
            .chain(self.crc.to_be_bytes().iter().copied())
            .collect()
    }

    pub fn calculate_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let crc_data: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .copied()
            .chain(data.iter().copied())
            .collect();
        crc32::checksum_ieee(&crc_data)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Length: {}\nChunk type: {}\nData: {:#?}\nCRC: {}",
            self.length, self.chunk_type, self.data, self.crc
        )
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            return Err(anyhow!("The array provided does not have enough data"));
        }
        let mut reader = BufReader::new(value);
        let mut buffer: [u8; 4] = [0, 0, 0, 0];

        reader.read_exact(&mut buffer)?;
        let data_length = u32::from_be_bytes(buffer);

        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;

        let mut data: Vec<u8> = vec![0; data_length as usize];
        reader.read_exact(&mut data)?;

        reader.read_exact(&mut buffer)?;
        let crc = u32::from_be_bytes(buffer);

        let computed_crc = Chunk::calculate_crc(&chunk_type, &data);
        if crc != computed_crc {
            return Err(anyhow!(
                "The CRC provided is not correct in relation to the other chunks"
            ));
        }

        Ok(Self {
            length: data_length,
            chunk_type,
            data,
            crc,
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}

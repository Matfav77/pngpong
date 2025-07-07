use anyhow::{Error, Result, anyhow};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    fn is_valid_png_byte(b: u8) -> bool {
        (b >= 65 && b <= 90) || (b >= 97 && b <= 122)
    }

    fn get_fifth_bit(b: &u8) -> u8 {
        b >> 5 & 1
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.bytes.clone()
    }

    pub fn is_critical(&self) -> bool {
        Self::get_fifth_bit(&self.bytes[0]) == 0
    }

    pub fn is_public(&self) -> bool {
        Self::get_fifth_bit(&self.bytes[1]) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        Self::get_fifth_bit(&self.bytes[2]) == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        Self::get_fifth_bit(&self.bytes[3]) == 1
    }

    pub fn is_valid(&self) -> bool {
        Self::is_valid_png_byte(self.bytes[0])
            && Self::is_valid_png_byte(self.bytes[1])
            && Self::is_valid_png_byte(self.bytes[2])
            && Self::is_valid_png_byte(self.bytes[3])
            && Self::is_reserved_bit_valid(&self)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        for b in bytes {
            if !Self::is_valid_png_byte(b) {
                return Err(anyhow!(
                    "provided character's byte is not in the correct range
"
                ));
            }
        }
        Ok(ChunkType { bytes })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        if bytes.len() == 4 && bytes.is_ascii() {
            return Ok(Self::try_from([bytes[0], bytes[1], bytes[2], bytes[3]])?);
        } else {
            return Err(anyhow!(
                "chunk name needs to be of length 4, length of name \"{}\" provided: {}",
                s,
                bytes.len()
            ));
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = std::str::from_utf8(&self.bytes);
        write!(f, "{}", str.expect("String not in the expected format"))
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.bytes() == other.bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());

        let bytes = "Rust".as_bytes();
        let bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let chunk = ChunkType { bytes };
        assert!(!chunk.is_valid());

        let bytes = "Ru1t".as_bytes();
        let bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let chunk = ChunkType { bytes };
        assert!(!chunk.is_valid());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

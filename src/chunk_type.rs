use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Debug)]
pub enum CreationError {
    InvalidByte,
    InvalidStrLength,
}

impl Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidByte => write!(f, "provided byte is not in the correct range"),
            Self::InvalidStrLength => write!(f, "string needs to be of length 4"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    fn is_valid_png_byte(b: u8) -> bool {
        (b >= 65 && b <= 90) || (b >= 97 && b <= 122)
    }

    fn get_fifth_bit(b: &u8) -> u8 {
        b >> 4 & 1
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
    type Error = CreationError;

    fn try_from(bytes: [u8; 4]) -> Result<Self, CreationError> {
        for b in bytes {
            if !Self::is_valid_png_byte(b) {
                return Err(CreationError::InvalidByte);
            }
        }
        Ok(ChunkType { bytes })
    }
}

impl FromStr for ChunkType {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() == 4 && bytes.is_ascii() {
            return Ok(Self::try_from([bytes[0], bytes[1], bytes[2], bytes[3]])?);
        } else {
            return Err(CreationError::InvalidStrLength);
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = std::str::from_utf8(&self.bytes);
        write!(f, "{}", str.expect("String not in the expected format"))
    }
}

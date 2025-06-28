use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Debug)]
enum CreationError {
    InvalidByte,
    InvalidStrLength,
}

impl Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidByte => write!(f, "provided byte is not in the correct range"),
            Self::InvalidStrLength => write!(f, "string needs to be of length 4")
        }
    }
}

pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    fn is_valid_png_byte(b: u8) -> bool {
        (b >= 65 && b <= 90) || (b >= 97 && b <= 122)
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


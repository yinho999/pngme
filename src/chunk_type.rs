use anyhow::bail;

use crate::Error;
use std::{char, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType(u8, u8, u8, u8);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    fn is_valid(&self) -> bool {
        self.is_valid_ascii() && self.is_reserved_bit_valid()
    }
    fn is_valid_ascii(&self) -> bool {
        self.bytes()
            .iter()
            .all(|&i| (65..=90).contains(&i) || (97..=122).contains(&i))
    }
    /// A type code is critical if bit 5 (value 32) of the first byte is 0
    fn is_critical(&self) -> bool {
        (self.0 >> 5) & 1 == 0
    }
    /// A type code is public if bit 5 (value 32) of the second byte is 0
    fn is_public(&self) -> bool {
        (self.1 >> 5) & 1 == 0
    }
    /// Bit 5 of the third byte is reserved and must be 0
    fn is_reserved_bit_valid(&self) -> bool {
        (self.2 >> 5) & 1 == 0
    }
    /// A type code is safe to copy if bit 5 (value 32) of the fourth byte is 1
    fn is_safe_to_copy(&self) -> bool {
        (self.3 >> 5) & 1 == 1
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk = Self(value[0], value[1], value[2], value[3]);
        match chunk.is_valid() {
            true => Ok(chunk),
            false => bail!("Invalid chunk type inputted"),
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.to_owned();
        let bytes = String::as_bytes(&binding);
        if bytes.len() != 4 {
            bail!("The input bytes not equals to 4");
        }
        let chunk_type = ChunkType(bytes[0], bytes[1], bytes[2], bytes[3]);
        if !chunk_type.is_valid_ascii() {
            bail!("Invalid ascii input")
        }
        Ok(chunk_type)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.0 as char, self.1 as char, self.2 as char, self.3 as char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

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

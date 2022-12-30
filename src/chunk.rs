use std::fmt::Display;

use anyhow::bail;
use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;
use crate::Error;
use crate::Result;

const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub const DATA_LENGTH: usize = 4;
    pub const CHUNK_TYPE_LENGTH: usize = 4;
    pub const CRC_LENGTH: usize = 4;

    pub const META_BYTES: usize = Chunk::DATA_LENGTH + Chunk::CHUNK_TYPE_LENGTH + Chunk::CRC_LENGTH;

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let mut total_bytes: Vec<u8> = chunk_type.bytes().to_vec();
        total_bytes.append(&mut data.clone());
        Chunk {
            length: data.len() as u32,
            chunk_type,
            data: data,
            crc: CRC32.checksum(&total_bytes),
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    fn data(&self) -> &[u8] {
        &self.data
    }
    fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String> {
        match String::from_utf8(self.data.clone()) {
            Ok(val) => Ok(val),
            Err(_) => bail!("Unable to convert from vec<u8> to utf8"),
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect::<Vec<u8>>()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(chunk_data: &[u8]) -> Result<Self> {
        if chunk_data.len() < 12 {
            bail!("Length of the chunk is not enough to convert to Chunk");
        }
        let data_length = u32::from_be_bytes(
            chunk_data[0..Chunk::DATA_LENGTH]
                .to_owned()
                .as_slice()
                .try_into()
                .expect("This should be 4 bytes"),
        );
        let chunk_type: [u8; 4] = chunk_data
            [Chunk::DATA_LENGTH..Chunk::DATA_LENGTH + Chunk::CHUNK_TYPE_LENGTH]
            .to_vec()
            .try_into()
            .expect("This should be 4 bytes");
        let message = chunk_data[Chunk::DATA_LENGTH + Chunk::CHUNK_TYPE_LENGTH
            ..Chunk::DATA_LENGTH + Chunk::CHUNK_TYPE_LENGTH + data_length as usize]
            .to_vec();
        let crc = u32::from_be_bytes(
            chunk_data[Chunk::DATA_LENGTH + Chunk::CHUNK_TYPE_LENGTH + data_length as usize
                ..Chunk::META_BYTES + data_length as usize]
                .try_into()
                .expect("this should be 4 bytes"),
        );
        let chunk_type = ChunkType::try_from(chunk_type).expect("Cannot convert");
        let create_chunk = Chunk::new(chunk_type, message);
        match crc == create_chunk.crc() {
            true => Ok(create_chunk),
            false => bail!("Invalid crc"),
        }
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = String::from_utf8(self.data().to_vec());

        match msg {
            Ok(msg) => write!(f, "{:?}", msg),
            Err(e) => write!(f, "{:?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
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

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

use crate::chunk_type::ChunkType;

/// Each PNG chunk contains four parts: length, chunk type, chunk data, and a
/// CRC (Cyclic Redundancy Check).
#[derive(Debug)]
pub struct Chunk {
    /// A 4-byte unsigned integer indicating the number of bytes in the chunk's
    /// data field. This length *only* counts bytes within the data field. It
    /// does not include the number of bytes used for representing itself, the
    /// chunk type code, or the CRC. For that reason, zero is a valid value for
    /// this field.
    length: u32,

    /// A 4-byte chunk type code represented through the ChunkType structure.
    chunk_type: ChunkType,

    /// The data bytes appropriate to the chunk type, if any.
    chunk_data: Vec<u8>,

    /// A 4-byte CRC (Cyclic Redundancy Check) calculated on the preceding bytes
    /// in the chunk, including the chunk type code and chunk data fields, but
    /// *not* including the length field. More information about the [CRC
    /// algorithm](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#CRC-algorithm).
    crc: u32,
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        //
    }

    fn length(&self) -> u32 {
        //
    }

    fn chunk_type(&self) -> &ChunkType {
        //
    }

    fn data(&self) -> &[u8] {
        //
    }

    fn crc(&self) -> u32 {
        //
    }

    fn data_as_string(&self) -> crate::Result<String> {
        //
    }

    fn as_bytes(&self) -> Vec<u8> {
        //
    }
}

impl TryFrom<&u8> for Chunk {
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        //
    }
}

#[derive(Debug)]
pub enum ChunkDecodingError {
    //
}

impl Error for ChunkDecodingError {}

impl fmt::Display for ChunkDecodingError {
    //
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

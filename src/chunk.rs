use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

use crate::chunk_type::ChunkType;

/// The PNG specification indicates that chunks cannot exceed 2^31 bytes.
const MAXIMUM_LENGTH: u32 = 1 << 31;

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
    /// Create a chunk from a ChunkType and chunk data.
    fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Chunk {
        Chunk {
            length: chunk_data.len() as u32,
            chunk_type,
            chunk_data,
            crc: crc::Crc::<u32>::new(&crc::CRC_32_CKSUM)
                .checksum(&[&chunk_type.bytes(), chunk_data.as_slice()].concat()),
        }
    }

    /// The number of bytes in chunk's data field. This is *not* the total
    /// number of bytes in the Chunk; it is the number of bytes in
    /// `chunk.data()`. To get the total number of bytes in the chunk, call
    /// `chunk.as_bytes().len()`.
    fn length(&self) -> u32 {
        self.length
    }

    /// The chunk type.
    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// The chunk data.
    fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    /// The pre-calculated CRC (cyclic redundancy check) of the chunk.
    fn crc(&self) -> u32 {
        self.crc
    }

    /// Represent the embedded data within the chunk as a UTF-8 string. If an
    /// error occurs, wrap the error response in a Box object.
    fn data_as_string(&self) -> crate::Result<String> {
        Ok(String::from_utf8(self.chunk_data.clone()).map_err(Box::new)?)
    }

    /// Every byte within chunk.
    fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type().bytes().iter())
            .chain(self.data().iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect::<Vec<u8>>()
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        //
    }
}

/// ChunkDecodingError is used while decoding a PNG chunk and an unexpected
/// scenario arises.
#[derive(Debug)]
pub enum ChunkDecodingError {
    /// The provided CRC did not match the calculated CRC.
    BadCrc(u32, u32),
    /// The provided chunk data length did not match the calculated length.
    BadLength(usize, usize),
    /// The provided chunk data is too large to fit into a single chunk.
    LongLength(usize),
}

impl Error for ChunkDecodingError {}

impl fmt::Display for ChunkDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadCrc(provided_crc, true_crc) => write!(
                f,
                "Bad chunk: Invalid CRC (received {provided_crc}, expected {true_crc})"
            ),
            Self::BadLength(provided_length, true_length) => write!(
                f,
                "Bad chunk: Invalid length (received {provided_length}, expected {true_length})"
            ),
            Self::LongLength(length) => write!(f, "Bad chunk: Length too long ({length} >= 2^31)"),
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

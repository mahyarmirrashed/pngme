use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// A 4-byte PNG chunk type code.
///
/// The [PNG specification](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
/// explains the concept in further detail.
///
/// Type codes are restricted to uppercase and lowercase ASCII characters. All
/// other codes are considered invalid.
#[derive(Eq, PartialEq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    /// Array slice reference to bytes underlying ChunkType.
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_valid(&self) -> bool {
        //
    }

    /// Section 3.2 of the PNG specification explains that type codes are
    /// restricted to consisting only of uppercase and lowercase ASCII
    /// characters. However, encoders/decoders should treat these codes as fixed
    /// binary values rather than character strings.
    ///
    /// For convenience, the conversions between characters and binary values of
    /// the allowable range of type code values are listed below:
    /// - 'A' => 65
    /// - 'Z' => 90
    /// - 'a' => 97
    /// - 'z' => 122
    const fn is_valid_byte(byte: u8) -> bool {
        (65 <= byte && byte <= 90) || (97 <= byte && byte <= 122)
    }

    fn is_critical(&self) -> bool {
        //
    }

    fn is_public(&self) -> bool {
        //
    }

    fn is_reserved_bit_valid(&self) -> bool {
        //
    }

    fn is_safe_to_copy(&self) -> bool {
        //
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    //
}

impl FromStr for ChunkType {
    //
}

impl fmt::Display for ChunkType {
    //
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

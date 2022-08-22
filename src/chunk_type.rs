use std::convert::TryFrom;
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
#[derive(Debug, Eq, PartialEq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    /// Array slice reference to bytes underlying ChunkType.
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// Validity only depends on whether reserved bit is valid.
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    /// Section 3.2 of the PNG specification explains that type codes are
    /// restricted to consisting only of uppercase and lowercase ASCII
    /// characters. However, encoders/decoders should treat these codes as fixed
    /// binary values rather than character strings.
    ///
    /// Fortunately, the Rust programming languages already makes these
    /// functions available through u8::is_ascii_lowercase and
    /// u8::is_ascii_uppercase.
    const fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_uppercase() || byte.is_ascii_lowercase()
    }

    /// Chunks that are necessary for the successful display of the file's
    /// contents are called "critical" chunks. Critical chunks are indicated by
    /// bit 5 of the first byte being high.
    fn is_critical(&self) -> bool {
        Self::is_indicator_zero(self.bytes[0], 5)
    }

    /// Chunks that are public are those that are part of the PNG specification
    /// or are registered in the list of PNG special-purpose public chunk types.
    /// Non-public/private chunks can be defined and used for their own
    /// purposes.
    fn is_public(&self) -> bool {
        Self::is_indicator_zero(self.bytes[1], 5)
    }

    /// In order for the PNG image to conform to the 2022 version of PNG, this
    /// bit must be zero. This bit is reserved for future expansion of the
    /// specification.
    fn is_reserved_bit_valid(&self) -> bool {
        Self::is_indicator_zero(self.bytes[2], 5)
    }

    /// Safe PNG chunks do not depend on image data. If changes are made to any
    /// critical chunks, including addition, modification, deletion, or
    /// reordering of critical chunks, then unrecognized unsafe chunks should
    /// not be copied to the output PNG file.
    fn is_safe_to_copy(&self) -> bool {
        !Self::is_indicator_zero(self.bytes[3], 5)
    }

    /// Checks whether the nth bit on the right of the binary representation of
    /// the byte is zero.
    const fn is_indicator_zero(byte: u8, n: u8) -> bool {
        (byte & (1 << n)) == 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in bytes.iter() {
            if !Self::is_valid_byte(*byte) {
                return Err(Box::new(ChunkTypeDecodingError::BadByte(*byte)));
            }
        }

        Ok(ChunkType { bytes })
    }
}

/// ChunkTypeDecodingError is used while decoding a PNG chunk and an unexpected
/// scenario arises.
#[derive(Debug)]
pub enum ChunkTypeDecodingError {
    /// During decoding, an invalid byte was encountered. The byte encapsulated
    /// is the first bad byte encountered.
    BadByte(u8),
    /// When attempting to cast from String to a ChunkType, there is a
    /// possibility that the provided string may not be four bytes long.
    BadLength(usize),
}

impl fmt::Display for ChunkTypeDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadByte(byte) => write!(f, "Bad byte: {byte} ({byte:b})"),
            Self::BadLength(length) => write!(f, "Bad length: {length} (expected 4)"),
        }
    }
}

impl Error for ChunkTypeDecodingError {}

impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Box::new(ChunkTypeDecodingError::BadLength(s.len())));
        }

        let mut bytes = [0u8; 4];

        for (index, byte) in s.as_bytes().iter().enumerate() {
            if Self::is_valid_byte(*byte) {
                bytes[index] = *byte;
            } else {
                return Err(Box::new(ChunkTypeDecodingError::BadByte(*byte)));
            }
        }

        Ok(ChunkType { bytes })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match std::str::from_utf8(&self.bytes) {
            Ok(res) => res,
            Err(e) => panic!("Invalid byte sequence: {}", e),
        };

        write!(f, "{}", res);

        Ok(())
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

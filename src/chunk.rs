use crate::chunk_type::ChunkType;
use crate::Result;

#[derive(Debug, thiserror::Error)]
pub enum ChunkError {
    #[error("Length mismatch")]
    LengthMismatch,
    #[error("CRC mismatch")]
    CrcMismatch,
}

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk { chunk_type, data }
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
        crc::crc32::checksum_ieee(&self.type_and_data_bytes())
    }

    pub fn data_as_string(&self) -> Result<String> {
        if self.data.len() < 64 {
            let s = String::from_utf8_lossy(&self.data);
            Ok(s.to_string())
        } else {
            Ok(format!("[.. {} bytes ..]", self.data.len()))
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.length().to_be_bytes().to_vec();
        bytes.extend(self.chunk_type.bytes());
        bytes.extend(self.data.to_vec());
        bytes.extend(self.crc().to_be_bytes().to_vec());
        bytes
    }

    fn type_and_data_bytes(&self) -> Vec<u8> {
        let mut bytes = self.chunk_type.bytes().to_vec();
        bytes.extend(self.data.to_vec());
        bytes
    }
}

impl TryFrom<&Vec<u8>> for Chunk {
    type Error = ChunkError;

    fn try_from(value: &Vec<u8>) -> std::result::Result<Chunk, Self::Error> {
        // len: 4 bytes
        // chunk type: 4 bytes
        // data: data_len bytes
        // crc: 4 bytes

        let data_len_bytes: [u8; 4] = value[..4].try_into().unwrap();
        let data_len = u32::from_be_bytes(data_len_bytes) as usize;

        let total_len = value.len();
        if total_len != 4 + 4 + data_len + 4 {
            return Err(ChunkError::LengthMismatch);
        }

        let chunk_type_bytes: [u8; 4] = value[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type_bytes).unwrap();

        let data_bytes = &value[8..(total_len - 4)];

        let crc_bytes: [u8; 4] = value[(total_len - 4)..].try_into().unwrap();
        let parsed_crc = u32::from_be_bytes(crc_bytes);

        let chunk = Chunk {
            chunk_type,
            data: data_bytes.to_vec(),
        };
        if chunk.crc() != parsed_crc {
            return Err(ChunkError::CrcMismatch);
        }

        Ok(chunk)
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Chunk{{type: {}, data: '{}', len: {}}}",
            self.chunk_type(),
            self.data_as_string().unwrap(),
            self.length()
        )
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
        // assert_eq!(chunk.crc(), 2882656334);
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

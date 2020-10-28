#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CodecId {
    Zstd,
}

pub trait Codec {
    const ID: CodecId;

    fn compress(&mut self, bytes: &[u8]) -> std::io::Result<Vec<u8>>;

    fn decompress(&mut self, bytes: &[u8], max_len: usize) -> std::io::Result<Vec<u8>>;

    fn id(&self) -> CodecId {
        Self::ID
    }
}

#[derive(Debug)]
pub struct ZstdCodec {
    level: i32,
}

impl ZstdCodec {
    pub fn new(level: i32) -> ZstdCodec {
        Self { level }
    }
}

impl Codec for ZstdCodec {
    const ID: CodecId = CodecId::Zstd;

    fn compress(&mut self, bytes: &[u8]) -> std::io::Result<Vec<u8>> {
        Ok(zstd::block::compress(bytes, self.level)?)
    }

    fn decompress(&mut self, bytes: &[u8], len: usize) -> std::io::Result<Vec<u8>> {
        Ok(zstd::block::decompress(bytes, len)?)
    }
}

use crate::{util::Skip, Codec};
use serde::{Deserialize, Serialize};

const MAGIC: &str = "oubliette";
const VERSION: u8 = 0;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Header {
    magic: String,
    version: u8,
    codec: crate::CodecId,
}

#[derive(Debug, Clone)]
pub struct Index {
    header: Header,
    map: fst::Map<Skip>,
}

#[derive(Debug, Clone)]
pub struct Archive {
    index: Index,
    data: Vec<u8>,
}

impl Index {
    pub(crate) fn new(codec: crate::CodecId, map: fst::Map<Vec<u8>>) -> Self {
        let header = Header {
            magic: MAGIC.to_owned(),
            version: VERSION,
            codec,
        };
        let map = map
            .map_data(|mut data| {
                let mut header_data = {
                    let mut serializer = serde_cbor::Serializer::new(Vec::<u8>::new());
                    serializer.self_describe().unwrap();
                    header.serialize(&mut serializer).unwrap();
                    serializer.into_inner()
                };
                let skip = header_data.len();
                header_data.append(&mut data);
                Skip::new(header_data, skip)
            })
            .unwrap();
        Self { header, map }
    }

    pub fn get(&self, path: &str) -> Option<(usize, usize)> {
        let index = self.map.get(path)?;
        Some(((index >> 32) as usize, index as u32 as usize))
    }

    pub fn serialize(self) -> Vec<u8> {
        self.map.into_fst().into_inner().into_inner()
    }

    pub fn deserialize(data: Vec<u8>) -> std::io::Result<Self> {
        let (header, offset) = {
            let mut deserializer = serde_cbor::Deserializer::from_slice(&data);
            let header = Header::deserialize(&mut deserializer)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
            (header, deserializer.byte_offset())
        };
        if header.magic != MAGIC {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "not oubliette",
            ));
        }
        if header.version != VERSION {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "unknown version",
            ));
        }
        let data = Skip::new(data, offset);
        let map = fst::Map::new(data)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        map.as_fst()
            .verify()
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        Ok(Self { header, map })
    }
}

impl Archive {
    pub fn new(index: Index, data: Vec<u8>) -> Self {
        Self { index, data }
    }

    pub fn get(&self, path: &str) -> std::io::Result<Option<Vec<u8>>> {
        let (offset, len) = if let Some(res) = self.index.get(path) {
            res
        } else {
            return Ok(None);
        };
        let data = &self.data[offset..(offset + len)];
        let data = crate::ZstdCodec::new(0).decompress(data, 10 * 1024 * 1024)?;
        Ok(Some(data))
    }

    pub fn serialize(self) -> (Vec<u8>, Vec<u8>) {
        (self.index.serialize(), self.data)
    }
}

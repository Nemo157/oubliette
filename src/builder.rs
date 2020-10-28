use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ArchiveBuilder<C> {
    codec: C,
    archive: BTreeMap<String, Vec<u8>>,
    total: usize,
    max_size: usize,
}

impl<C: crate::Codec> ArchiveBuilder<C> {
    pub fn new(codec: C, max_size: usize) -> Self {
        Self {
            codec,
            archive: BTreeMap::new(),
            total: 0,
            max_size,
        }
    }

    pub fn add_directory(&mut self, dir: &str) -> std::io::Result<()> {
        let mut iter = walkdir::WalkDir::new(dir).into_iter();
        while let Some(entry) = iter.next().transpose()? {
            if !entry.file_type().is_file() {
                continue;
            }
            let data = self.codec.compress(&std::fs::read(entry.path())?)?;
            if data.len() > u32::max_value() as usize {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "item too large",
                ));
            }
            self.total += data.len();
            if self.total > self.max_size {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "total too large",
                ));
            }
            let relative = entry
                .path()
                .strip_prefix(dir)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
            let relative = relative.to_str().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, "non-unicode path")
            })?;
            self.archive.insert(relative.to_owned(), data);
        }
        Ok(())
    }

    pub fn finish(self) -> std::io::Result<crate::Archive> {
        let mut data = Vec::with_capacity(self.total);
        let mut builder = fst::MapBuilder::memory();
        let mut offset = 0;
        for (path, mut datum) in self.archive {
            builder
                .insert(path, (offset << 32) + (datum.len() as u64))
                .unwrap();
            offset += datum.len() as u64;
            data.append(&mut datum);
        }
        Ok(crate::Archive::new(
            crate::Index::new(C::ID, builder.into_map()),
            data,
        ))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Skip {
    data: Vec<u8>,
    skip: usize,
}

impl Skip {
    pub(crate) fn new(data: Vec<u8>, skip: usize) -> Self {
        Self { data, skip }
    }

    pub(crate) fn into_inner(self) -> Vec<u8> {
        self.data
    }
}

impl AsRef<[u8]> for Skip {
    fn as_ref(&self) -> &[u8] {
        &self.data.as_slice()[self.skip..]
    }
}

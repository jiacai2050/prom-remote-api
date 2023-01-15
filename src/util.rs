use crate::types::{Error, Result};

pub(crate) fn decode_snappy(raw: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = snap::raw::Decoder::new();
    decoder.decompress_vec(raw).map_err(Error::SnappyDecode)
}

pub(crate) fn encode_snappy(input: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = snap::raw::Encoder::new();
    encoder.compress_vec(input).map_err(Error::SnappyEncode)
}

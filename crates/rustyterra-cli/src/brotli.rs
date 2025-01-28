use std::io::Cursor;

use brotli::enc::BrotliEncoderParams;

// Quality: 0 - 11
pub fn compress_with_brotli(data: &[u8], quality: i32) -> Result<Vec<u8>, ()> {
  let mut reader = Cursor::new(data);
  let mut compressed_data = Vec::new();

  let mut params = BrotliEncoderParams::default();
  params.quality = quality;

  brotli::BrotliCompress(&mut reader, &mut compressed_data, &params).expect("failed to compress");

  Ok(compressed_data)
}

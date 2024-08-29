use std::fs::File;
use std::io::{Read, Write};
use std::ops::Deref;

use crate::shared::error::WebpError;

pub fn to_file(input: &str, output_path: &str, is_lossless: bool, quality: f32) -> Result<(), Box<dyn std::error::Error>> {
    let compressed_image = to_mem(input, is_lossless, quality)?;

    let mut output_file = File::create(output_path)?;

    output_file.write_all(&compressed_image).map_err(WebpError::WriteError)?;

    Ok(())
}

pub fn to_mem(input: &str, is_lossless: bool, quality: f32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(input)?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let decoder = webp::Decoder::new(&data);

    let decode_webp = decoder.decode().ok_or(WebpError::DecodeError)?;
    let image = decode_webp.to_image();

    let encoder = webp::Encoder::from_image(&image).map_err(|_| WebpError::EncodeError)?;
    let encode = encoder.encode_simple(is_lossless, quality).map_err(|_| WebpError::CompressError)?;

    Ok(encode.deref().to_vec())
}

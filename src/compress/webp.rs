use std::fs::File;
use std::io::{Read, Write};
use std::ops::Deref;

use anyhow::{Ok, Result};

use crate::constant::error::WebpError;

pub fn webp_compress(input_path: &str, output_path: &str) -> Result<()> {
    let mut input_file = File::open(input_path)?;

    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    let mut output_file = File::create(output_path)?;
    let compressed_image = compress_to_mem(input_data)?;

    output_file
        .write_all(&compressed_image)
        .map_err(|e| WebpError::WriteError(e))?;

    Ok(())
}

fn compress_to_mem(in_file: Vec<u8>) -> Result<Vec<u8>> {
    let decoder = webp::Decoder::new(&in_file);

    let input_webp = decoder.decode().ok_or(WebpError::DecodeError)?;
    let input_image = input_webp.to_image();

    let encoder = webp::Encoder::from_image(&input_image).map_err(|_| WebpError::EncodeError)?;
    let encode = encoder
        .encode_simple(true, 75.0)
        .map_err(|_| WebpError::CompressError)?;

    Ok(encode.deref().to_vec())
}

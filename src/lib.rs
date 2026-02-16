use wasm_bindgen::prelude::*;
use std::io::Write;

// Runs once on mount
#[wasm_bindgen(start)]
pub fn main_js() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn compress_brotli(data: &[u8], quality: u32) -> Vec<u8> {
    let mut compressor = brotli::CompressorWriter::new(Vec::new(), 4096, quality, 22);
    
    compressor.write_all(data).expect("Brotli compression failed");
    compressor.into_inner()
}

#[wasm_bindgen]
pub fn decompress_brotli(data: &[u8]) -> Vec<u8> {
    let mut decompressor = brotli::Decompressor::new(data, 4096);
    let mut buffer = Vec::new();
    
    use std::io::Read;
    decompressor.read_to_end(&mut buffer).expect("Brotli decompression failed");
    buffer
}

#[wasm_bindgen]
pub fn compress_deflate(data: &[u8]) -> Vec<u8> {
    use flate2::{write::DeflateEncoder, Compression};
    
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    
    encoder.write_all(data).expect("Deflate compression failed");
    encoder.finish().expect("Deflate finish failed")
}

#[wasm_bindgen]
pub fn decompress_deflate(data: &[u8]) -> Vec<u8> {
    use flate2::read::DeflateDecoder;
    use std::io::Read;

    let mut decoder = DeflateDecoder::new(data);
    let mut buffer = Vec::new();
    
    decoder.read_to_end(&mut buffer).expect("Deflate decompression failed");
    buffer
}
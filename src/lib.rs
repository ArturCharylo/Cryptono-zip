pub mod algorithms;

use algorithms::{huffman, lz77};
use algorithms::lz77::LZ77Token;

pub enum Strategy {
    HuffmanOnly,
    LZ77Only,
    Deflate,
}

pub fn compress(input: &[u8], strategy: Strategy) -> Vec<u8> {
    match strategy {
        Strategy::HuffmanOnly => huffman::compress(input),
        
        Strategy::LZ77Only => {
            let tokens = lz77::compress(input);
            serialize_tokens(&tokens)
        },
        
        Strategy::Deflate => {
            let tokens = lz77::compress(input);
            let serialized = serialize_tokens(&tokens);
            huffman::compress(&serialized)
        }
    }
}

fn serialize_tokens(tokens: &[LZ77Token]) -> Vec<u8> {
    let mut buffer = Vec::new();
    for token in tokens {
        match token {
            LZ77Token::Literal(byte) => {
                buffer.push(0);
                buffer.push(*byte);
            },
            LZ77Token::Reference { distance, length } => {
                buffer.push(1);
                buffer.push((distance >> 8) as u8);
                buffer.push((distance & 0xFF) as u8);
                buffer.push(*length as u8);
            }
        }
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_simple() {
        let input = b"AAABBC";
        let compressed = compress(input, Strategy::HuffmanOnly);
        assert!(!compressed.is_empty());
    }

    #[test]
    fn test_lz77_integration() {
        let input = b"test test test test test test";
        let compressed = compress(input, Strategy::LZ77Only);
        assert!(compressed.len() < input.len());
    }
    
    #[test]
    fn test_deflate_chain() {
        let input = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"; 
        let compressed = compress(input, Strategy::Deflate);
        assert!(compressed.len() < input.len());
    }
}
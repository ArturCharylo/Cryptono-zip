pub mod algorithms;


pub use algorithms::huffman::compress_data;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_simple() {
        let input = "AAABBC";
        let compressed = compress_data(input);
        assert!(!compressed.is_empty());
    }

    #[test]
    fn test_single_char() {
        let input = "AAAA";
        let compressed = compress_data(input);
        assert!(compressed.len() < input.len());
    }
}
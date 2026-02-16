use std::cmp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LZ77Token {
    Literal(u8),
    Reference { distance: u16, length: u16 },
}

pub fn compress(input: &[u8]) -> Vec<LZ77Token> {
    let mut tokens = Vec::new();
    let mut cursor = 0;
    
    let window_size: usize = 4096; // 4KB
    let min_match_len: usize = 3;

    while cursor < input.len() {
        let (distance, length) = find_longest_match(input, cursor, window_size);

        if length >= min_match_len {
            tokens.push(LZ77Token::Reference {
                distance: distance as u16,
                length: length as u16,
            });
            cursor += length;
        } else {
            tokens.push(LZ77Token::Literal(input[cursor]));
            cursor += 1;
        }
    }

    tokens
}

fn find_longest_match(input: &[u8], cursor: usize, window_size: usize) -> (usize, usize) {
    let start_window = if cursor > window_size { cursor - window_size } else { 0 };

    let dictionary = &input[start_window..cursor];
    let lookahead = &input[cursor..];

    if lookahead.is_empty() {
        return (0, 0);
    }

    let mut best_len = 0;
    let mut best_dist = 0;

    let max_match = cmp::min(lookahead.len(), 255);
    
    for i in 0..dictionary.len() {
        let mut len = 0;
        
        while len < max_match && dictionary[i + len] == lookahead[len] {
            len += 1;
        }

        if len > best_len {
            best_len = len;
            best_dist = dictionary.len() - i;
        }
    }

    (best_dist, best_len)
}
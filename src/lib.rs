use std::fmt::Write;

#[derive(Debug, Eq, PartialEq)] 
pub struct Node {
    pub freq: u32,
    pub ch: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    // List node constructor
    pub fn new_leaf(freq: u32, ch: char) -> Node {
        Node {
            freq,
            ch: Some(ch),
            left: None,
            right: None,
        }
    }

    // Inner node constructuor
    pub fn new_internal(freq: u32, left: Node, right: Node) -> Node {
        Node {
            freq,
            ch: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

pub fn compress_data(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut compressed = String::new();

    let mut chars = input.chars().peekable();

    while let Some(curr) = chars.next() {
        let mut count = 1;

        while chars.peek() == Some(&curr) {
            count += 1;
            chars.next();
        }

        let _ = write!(compressed, "{}{}", count, curr);
    }
    compressed
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_rle() {
        assert_eq!(compress_data("AAAAABBBCC"), "5A3B2C");
    }

    #[test]
    fn test_single_chars() {
        assert_eq!(compress_data("ABC"), "1A1B1C");
    }
    
    #[test]
    fn test_empty() {
        assert_eq!(compress_data(""), "");
    }
}
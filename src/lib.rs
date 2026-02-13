use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub freq: u32,
    pub ch: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    pub fn new_leaf(freq: u32, ch: char) -> Node {
        Node {
            freq,
            ch: Some(ch),
            left: None,
            right: None,
        }
    }

    pub fn new_internal(freq: u32, left: Node, right: Node) -> Node {
        Node {
            freq,
            ch: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

pub fn calculate_freq(input: &str) -> HashMap<char, u32> {
    let mut freq_map = HashMap::new();
    for chr in input.chars() {
        *freq_map.entry(chr).or_insert(0) += 1;
    }
    freq_map
}

pub fn build_tree(freq_map: &HashMap<char, u32>) -> Option<Node> {
    let mut heap = BinaryHeap::new();

    for (&chr, &freq) in freq_map {
        heap.push(Node::new_leaf(freq, chr));
    }

    if heap.is_empty() {
        return None;
    }

    while heap.len() > 1 {
        let left_node = heap.pop().unwrap();
        let right_node = heap.pop().unwrap();

        let new_freq = left_node.freq + right_node.freq;
        heap.push(Node::new_internal(new_freq, left_node, right_node));
    }

    heap.pop()
}

fn generate_codes(node: &Node, current_code: String, codes: &mut HashMap<char, String>) {
    if let Some(chr) = node.ch {
        let code_to_insert = if current_code.is_empty() {
            "0".to_string()
        } else {
            current_code
        };
        codes.insert(chr, code_to_insert);
        return;
    }

    if let Some(ref left) = node.left {
        let mut left_code = current_code.clone();
        left_code.push('0');
        generate_codes(left, left_code, codes);
    }

    if let Some(ref right) = node.right {
        let mut right_code = current_code.clone();
        right_code.push('1');
        generate_codes(right, right_code, codes);
    }
}

pub fn compress_data(input: &str) -> String {
    let freq_map = calculate_freq(input);
    let root = build_tree(&freq_map);

    let mut codes = HashMap::new();

    if let Some(tree_root) = root {
        generate_codes(&tree_root, String::new(), &mut codes);
    }

    let mut compressed = String::new();
    for chr in input.chars() {
        if let Some(code) = codes.get(&chr) {
            compressed.push_str(code);
        }
    }

    compressed
}

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
        assert_eq!(compressed, "0000");
    }
}
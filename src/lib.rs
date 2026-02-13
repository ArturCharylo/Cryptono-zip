use std::{collections::{BinaryHeap, HashMap}};

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.freq.cmp(&self.freq)
    }
}

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

pub fn build_tree(freq_map: HashMap<char, u32>) -> Option<Node>{
    let mut heap = BinaryHeap::new();

    for (chr, freq) in freq_map{
        heap.push(Node::new_leaf(freq, chr));
    }

    while heap.len() > 1 {
        let left_node = heap.pop().unwrap();
        let right_node = heap.pop().unwrap();

        heap.push(Node::new_internal(left_node.freq + right_node.freq, left_node, right_node));
    }
    heap.pop()
}

pub fn calculat_freq(input: &str) -> std::collections::HashMap<char, u32> {
    let mut freq_map: HashMap<char, u32> = std::collections::HashMap::new();
    for chr in input.chars() {
        let count = freq_map.entry(chr).or_insert(0);
        *count += 1;
    }
    freq_map
}

fn generate_codes(node: &Node, current_code: String, codes: &mut HashMap<char, String>) {
    if let Some(chr) = node.ch {
        codes.insert(chr, current_code);
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
    let freq_map = calculat_freq(input);
    let root = build_tree(freq_map);

    let mut codes = HashMap::new();
    if let Some(tree_root) = root {
        generate_codes(&tree_root, String::new(), &mut codes);
    }

    let mut compressed = String::new();
    
    for chr in input.chars() {
        compressed.push_str(codes.get(&chr).expect("No Huffman code found"));
    }
    compressed
}

#[cfg(test)]
mod tests {
    use super::*;
}
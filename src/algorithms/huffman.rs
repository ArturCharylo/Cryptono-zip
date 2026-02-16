use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use bit_vec::BitVec;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub freq: u32,
    pub val: Option<u8>,
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
    pub fn new_leaf(freq: u32, val: u8) -> Node {
        Node { freq, val: Some(val), left: None, right: None }
    }

    pub fn new_internal(freq: u32, left: Node, right: Node) -> Node {
        Node { freq, val: None, left: Some(Box::new(left)), right: Some(Box::new(right)) }
    }
}

pub fn calculate_freq(input: &[u8]) -> HashMap<u8, u32> {
    let mut freq_map = HashMap::new();
    for &val in input {
        *freq_map.entry(val).or_insert(0) += 1;
    }
    freq_map
}

pub fn build_tree(freq_map: &HashMap<u8, u32>) -> Option<Node> {
    let mut heap = BinaryHeap::new();
    for (&val, &freq) in freq_map {
        heap.push(Node::new_leaf(freq, val));
    }
    
    if heap.is_empty() { return None; }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        heap.push(Node::new_internal(left.freq + right.freq, left, right));
    }
    heap.pop()
}

fn generate_codes(node: &Node, current_code: BitVec, codes: &mut HashMap<u8, BitVec>) {
    if let Some(val) = node.val {
        codes.insert(val, if current_code.is_empty() { 
            let mut b = BitVec::new(); b.push(false); b 
        } else { current_code });
        return;
    }
    if let Some(ref left) = node.left {
        let mut c = current_code.clone(); c.push(false);
        generate_codes(left, c, codes);
    }
    if let Some(ref right) = node.right {
        let mut c = current_code.clone(); c.push(true);
        generate_codes(right, c, codes);
    }
}

pub fn compress(input: &[u8]) -> Vec<u8> {
    let freq_map = calculate_freq(input);
    let root = build_tree(&freq_map);
    let mut codes = HashMap::new();

    if let Some(tree_root) = root {
        generate_codes(&tree_root, BitVec::new(), &mut codes);
    }

    let mut compressed = BitVec::new();
    for &val in input {
        if let Some(code) = codes.get(&val) {
            for bit in code {
                compressed.push(bit);
            }
        }
    }
    compressed.to_bytes()
}
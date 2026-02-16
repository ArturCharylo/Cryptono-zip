#[derive(Debug)]
struct Token{
    offset: u32,
    length: u32,
    next_char: Option<char>,
}

fn lz77_compress(input: &str, window_size: usize) -> Vec<Token>{
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut cursor = 0;

    while cursor < chars.len(){
        let (offset, length) = find_match(&chars, cursor, window_size);
        let next_char = chars.get(cursor + length as usize).cloned();
        
        tokens.push(Token { offset, length, next_char });
        
        cursor += (length as usize) + 1;
    }
    tokens
}

fn find_match(input: &[char], cursor: usize, window_size: usize) -> (u32, u32){
    let start = if cursor > window_size { cursor - window_size } else { 0 };
    let dictionary = &input[start..cursor];
    let lookahead = &input[cursor..];

    for len in (1..=lookahead.len()).rev(){
        let curr_match = &lookahead[0..len];
        if let Some(i) = dictionary.windows(len).position(|window| window == curr_match) {
            let offset = (dictionary.len() - i) as u32;
            let length = len as u32;
            return (offset, length);
        }
    }
    (0,0)
}
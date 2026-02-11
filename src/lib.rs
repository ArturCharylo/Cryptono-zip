pub fn compress_data(input: &str) -> String {
    // Placeholder for compression logic
    format!("Feature in work, input length: {}", input.len())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = compress_data("test");
        assert!(result.contains("4"));
    }
}
const BASE: usize = 256;
const MOD: usize = 997; // A prime number

pub fn rabin_karp(pattern: &[u8], text: &[u8]) -> Vec<usize> {

    let mut result: Vec<usize> = Vec::new();

    let pattern_len = pattern.len();
    let text_len = text.len();

    if pattern_len == 0 || pattern_len > text_len {
        return result;
    }


    // Calculate hash of the pattern
    let pattern_hash: usize = pattern.iter().fold(0, |acc, &x| (acc * BASE + x as usize) % MOD);

    // Iterate over substrings of the text
    for i in 0..=text_len - pattern_len {
        // Calculate hash of the current substring
        let text_hash: usize = text[i..i + pattern_len].iter().fold(0, |acc, &x| (acc * BASE + x as usize) % MOD);

        // Check for match
        if pattern_hash == text_hash && pattern == &text[i..i + pattern_len] {
            result.push(i);
        }
    }

    result
}
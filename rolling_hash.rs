const MOD: usize = 1_000_000_007;
const BASE: usize = 256;

pub fn rolling_hash(pattern: &[u8], text: &[u8]) -> Vec<usize> {

    //Length exeption
    if pattern.len() > text.len() || pattern.len() == 0 {
        return Vec::new();
    }



    let mut result: Vec<usize> = Vec::new();

    // Calculate hash of pattern
    let pattern_hash: usize = pattern.iter().fold(0, |acc: usize, &x| (acc * BASE + x as usize) & (MOD - 1));

    // Calculate initial hash of text
    let mut text_hash: usize = text.iter().take(pattern.len()).fold(0, |acc: usize, &x| (acc * BASE + x as usize) & (MOD - 1));

    // Check for match in initial substring
    if pattern_hash == text_hash && pattern == &text[0..pattern.len()] {
        result.push(0);
    }

    // Precompute powers of the base
    let mut base_pow: usize = 1;
    for _ in 0..pattern.len() {
        base_pow = (base_pow * BASE) & (MOD - 1);
    }

    // Iterate over remaining substrings of text
    for i in 1..text.len() - pattern.len() + 1 {
        // Update hash using sliding window technique
        text_hash = (((text_hash * BASE) & (MOD - 1)) + text[i + pattern.len() - 1] as usize - ((text[i - 1] as usize * base_pow) & (MOD - 1))) & (MOD - 1);

        // Check for match
        if pattern_hash == text_hash && pattern == &text[i..i + pattern.len()] {
            result.push(i);
        }
    }

    result
}

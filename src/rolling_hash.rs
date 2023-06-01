

pub fn rolling_hash(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let n: usize = text.len();
    let m: usize = pattern.len();
    let mut results: Vec<usize> = Vec::new();

    if m == 0 || m > n {
        return results;
    }

    // Calculate hash of the pattern
    let pattern_hash: u64 = pattern.iter().fold(0, |acc, &x| (acc << 8) + u64::from(x));

    // Calculate initial hash of the first substring in the text
    let mut text_hash: u64 = text.iter().take(m).fold(0, |acc, &x| (acc << 8) + u64::from(x));

    // Check if the first substring matches the pattern
    if pattern_hash == text_hash && pattern == &text[0..m] {
        results.push(0);
    }

    // Calculate the rolling hash for the remaining substrings
    for i in 1..=n - m {
        // Remove the leftmost character from the rolling hash
        text_hash -= u64::from(text[i - 1]) << (8 * (m - 1));

        // Shift the rolling hash left by one position and add the new character
        text_hash = (text_hash << 8) + u64::from(text[i + m - 1]);

        // Check if the rolling hash matches the pattern
        if pattern_hash == text_hash && pattern == &text[i..i + m] {
            results.push(i);
        }
    }

    results
}

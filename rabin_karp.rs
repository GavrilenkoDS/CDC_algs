const PRIME: u64 = 101;

pub fn rabin_karp(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let n: usize = text.len();
    let m: usize = pattern.len();

    let mut results: Vec<usize> = Vec::new();

    if m == 0 || m > n {
        return results;
    }


    let mut pattern_hash: u64 = 0;
    let mut text_hash: u64 = 0;

    // calculate hash of pattern and first m characters of text
    for i in 0..m {
        pattern_hash = pattern_hash.wrapping_mul(PRIME).wrapping_add(pattern[i] as u64);
        text_hash = text_hash.wrapping_mul(PRIME).wrapping_add(text[i] as u64);
    }

    // iterate over remaining substrings of text
    for i in 0..n - m + 1 {
        // check if hash values match
        if pattern_hash == text_hash {
            // check if the substrings match
            if pattern == &text[i..i + m] {
                results.push(i);
            }
        }

        // update hash of text for next substring
        if i < n - m {
            text_hash = text_hash.wrapping_sub((text[i] as u64).wrapping_mul(PRIME.pow(m as u32 - 1)));
            text_hash = text_hash.wrapping_mul(PRIME);
            text_hash = text_hash.wrapping_add(text[i + m] as u64);
        }
    }

    results
}
